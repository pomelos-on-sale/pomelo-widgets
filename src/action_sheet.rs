//! Generic modal Action Sheet primitives for Pomelo OS apps.
//!
//! Provides standard bottom-anchored action sheets adhering to Pomelo OS design specifications:
//! - Full-screen dimmed backdrop (60% opacity) blocking background touches
//! - Reaches all screen edges (0 margin)
//! - Unified fixed proportional height (100% of 480px = 480px) ensuring predictable layout across all sheets
//! - Standard top navigation row with "✕" (Cancel / Close) and "✓" (Confirm / OK) capsule buttons
//! - Unified body hosting centered title, icon, and message (without redundant internal buttons)
//! - Strict typing ensuring modals in applications are structured as ActionSheets rather than arbitrary dialogs.

use iced::border::Radius;
use iced::widget::{button, container, opaque, text, Column, Row, Space};
use iced::{Border, Color, Element, Length, Padding, Shadow};
use pomelo_material_symbols::Icon;

use crate::preferences::{FontTier, ThemeMode};

/// The default proportion of the screen height occupied by an [`ActionSheet`] (100%).
pub const ACTION_SHEET_HEIGHT_RATIO: f32 = 1.0;
/// The unified fixed height in pixels of an [`ActionSheet`] on a 480px screen (480px).
pub const ACTION_SHEET_HEIGHT: f32 = 480.0;
/// Width of the top capsule action buttons (matching Pomelo OS capsule navigation button: 72px).
pub const CAPSULE_BUTTON_WIDTH: f32 = 72.0;
/// Height of the top capsule action buttons (matching Pomelo OS capsule navigation button: 48px).
pub const CAPSULE_BUTTON_HEIGHT: f32 = 48.0;
/// Glyph font size for top capsule action buttons.
pub const CAPSULE_BUTTON_GLYPH: f32 = FontTier::Standard.base_size(); // 24.0 px
/// Title font size for top capsule navigation bar in an [`ActionSheet`] (standard Standard tier: 24px).
pub const TOP_BAR_TITLE_FONT: f32 = FontTier::Standard.base_size();
/// Top bar icon size in an [`ActionSheet`] (standard Standard tier: 24px).
pub const TOP_BAR_ICON: f32 = FontTier::Standard.base_size();
/// Title font size for the centered body title in an [`ActionSheet`] (standard Standard tier: 24px).
pub const BODY_TITLE_FONT: f32 = FontTier::Standard.base_size();
/// Icon size for the centered body title in an [`ActionSheet`] (standard Standard tier: 24px).
pub const BODY_TITLE_ICON: f32 = FontTier::Standard.base_size();
/// Message font size for the centered body text in an [`ActionSheet`] (standard Standard tier: 24px).
pub const BODY_MESSAGE_FONT: f32 = FontTier::Standard.base_size();

/// Visual style of an action button within an [`ActionSheet`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActionStyle {
    /// Standard primary accent action (iOS system blue).
    #[default]
    Primary,
    /// Destructive high-risk action (iOS system red).
    Destructive,
    /// Muted or secondary action (primary label text color).
    Secondary,
}

/// An individual action button in an [`ActionSheet`].
#[derive(Debug, Clone)]
pub struct ActionItem<Message> {
    pub label: String,
    pub message: Message,
    pub style: ActionStyle,
    pub icon: Option<Icon>,
    pub enabled: bool,
}

impl<Message> ActionItem<Message> {
    /// Creates a new primary action button.
    pub fn new(label: impl Into<String>, message: Message) -> Self {
        Self {
            label: label.into(),
            message,
            style: ActionStyle::Primary,
            icon: None,
            enabled: true,
        }
    }

    /// Creates a destructive (red) action button.
    pub fn destructive(label: impl Into<String>, message: Message) -> Self {
        Self {
            label: label.into(),
            message,
            style: ActionStyle::Destructive,
            icon: None,
            enabled: true,
        }
    }

