//! Standard full-screen sub-page and modal page containers for Pomelo OS apps.
//!
//! Provides unified navigation and modal overlays adhering to Pomelo OS design specifications:
//! - Full-screen dimmed backdrop (60% opacity) blocking background touches
//! - Reaches all screen edges (0 margin, 480×480px)
//! - Standard top navigation bar with:
//!   - Nav mode ([`SubPage::nav`]): Back button ("←") on left, centered title, symmetric 72px space on right.
//!   - Modal mode ([`SubPage::modal`]): Cancel button ("✕") on left, centered title, Confirm button ("✓") on right.
//! - Body hosting arbitrary custom [`Element`] with automatic vertical scrolling and configurable padding.

use iced::border::Radius;
use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{button, container, scrollable, text, Column, Row, Space};
use iced::{Border, Color, Element, Length, Padding, Shadow};
use pomelo_material_symbols::Icon;

use crate::opaque::opaque;
use crate::preferences::{FontTier, ThemeMode};

/// The default proportion of the screen height occupied by a [`SubPage`] (100%).
pub const SUB_PAGE_HEIGHT_RATIO: f32 = 1.0;
/// The unified fixed height in pixels of a [`SubPage`] on a 480px screen (480px).
pub const SUB_PAGE_HEIGHT: f32 = 480.0;
/// Width of top capsule action buttons (matching Pomelo OS capsule button: 72px).
pub const CAPSULE_BUTTON_WIDTH: f32 = 72.0;
/// Height of top capsule action buttons (matching Pomelo OS capsule button: 48px).
pub const CAPSULE_BUTTON_HEIGHT: f32 = 48.0;
/// Glyph font size for top capsule action buttons.
pub const CAPSULE_BUTTON_GLYPH: f32 = FontTier::Standard.base_size(); // 24.0 px
/// Title font size for top navigation bar in a [`SubPage`] (standard 24px).
pub const TOP_BAR_TITLE_FONT: f32 = FontTier::Standard.base_size();
/// Top bar icon size in a [`SubPage`] (standard 24px).
pub const TOP_BAR_ICON: f32 = FontTier::Standard.base_size();


/// Visual style of an action button within a [`SubPage`].
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

/// An individual action button in a [`SubPage`].
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

    /// Sets the style of this action button.
    pub fn style(mut self, style: ActionStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets whether this action button is enabled.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// A strongly-typed full-screen sub-page and modal container.
pub struct SubPage<'a, Message> {
    pub title: Option<String>,
    pub title_icon: Option<Icon>,
    pub leading: Option<ActionItem<Message>>,
    pub trailing: Option<ActionItem<Message>>,
    pub custom_body: Option<Element<'a, Message>>,
    pub body_padding: Option<Padding>,
    pub theme: ThemeMode,
    pub font_tier: FontTier,
}

impl<'a, Message> std::fmt::Debug for SubPage<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SubPage")
            .field("title", &self.title)
            .field("title_icon", &self.title_icon)
            .field("has_leading", &self.leading.is_some())
            .field("has_trailing", &self.trailing.is_some())
            .field("has_custom_body", &self.custom_body.is_some())
            .field("body_padding", &self.body_padding)
            .field("theme", &self.theme)
            .field("font_tier", &self.font_tier)
            .finish()
    }
}

impl<'a, Message> SubPage<'a, Message> {
    /// Creates a navigation sub-page with a back button ("←") on the left, centered title,
    /// and symmetric 72px space on the right for items like label, ringtone, or repeat selection.
    pub fn nav(title: impl Into<String>, back_msg: Message, theme: ThemeMode) -> Self {
        Self {
            title: Some(title.into()),
            title_icon: None,
            leading: Some(ActionItem::secondary("返回", back_msg).icon(Icon::ARROW_BACK)),
            trailing: None,
            custom_body: None,
            body_padding: None,
            theme,
            font_tier: FontTier::Standard,
        }
    }

    /// Creates a modal transaction sub-page with a centered title, ready for cancel ("✕") and confirm ("✓") actions.
    pub fn modal(title: impl Into<String>, theme: ThemeMode) -> Self {
        Self {
            title: Some(title.into()),
            title_icon: None,
            leading: None,
            trailing: None,
            custom_body: None,
            body_padding: None,
            theme,
            font_tier: FontTier::Standard,
        }
    }

    /// Creates an empty generic sub-page.
    pub fn new(theme: ThemeMode) -> Self {
        Self {
            title: None,
            title_icon: None,
            leading: None,
            trailing: None,
            custom_body: None,
            body_padding: None,
            theme,
            font_tier: FontTier::Standard,
        }
    }

    /// Sets the font tier for this [`SubPage`]. Defaults to [`FontTier::Standard`].
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

    /// Sets the left cancel / close action button ("✕", Secondary style).
    pub fn cancel(mut self, message: Message) -> Self {
        self.leading = Some(ActionItem::secondary("取消", message).icon(Icon::CLOSE));
        self
    }

