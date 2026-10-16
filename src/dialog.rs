//! Generic modal dialog primitives for Pomelo OS apps.
//!
//! Provides standard centered modal dialogs, dimmed backdrops, and dialog buttons
//! adhering to Pomelo OS design specifications:
//! - 360 px width card centered on 480 px panels
//! - Semi-transparent dimmed backdrop blocking background touches
//! - Standard button heights (44 px), borders, and press-state shading

use iced::widget::{button, container, opaque, text, Column, Space};
use iced::{Border, Color, Element, Length, Shadow};

use crate::preferences::ThemeMode;

/// The standard dialog box width in pixels (centered on 480 px panels).
pub const DIALOG_WIDTH: f32 = 360.0;

/// The standard dialog inner padding in pixels.
pub const DIALOG_PADDING: f32 = 20.0;

/// The standard dialog card corner radius in pixels.
pub const DIALOG_RADIUS: f32 = 16.0;

/// The standard card border outline width in pixels.
pub const DIALOG_BORDER_WIDTH: f32 = 1.0;

/// The standard dialog action button height in pixels.
pub const BUTTON_HEIGHT: f32 = 44.0;

/// The standard dialog action button corner radius in pixels.
pub const BUTTON_RADIUS: f32 = 12.0;

/// The standard dimmed backdrop color (60% opaque black).
pub const BACKDROP_COLOR: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.6);

/// The standard primary accent color for confirm buttons.
pub const ACCENT: Color = Color::from_rgb(10.0 / 255.0, 132.0 / 255.0, 255.0 / 255.0);

/// The standard danger color for high-risk confirm buttons (e.g. format, restore).
pub const DANGER: Color = Color::from_rgb(255.0 / 255.0, 59.0 / 255.0, 48.0 / 255.0);

/// Wraps `content` in a dimmed, full-screen opaque backdrop centered on the screen.
///
/// Touch and mouse events outside the centered content are absorbed by the backdrop,
/// preventing underlying components from receiving accidental touches.
pub fn modal_dialog<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    opaque(
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(BACKDROP_COLOR.into()),
                ..container::Style::default()
            }),
    )
    .into()
}

/// Standard 360 px dialog card container with card background, border, and padding.
pub fn dialog_card<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: ThemeMode,
) -> container::Container<'a, Message> {
    container(content)
        .width(Length::Fixed(DIALOG_WIDTH))
        .padding(DIALOG_PADDING)
        .style(move |_theme| container::Style {
            background: Some(theme.card().into()),
            border: Border {
                color: theme.border(),
                width: DIALOG_BORDER_WIDTH,
                radius: DIALOG_RADIUS.into(),
            },
            ..container::Style::default()
        })
}

/// Standard 360 px modal dialog box with card background, border, title, body, and action buttons.
pub fn dialog_box<'a, Message: 'a>(
    title: impl Into<Element<'a, Message>>,
    body: impl Into<Element<'a, Message>>,
    actions: impl Into<Element<'a, Message>>,
    theme: ThemeMode,
) -> Element<'a, Message> {
    let dialog_content = Column::with_children(vec![
        title.into(),
        Space::new().height(Length::Fixed(12.0)).into(),
        body.into(),
        Space::new().height(Length::Fixed(24.0)).into(),
        actions.into(),
    ])
    .width(Length::Fill);

    modal_dialog(dialog_card(dialog_content, theme))
}

/// Visual style for dialog action buttons.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DialogButtonStyle {
    /// Secondary button with track background (used for Cancel).
    Cancel,
    /// Primary accent button (e.g. OK, Confirm).
    Primary,
    /// Destructive action button with red fill (e.g. Format, Clear, Restore).
    Danger,
    /// Custom background color.
    Custom(Color),
}