    /// Creates a secondary action button.
    pub fn secondary(label: impl Into<String>, message: Message) -> Self {
        Self {
            label: label.into(),
            message,
            style: ActionStyle::Secondary,
            icon: None,
            enabled: true,
        }
    }

    /// Attaches an icon to this action button.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets whether this action button is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// A strongly-typed, bottom-aligned modal action sheet.
pub struct ActionSheet<'a, Message> {
    pub title: Option<String>,
    pub title_icon: Option<Icon>,
    pub message: Option<String>,
    pub custom_body: Option<Element<'a, Message>>,
    pub body_padding: Option<Padding>,
    pub actions: Vec<ActionItem<Message>>,
    pub cancel: Option<ActionItem<Message>>,
    pub theme: ThemeMode,
    pub font_tier: FontTier,
}

impl<'a, Message> std::fmt::Debug for ActionSheet<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ActionSheet")
            .field("title", &self.title)
            .field("title_icon", &self.title_icon)
            .field("message", &self.message)
            .field("has_custom_body", &self.custom_body.is_some())
            .field("body_padding", &self.body_padding)
            .field("actions_count", &self.actions.len())
            .field("has_cancel", &self.cancel.is_some())
            .field("theme", &self.theme)
            .field("font_tier", &self.font_tier)
            .finish()
    }
}

impl<'a, Message> ActionSheet<'a, Message> {
    /// Creates a new empty [`ActionSheet`] with the given theme and standard font tier ([`FontTier::Standard`]).
    pub fn new(theme: ThemeMode) -> Self {
        Self {
            title: None,
            title_icon: None,
            message: None,
            custom_body: None,
            body_padding: None,
            actions: Vec::new(),
            cancel: None,
            theme,
            font_tier: FontTier::Standard,
        }
    }

    /// Sets the font tier for this [`ActionSheet`]. Defaults to [`FontTier::Standard`].
    pub fn font_tier(mut self, tier: FontTier) -> Self {
        self.font_tier = tier;
        self
    }

