//! Floating bottom-centered Toast widget for Pomelo OS apps.
//!
//! Provides a non-modal, self-dismissing text banner adhering to standard mobile OS specifications:
//! - Position: Bottom centered
//! - Content: Text only, no icons
//! - Appearance: Fixed pill shape (no customizable colors), dark translucent glass style
//! - Duration: Fixed 2.0s duration (matching Android `Toast.LENGTH_SHORT` and iOS standards)
//! - Non-blocking: Outside touches pass through to underlying widgets seamlessly.

use core::time::Duration;
use iced::widget::{container, text};
use iced::{Border, Color, Element, Length, Padding, Shadow, Task, Vector};

use crate::opaque::opaque;
use crate::preferences::{FontSizeTier, ThemeMode};

/// Fixed duration the toast stays visible on screen (2.0 seconds).
pub const TOAST_DURATION: Duration = Duration::from_millis(2000);

/// Distance from the bottom of the screen to the toast pill (40px).
pub const TOAST_BOTTOM_MARGIN: f32 = 40.0;

/// Corner radius of the pill container (20px).
pub const TOAST_RADIUS: f32 = 20.0;

/// Default text size inside the toast (20px, Standard Small tier).
pub const TOAST_FONT_SIZE: f32 = FontSizeTier::Small.base_size();

/// Maximum width of the toast pill (380px on a 480px display).
pub const TOAST_MAX_WIDTH: f32 = 380.0;

/// Internal padding of the toast pill.
pub const TOAST_PADDING: Padding = Padding {
    top: 10.0,
    bottom: 10.0,
    left: 20.0,
    right: 20.0,
};

/// Encapsulated state for an application's Toast.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Toast {
    message: Option<String>,
    id: u32,
}

impl Toast {
    /// Creates a new hidden toast state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the text message and increments the generation id without scheduling a task.
    ///
    /// Returns the new generation `id`.
    pub fn set_text(&mut self, text: impl Into<String>) -> u32 {
        self.id = self.id.wrapping_add(1);
        self.message = Some(text.into());
        self.id
    }

    /// Shows a text message in the toast for the standard duration (2.0s),
    /// returning a [`Task`] that emits the dismissal message upon expiry.
    pub fn show<Message: Send + 'static>(
        &mut self,
        text: impl Into<String>,
        on_dismiss: impl FnOnce(u32) -> Message + Send + 'static,
    ) -> Task<Message> {
        let current_id = self.set_text(text);

        Task::perform(
            async move {
                let (tx, rx) = iced::futures::channel::oneshot::channel();
                std::thread::Builder::new()
                    .name("toast_timer".into())
                    .spawn(move || {
                        std::thread::sleep(TOAST_DURATION);
                        let _ = tx.send(());
                    })
                    .ok();
                let _ = rx.await;
                current_id
            },
            on_dismiss,
        )
    }

    /// Dismisses the toast if `id` matches the current active toast.
    ///
    /// Returns `true` if dismissed, or `false` if the toast was already superseded or closed.
    pub fn dismiss(&mut self, id: u32) -> bool {
        if self.id == id {
            self.message = None;
            true
        } else {
            false
        }
    }

    /// Immediately clears and hides the toast.
    pub fn clear(&mut self) {
        self.message = None;
    }

    /// Returns the active message text, if visible.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns whether the toast is currently visible.
    pub fn is_visible(&self) -> bool {
        self.message.is_some()
    }

    /// Returns the current generation id.
    pub fn id(&self) -> u32 {
        self.id
    }
}

/// Builds the floating bottom-centered Toast widget element.
///
/// Returns `None` if `toast` is not currently visible.
pub fn toast_view<'a, Message: 'a>(
    toast: &Toast,
    theme: ThemeMode,
) -> Option<Element<'a, Message>> {
    let message = toast.message()?;

    let (bg_color, border_color, shadow_color) = match theme {
        ThemeMode::Dark => (
            Color::from_rgba(0.18, 0.18, 0.20, 0.95),
            Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            Color::from_rgba(0.0, 0.0, 0.0, 0.5),
        ),
        ThemeMode::Light => (
            Color::from_rgba(0.14, 0.14, 0.16, 0.92),
            Color::from_rgba(1.0, 1.0, 1.0, 0.12),
            Color::from_rgba(0.0, 0.0, 0.0, 0.35),
        ),
    };

    let pill = opaque(
        container(
            text(message.to_string())
                .size(TOAST_FONT_SIZE)
                .color(Color::WHITE)
                .align_x(iced::alignment::Horizontal::Center)
                .wrapping(text::Wrapping::Word),
        )
        .padding(TOAST_PADDING)
        .max_width(TOAST_MAX_WIDTH)
        .style(move |_| container::Style {
            background: Some(bg_color.into()),
            border: Border {
                color: border_color,
                width: 1.0,
                radius: TOAST_RADIUS.into(),
            },
            shadow: Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
                color: shadow_color,
            },
            ..container::Style::default()
        }),
    );

    let overlay = container(pill)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(iced::alignment::Vertical::Bottom)
        .center_x(Length::Fill)
        .padding(Padding {
            bottom: TOAST_BOTTOM_MARGIN,
            left: 20.0,
            right: 20.0,
            top: 0.0,
        });

    Some(overlay.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toast_initial_state() {
        let toast = Toast::new();
        assert!(!toast.is_visible());
        assert_eq!(toast.message(), None);
        assert_eq!(toast.id(), 0);
    }

    #[test]
    fn toast_set_text_and_visibility() {
        let mut toast = Toast::new();
        let id1 = toast.set_text("Hello Pomelo");
        assert_eq!(id1, 1);
        assert!(toast.is_visible());
        assert_eq!(toast.message(), Some("Hello Pomelo"));

        let id2 = toast.set_text("Second Toast");
        assert_eq!(id2, 2);
        assert_eq!(toast.message(), Some("Second Toast"));
    }

    #[test]
    fn toast_dismiss_matching_id() {
        let mut toast = Toast::new();
        let id1 = toast.set_text("Test");
        let id2 = toast.set_text("Newer Test");

        // Stale dismiss from id1 should not close the toast
        assert!(!toast.dismiss(id1));
        assert!(toast.is_visible());
        assert_eq!(toast.message(), Some("Newer Test"));

        // Matching dismiss from id2 should close the toast
        assert!(toast.dismiss(id2));
        assert!(!toast.is_visible());
        assert_eq!(toast.message(), None);
    }

    #[test]
    fn toast_clear() {
        let mut toast = Toast::new();
        toast.set_text("Disappearing");
        assert!(toast.is_visible());
        toast.clear();
        assert!(!toast.is_visible());
        assert_eq!(toast.message(), None);
    }

    #[test]
    fn toast_view_visibility() {
        let mut toast = Toast::new();
        assert!(toast_view::<()>(&toast, ThemeMode::Dark).is_none());

        toast.set_text("Visible");
        assert!(toast_view::<()>(&toast, ThemeMode::Dark).is_some());
        assert!(toast_view::<()>(&toast, ThemeMode::Light).is_some());
    }
}