/// Builds a styled modal dialog action button with active pressed-state shading.
pub fn dialog_button<'a, Message: 'a + Clone>(
    label: impl Into<String>,
    on_press: Message,
    button_style: DialogButtonStyle,
    theme: ThemeMode,
) -> Element<'a, Message> {
    let label_str = label.into();
    let is_cancel = matches!(button_style, DialogButtonStyle::Cancel);

    let text_color = if is_cancel {
        theme.text_primary()
    } else {
        Color::WHITE
    };

    button(
        container(
            text(label_str)
                .size(17.0)
                .color(text_color),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill),
    )
    .width(Length::FillPortion(1))
    .height(Length::Fixed(BUTTON_HEIGHT))
    .padding(0)
    .style(move |_theme, status| {
        let (bg, fg) = match button_style {
            DialogButtonStyle::Cancel => {
                let fill = if matches!(status, button::Status::Pressed | button::Status::Hovered) {
                    match theme {
                        ThemeMode::Dark => Color::from_rgb8(74, 74, 80),
                        ThemeMode::Light => Color::from_rgb8(209, 209, 214),
                    }
                } else {
                    match theme {
                        ThemeMode::Dark => Color::from_rgb8(60, 60, 65),
                        ThemeMode::Light => Color::from_rgb8(229, 229, 234),
                    }
                };
                (Some(fill.into()), theme.text_primary())
            }
            DialogButtonStyle::Primary => {
                let bg_color = ACCENT;
                let fill = if matches!(status, button::Status::Pressed | button::Status::Hovered) {
                    Color::from_rgba(bg_color.r * 0.85, bg_color.g * 0.85, bg_color.b * 0.85, bg_color.a)
                } else {
                    bg_color
                };
                (Some(fill.into()), Color::WHITE)
            }
            DialogButtonStyle::Danger => {
                let bg_color = DANGER;
                let fill = if matches!(status, button::Status::Pressed | button::Status::Hovered) {
                    Color::from_rgba(bg_color.r * 0.85, bg_color.g * 0.85, bg_color.b * 0.85, bg_color.a)
                } else {
                    bg_color
                };
                (Some(fill.into()), Color::WHITE)
            }
            DialogButtonStyle::Custom(bg_color) => {
                let fill = if matches!(status, button::Status::Pressed | button::Status::Hovered) {
                    Color::from_rgba(bg_color.r * 0.85, bg_color.g * 0.85, bg_color.b * 0.85, bg_color.a)
                } else {
                    bg_color
                };
                (Some(fill.into()), Color::WHITE)
            }
        };
        button::Style {
            background: bg,
            text_color: fg,
            border: Border {
                radius: BUTTON_RADIUS.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
            snap: false,
        }
    })
    .on_press(on_press)
    .into()
}

/// Convenience builder for a modal cancel button.
pub fn dialog_cancel_button<'a, Message: 'a + Clone>(
    label: impl Into<String>,
    on_press: Message,
    theme: ThemeMode,
) -> Element<'a, Message> {
    dialog_button(label, on_press, DialogButtonStyle::Cancel, theme)
}

/// Convenience builder for a primary modal confirmation button.
pub fn dialog_confirm_button<'a, Message: 'a + Clone>(
    label: impl Into<String>,
    on_press: Message,
    theme: ThemeMode,
) -> Element<'a, Message> {
    dialog_button(label, on_press, DialogButtonStyle::Primary, theme)
}

/// Convenience builder for a destructive/danger modal confirmation button.
pub fn dialog_danger_button<'a, Message: 'a + Clone>(
    label: impl Into<String>,
    on_press: Message,
    theme: ThemeMode,
) -> Element<'a, Message> {
    dialog_button(label, on_press, DialogButtonStyle::Danger, theme)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::Row;

    #[test]
    fn dialog_constants_sane() {
        assert_eq!(DIALOG_WIDTH, 360.0);
        assert_eq!(BUTTON_HEIGHT, 44.0);
        assert_eq!(DIALOG_PADDING, 20.0);
    }

    #[test]
    fn modal_dialog_builds_without_panic() {
        let card = text("Hello dialog");
        let _modal: Element<'_, ()> = modal_dialog(card);
    }

    #[test]
    fn dialog_box_builds_without_panic() {
        let title = text("Title");
        let body = text("Body description");
        let actions = Row::with_children(vec![
            dialog_cancel_button("Cancel", 0usize, ThemeMode::Dark),
            dialog_confirm_button("OK", 1usize, ThemeMode::Dark),
        ]);
        let _dlg: Element<'_, usize> = dialog_box(title, body, actions, ThemeMode::Dark);
    }
}