    /// Sets the title text.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        let s = title.into();
        if !s.trim().is_empty() {
            self.title = Some(s);
        }
        self
    }

    /// Sets the icon displayed beside the title.
    pub fn title_icon(mut self, icon: Icon) -> Self {
        self.title_icon = Some(icon);
        self
    }

    /// Sets the explanatory message / prompt text.
    pub fn message(mut self, message: impl Into<String>) -> Self {
        let s = message.into();
        if !s.trim().is_empty() {
            self.message = Some(s);
        }
        self
    }

    /// Sets a custom body [`Element`] for this [`ActionSheet`].
    ///
    /// When set, the custom body replaces the default icon + title + message column,
    /// allowing arbitrary widgets (forms, keyboards, pickers, lists) to be hosted
    /// inside the standard 100% modal sheet.
    pub fn custom_body(mut self, body: impl Into<Element<'a, Message>>) -> Self {
        self.custom_body = Some(body.into());
        self
    }

    /// Alias for [`custom_body`](Self::custom_body).
    pub fn body(self, body: impl Into<Element<'a, Message>>) -> Self {
        self.custom_body(body)
    }

    /// Sets custom padding for the body container.
    pub fn body_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.body_padding = Some(padding.into());
        self
    }

    /// Adds an action item with the given style.
    pub fn action(
        mut self,
        label: impl Into<String>,
        message: Message,
        style: ActionStyle,
    ) -> Self {
        self.actions.push(ActionItem {
            label: label.into(),
            message,
            style,
            icon: None,
            enabled: true,
        });
        self
    }

    /// Adds an [`ActionItem`].
    pub fn action_item(mut self, item: ActionItem<Message>) -> Self {
        self.actions.push(item);
        self
    }

    /// Adds a destructive (red) action item.
    pub fn destructive(mut self, label: impl Into<String>, message: Message) -> Self {
        self.actions.push(ActionItem::destructive(label, message));
        self
    }

    /// Sets the cancel / dismiss action in the distinct lower card.
    pub fn cancel(mut self, label: impl Into<String>, message: Message) -> Self {
        self.cancel = Some(ActionItem::secondary(label, message));
        self
    }

    /// Sets a customized cancel action item.
    pub fn cancel_item(mut self, item: ActionItem<Message>) -> Self {
        self.cancel = Some(item);
        self
    }

    /// Convenience constructor for a notification / alert action sheet with a single dismiss button.
    pub fn notice(
        title: impl Into<String>,
        message: impl Into<String>,
        ok_label: impl Into<String>,
        ok_msg: Message,
        theme: ThemeMode,
    ) -> Self {
        Self::new(theme)
            .title(title)
            .message(message)
            .cancel(ok_label, ok_msg)
    }

    /// Convenience constructor for a standard confirmation action sheet (Confirm + Cancel).
    pub fn confirm(
        prompt: impl Into<String>,
        confirm_label: impl Into<String>,
        confirm_msg: Message,
        cancel_label: impl Into<String>,
        cancel_msg: Message,
        theme: ThemeMode,
    ) -> Self {
        Self::new(theme)
            .message(prompt)
            .action(confirm_label, confirm_msg, ActionStyle::Primary)
            .cancel(cancel_label, cancel_msg)
    }

    /// Convenience constructor for a high-risk destructive action sheet (Destructive danger + Cancel).
    pub fn destructive_confirm(
        prompt: impl Into<String>,
        danger_label: impl Into<String>,
        danger_msg: Message,
        cancel_label: impl Into<String>,
        cancel_msg: Message,
        theme: ThemeMode,
    ) -> Self {
        Self::new(theme)
            .message(prompt)
            .destructive(danger_label, danger_msg)
            .cancel(cancel_label, cancel_msg)
    }

    /// Renders the action sheet into an iced [`Element`].
    pub fn view(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let font_size = self.font_tier.base_size();
        let mut top_bar = Row::new()
            .width(Length::Fill)
            .align_y(iced::Alignment::Center);

        // Top-left: "✕" Close / Cancel capsule button
        if let Some(cancel) = &self.cancel {
            let cancel_icon = cancel.icon.unwrap_or(Icon::CLOSE);
            let cancel_msg = if cancel.enabled {
                Some(cancel.message.clone())
            } else {
                None
            };
            top_bar = top_bar.push(capsule_icon_button(
                cancel_icon,
                cancel_msg,
                cancel.style,
                true,
                self.theme,
            ));
        } else {
            top_bar = top_bar.push(Space::new().width(Length::Fixed(CAPSULE_BUTTON_WIDTH)));
        }

        // Center space or title
        if self.custom_body.is_some() && (self.title.is_some() || self.title_icon.is_some()) {
            let mut title_row = Row::new()
                .align_y(iced::Alignment::Center)
                .spacing(8);

            if let Some(icon) = self.title_icon {
                title_row = title_row.push(
                    crate::icon(icon)
                        .size(font_size)
                        .color(self.theme.accent()),
                );
            }

            if let Some(title) = &self.title {
                title_row = title_row.push(
                    text(title.clone())
                        .size(font_size)
                        .color(self.theme.text_primary()),
                );
            }

            top_bar = top_bar.push(
                container(title_row)
                    .width(Length::Fill)
                    .center_x(Length::Fill),
            );
        } else {
            top_bar = top_bar.push(Space::new().width(Length::Fill));
        }

        // Top-right: "✓" Confirm / Done capsule button
        if let Some(action) = self.actions.first() {
            let action_icon = action.icon.unwrap_or(Icon::CHECK);
            let action_msg = if action.enabled {
                Some(action.message.clone())
            } else {
                None
            };
            top_bar = top_bar.push(capsule_icon_button(
                action_icon,
                action_msg,
                action.style,
                false,
                self.theme,
            ));
        } else if let Some(cancel) = &self.cancel {
            // For a notice modal with a single dismiss/OK action, the right button confirms acknowledgment.
            let cancel_msg = if cancel.enabled {
                Some(cancel.message.clone())
            } else {
                None
            };
            top_bar = top_bar.push(capsule_icon_button(
                Icon::CHECK,
                cancel_msg,
                ActionStyle::Primary,
                false,
                self.theme,
            ));
        } else {
            top_bar = top_bar.push(Space::new().width(Length::Fixed(CAPSULE_BUTTON_WIDTH)));
        }

        let top_container = container(top_bar)
            .width(Length::Fill)
            .padding(Padding {
                top: 14.0,
                bottom: 6.0,
                left: 16.0,
                right: 16.0,
            });

        let body_container = if let Some(custom) = self.custom_body {
            let padding = self.body_padding.unwrap_or(Padding::ZERO);
            container(custom)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(padding)
        } else {
            let mut body_col = Column::new()
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .spacing(10);

            if self.title_icon.is_some() || self.title.is_some() {
                let mut title_row = Row::new()
                    .align_y(iced::Alignment::Center)
                    .spacing(8);

                if let Some(icon) = self.title_icon {
                    title_row = title_row.push(
                        crate::icon(icon)
                            .size(font_size)
                            .color(self.theme.accent()),
                    );
                }

                if let Some(title) = &self.title {
                    title_row = title_row.push(
                        text(title.clone())
                            .size(font_size)
                            .color(self.theme.text_primary()),
                    );
                }

                body_col = body_col.push(title_row);
            }

            if let Some(msg) = &self.message {
                body_col = body_col.push(
                    text(msg.clone())
                        .size(font_size)
                        .color(self.theme.text_secondary())
                        .align_x(iced::alignment::Horizontal::Center),
                );
            }

            let padding = self.body_padding.unwrap_or(Padding {
                top: 8.0,
                bottom: 24.0,
                left: 24.0,
                right: 24.0,
            });

            container(body_col)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .padding(padding)
        };

        let sheet_content = Column::with_children(vec![
            top_container.into(),
            body_container.into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill);

        let theme = self.theme;
        let sheet = container(sheet_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(theme.card().into()),
                border: Border {
                    color: theme.border(),
                    width: 1.0,
                    radius: Radius {
                        top_left: 0.0,
                        top_right: 0.0,
                        bottom_left: 0.0,
                        bottom_right: 0.0,
                    },
                },
                ..container::Style::default()
            });

        opaque(
            container(sheet)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .align_y(iced::alignment::Vertical::Bottom)
                .padding(0)
                .style(|_t| container::Style {
                    background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.6).into()),
                    ..container::Style::default()
                }),
        )
        .into()
    }
}