    /// Sets the left back button ("←", Secondary style).
    pub fn back(mut self, message: Message) -> Self {
        self.leading = Some(ActionItem::secondary("返回", message).icon(Icon::ARROW_BACK));
        self
    }

    /// Sets a customized leading action item on the top-left.
    pub fn leading(mut self, item: ActionItem<Message>) -> Self {
        self.leading = Some(item);
        self
    }

    /// Sets the right confirm / done action button ("✓", Primary style).
    pub fn confirm(mut self, message: Message) -> Self {
        self.trailing = Some(ActionItem::new("确定", message).icon(Icon::CHECK));
        self
    }

    /// Sets a destructive (red) right confirm button.
    pub fn confirm_destructive(mut self, message: Message) -> Self {
        self.trailing = Some(ActionItem::destructive("确定", message).icon(Icon::CHECK));
        self
    }

    /// Sets a customized trailing action item on the top-right.
    pub fn trailing(mut self, item: ActionItem<Message>) -> Self {
        self.trailing = Some(item);
        self
    }

    /// Sets the custom body [`Element`] for this [`SubPage`].
    pub fn body(mut self, body: impl Into<Element<'a, Message>>) -> Self {
        self.custom_body = Some(body.into());
        self
    }

    /// Alias for [`body`](Self::body).
    pub fn custom_body(self, body: impl Into<Element<'a, Message>>) -> Self {
        self.body(body)
    }

    /// Sets custom padding for the body container.
    pub fn body_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.body_padding = Some(padding.into());
        self
    }

    /// Renders the sub-page into an iced [`Element`].
    pub fn view(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let font_size = self.font_tier.base_size();
        let mut top_bar = Row::new()
            .width(Length::Fill)
            .align_y(iced::Alignment::Center);

        // Top-left: Leading capsule button (Cancel / Back) or symmetric 72px space
        if let Some(leading) = &self.leading {
            let icon = leading.icon.unwrap_or(Icon::CLOSE);
            let msg = if leading.enabled {
                Some(leading.message.clone())
            } else {
                None
            };
            top_bar = top_bar.push(capsule_icon_button(
                icon,
                msg,
                leading.style,
                true,
                self.theme,
            ));
        } else {
            top_bar = top_bar.push(Space::new().width(Length::Fixed(CAPSULE_BUTTON_WIDTH)));
        }

        // Center: Title row strictly centered
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

        // Top-right: Trailing capsule button (Confirm / Done) or symmetric 72px space
        if let Some(trailing) = &self.trailing {
            let icon = trailing.icon.unwrap_or(Icon::CHECK);
            let msg = if trailing.enabled {
                Some(trailing.message.clone())
            } else {
                None
            };
            top_bar = top_bar.push(capsule_icon_button(
                icon,
                msg,
                trailing.style,
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
                .padding(padding)
        } else {
            container(Space::new()).width(Length::Fill)
        };

        let body_scrollable = scrollable(body_container)
            .direction(Direction::Vertical(Scrollbar::hidden()))
            .width(Length::Fill)
            .height(Length::Fill);

        let page_content = Column::with_children(vec![
            top_container.into(),
            crate::gesture::gesture_scope(body_scrollable).into(),
        ])
        .width(Length::Fill)
        .height(Length::Fill);

        let theme = self.theme;
        let sheet = container(page_content)
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
    }
}

/// Helper function to build a standard top capsule action button (72×48, radius 24).
pub fn capsule_icon_button<'a, Message: Clone + 'a>(
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

impl<'a, Message: Clone + 'a> From<SubPage<'a, Message>> for Element<'a, Message> {
    fn from(sub_page: SubPage<'a, Message>) -> Self {
        sub_page.view()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_page_nav_builds() {
        let page = SubPage::<usize>::nav("标签", 0, ThemeMode::Dark)
            .body(text("Label options"));
        let _elem: Element<'_, usize> = page.view();
    }

    #[test]
    fn sub_page_modal_builds() {
        let page = SubPage::<usize>::modal("编辑闹钟", ThemeMode::Dark)
            .cancel(0)
            .confirm(1)
            .body(text("Alarm form"));
        let _elem: Element<'_, usize> = page.view();
    }

    #[test]
    fn sub_page_destructive_confirm_builds() {
        let page = SubPage::<usize>::modal("删除确认", ThemeMode::Light)
            .cancel(0)
            .confirm_destructive(2)
            .body(text("Are you sure?"));
        let _elem: Element<'_, usize> = page.view();
    }

    #[test]
    fn sub_page_font_tier_defaults_to_standard() {
        let page: SubPage<usize> = SubPage::new(ThemeMode::Dark);
        assert_eq!(page.font_tier, FontTier::Standard);
        assert_eq!(page.font_tier.base_size(), 24.0);

        let custom = page.font_tier(FontTier::Small);
        assert_eq!(custom.font_tier, FontTier::Small);
        assert_eq!(custom.font_tier.base_size(), 20.0);
    }
}