/// Helper function to build a standard top capsule action button (72×48, radius 24).
fn capsule_icon_button<'a, Message: Clone + 'a>(
    icon: Icon,
    message: Option<Message>,
    style: ActionStyle,
    is_cancel: bool,
    theme: ThemeMode,
) -> Element<'a, Message> {
    let (normal_bg, pressed_bg, icon_color) = if is_cancel {
        match theme {
            ThemeMode::Dark => (
                Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                Color::from_rgba(1.0, 1.0, 1.0, 0.85),
            ),
            ThemeMode::Light => (
                Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                Color::from_rgba(0.0, 0.0, 0.0, 0.14),
                Color::from_rgba(0.0, 0.0, 0.0, 0.85),
            ),
        }
    } else {
        match style {
            ActionStyle::Primary => (
                theme.accent(),
                Color::from_rgb(
                    theme.accent().r * 0.85,
                    theme.accent().g * 0.85,
                    theme.accent().b * 0.85,
                ),
                Color::WHITE,
            ),
            ActionStyle::Destructive => (
                theme.danger(),
                Color::from_rgb(
                    theme.danger().r * 0.85,
                    theme.danger().g * 0.85,
                    theme.danger().b * 0.85,
                ),
                Color::WHITE,
            ),
            ActionStyle::Secondary => match theme {
                ThemeMode::Dark => (
                    Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                    Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                    theme.text_primary(),
                ),
                ThemeMode::Light => (
                    Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                    Color::from_rgba(0.0, 0.0, 0.0, 0.14),
                    theme.text_primary(),
                ),
            },
        }
    };

    let btn = button(
        container(
            text(icon.glyph())
                .font(pomelo_material_symbols::font())
                .size(CAPSULE_BUTTON_GLYPH)
                .color(icon_color),
        )
        .center_x(Length::Fixed(CAPSULE_BUTTON_WIDTH))
        .center_y(Length::Fixed(CAPSULE_BUTTON_HEIGHT)),
    )
    .padding(0)
    .style(move |_t, status| {
        let bg = if matches!(status, button::Status::Pressed | button::Status::Hovered) {
            pressed_bg
        } else {
            normal_bg
        };
        button::Style {
            background: Some(bg.into()),
            border: Border {
                radius: (CAPSULE_BUTTON_HEIGHT / 2.0).into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
            snap: false,
            ..button::Style::default()
        }
    });

    if let Some(msg) = message {
        btn.on_press(msg).into()
    } else {
        btn.into()
    }
}

impl<'a, Message: Clone + 'a> From<ActionSheet<'a, Message>> for Element<'a, Message> {
    fn from(sheet: ActionSheet<'a, Message>) -> Self {
        sheet.view()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_sheet_notice_builds() {
        let sheet: ActionSheet<usize> = ActionSheet::notice(
            "Sync Complete",
            "Today's schedule was successfully updated.",
            "OK",
            1,
            ThemeMode::Dark,
        );
        let _elem: Element<'_, usize> = sheet.view();
    }

    #[test]
    fn action_sheet_destructive_confirm_builds() {
        let sheet: ActionSheet<usize> = ActionSheet::destructive_confirm(
            "Format SD card?",
            "Format",
            2,
            "Cancel",
            0,
            ThemeMode::Light,
        );
        let _elem: Element<'_, usize> = sheet.view();
    }

    #[test]
    fn action_sheet_multi_actions_builds() {
        let sheet: ActionSheet<usize> = ActionSheet::new(ThemeMode::Dark)
            .title("Options")
            .message("Choose an option")
            .action("Option A", 10, ActionStyle::Primary)
            .action("Option B", 20, ActionStyle::Secondary)
            .destructive("Delete", 30)
            .cancel("Cancel", 0);
        let _elem: Element<'_, usize> = sheet.view();
    }

    #[test]
    fn action_sheet_custom_body_builds() {
        let sheet = ActionSheet::new(ThemeMode::Dark)
            .title_icon(Icon::WIFI)
            .title("Home-5G")
            .action_item(ActionItem::new("Connect", 1).icon(Icon::CHECK))
            .cancel_item(ActionItem::secondary("Cancel", 0).icon(Icon::CLOSE))
            .custom_body(text("Custom Input Widget"));
        let _elem: Element<'_, usize> = sheet.view();
    }

    #[test]
    fn action_sheet_font_tier_defaults_to_standard() {
        let sheet: ActionSheet<usize> = ActionSheet::new(ThemeMode::Dark);
        assert_eq!(sheet.font_tier, FontTier::Standard);
        assert_eq!(sheet.font_tier.base_size(), 24.0);

        let custom_tier_sheet = sheet.font_tier(FontTier::Small);
        assert_eq!(custom_tier_sheet.font_tier, FontTier::Small);
        assert_eq!(custom_tier_sheet.font_tier.base_size(), 20.0);
    }
}
