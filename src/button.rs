//! Generic rounded button component conforming to Pomelo OS design language.
//!
//! Provides customizable, high-precision tactile buttons supporting:
//! - Visual variants: `Secondary` (translucent frame), `Primary` (filled accent),
//!   `Tinted` (accent-tinted frame & surface), `Outline` (clean border), `Ghost`, and `Destructive`
//! - Shapes: `Pill` (full circular ends / circular icon button) and `Rounded` (configurable radius)
//! - Sizes: `Compact` (32px), `Standard` (38px), `Large` (46px), and `Capsule` (48px)
//! - Content: text label, icon, or icon + text combination
//! - Automatic touch/hover state styling and accessibility

use std::borrow::Cow;
use iced::widget::{button, container, text, Row};
use iced::{Alignment, Border, Color, Element, Length, Padding, Shadow};
use pomelo_material_symbols::Icon;

use crate::preferences::ThemeMode;

/// Visual styling variant for a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    /// Translucent frosted background with a distinct 1px rounded border frame.
    /// Standard for top navigation bars, auxiliary controls, and header actions.
    #[default]
    Secondary,
    /// Solid filled accent background with white text/icon.
    Primary,
    /// Subtle accent-tinted background with a tinted border frame.
    Tinted,
    /// Transparent background with a distinct 1px border.
    Outline,
    /// Borderless and backgroundless plain button.
    Ghost,
    /// Destructive high-risk action button (system red).
    Destructive,
}

/// Size tier for a [`Button`], defining height, font size, icon size, and padding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonSize {
    /// Compact tier: 32px height, 14px font, 18px icon.
    Compact,
    /// Standard tier: 38px height, 16px font, 22px icon.
    #[default]
    Standard,
    /// Large tier: 46px height, 18px font, 24px icon.
    Large,
    /// Navigation capsule tier: 48px height, 22px font, 24px icon, min-width 72px.
    Capsule,
}

impl ButtonSize {
    /// The canonical height of this size tier in pixels.
    pub const fn height(self) -> f32 {
        match self {
            Self::Compact => 32.0,
            Self::Standard => 38.0,
            Self::Large => 46.0,
            Self::Capsule => 48.0,
        }
    }

    /// The text font size for this size tier in pixels.
    pub const fn font_size(self) -> f32 {
        match self {
            Self::Compact => 14.0,
            Self::Standard => 16.0,
            Self::Large => 18.0,
            Self::Capsule => 22.0,
        }
    }

    /// The icon glyph size for this size tier in pixels.
    pub const fn icon_size(self) -> f32 {
        match self {
            Self::Compact => 18.0,
            Self::Standard => 22.0,
            Self::Large => 24.0,
            Self::Capsule => 24.0,
        }
    }

    /// Default horizontal padding for text-bearing buttons in pixels.
    pub const fn horizontal_padding(self) -> f32 {
        match self {
            Self::Compact => 10.0,
            Self::Standard => 14.0,
            Self::Large => 18.0,
            Self::Capsule => 16.0,
        }
    }
}

/// Corner geometry of a [`Button`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonShape {
    /// Full capsule / pill (corner radius = height / 2.0).
    /// For icon-only square buttons, this produces a perfect circle.
    Pill,
    /// Rounded rectangle with specified corner radius in pixels.
    Rounded(f32),
}

impl Default for ButtonShape {
    fn default() -> Self {
        Self::Pill
    }
}

/// Strongly-typed, rounded button widget conforming to Pomelo OS design language.
pub struct Button<'a, Message> {
    text: Option<Cow<'static, str>>,
    icon: Option<Icon>,
    custom: Option<Element<'a, Message>>,
    on_press: Option<Message>,
    variant: ButtonVariant,
    size: ButtonSize,
    shape: ButtonShape,
    theme: ThemeMode,
    color: Option<Color>,
    enabled: bool,
    width: Option<Length>,
    height: Option<Length>,
    border_width: f32,
    border_color: Option<Color>,
    padding: Option<Padding>,
}

impl<'a, Message> std::fmt::Debug for Button<'a, Message> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("text", &self.text)
            .field("icon", &self.icon)
            .field("variant", &self.variant)
            .field("size", &self.size)
            .field("shape", &self.shape)
            .field("theme", &self.theme)
            .field("color", &self.color)
            .field("enabled", &self.enabled)
            .finish()
    }
}

impl<'a, Message> Button<'a, Message> {
    /// Creates a button with text label.
    pub fn new(text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: Some(text.into()),
            icon: None,
            custom: None,
            on_press: None,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Standard,
            shape: ButtonShape::Pill,
            theme: ThemeMode::Dark,
            color: None,
            enabled: true,
            width: None,
            height: None,
            border_width: 1.0,
            border_color: None,
            padding: None,
        }
    }

    /// Alias for [`new`](Self::new).
    pub fn text(text: impl Into<Cow<'static, str>>) -> Self {
        Self::new(text)
    }

    /// Creates an icon-only button (defaults to circular pill with equal width and height).
    pub fn icon(icon: Icon) -> Self {
        Self {
            text: None,
            icon: Some(icon),
            custom: None,
            on_press: None,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Standard,
            shape: ButtonShape::Pill,
            theme: ThemeMode::Dark,
            color: None,
            enabled: true,
            width: None,
            height: None,
            border_width: 1.0,
            border_color: None,
            padding: None,
        }
    }

    /// Creates a button with both an icon and text label.
    pub fn icon_text(icon: Icon, text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            text: Some(text.into()),
            icon: Some(icon),
            custom: None,
            on_press: None,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Standard,
            shape: ButtonShape::Pill,
            theme: ThemeMode::Dark,
            color: None,
            enabled: true,
            width: None,
            height: None,
            border_width: 1.0,
            border_color: None,
            padding: None,
        }
    }

    /// Creates a button wrapping a custom element.
    pub fn custom(element: impl Into<Element<'a, Message>>) -> Self {
        Self {
            text: None,
            icon: None,
            custom: Some(element.into()),
            on_press: None,
            variant: ButtonVariant::Secondary,
            size: ButtonSize::Standard,
            shape: ButtonShape::Pill,
            theme: ThemeMode::Dark,
            color: None,
            enabled: true,
            width: None,
            height: None,
            border_width: 1.0,
            border_color: None,
            padding: None,
        }
    }

    /// Sets the message dispatched when the button is pressed.
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the optional message dispatched when the button is pressed.
    pub fn on_press_opt(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Sets the button visual styling variant.
    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets primary filled accent style.
    pub fn primary(mut self) -> Self {
        self.variant = ButtonVariant::Primary;
        self
    }

    /// Sets secondary translucent framed style.
    pub fn secondary(mut self) -> Self {
        self.variant = ButtonVariant::Secondary;
        self
    }

    /// Sets tinted accent frame style.
    pub fn tinted(mut self) -> Self {
        self.variant = ButtonVariant::Tinted;
        self
    }

    /// Sets outline border style.
    pub fn outline(mut self) -> Self {
        self.variant = ButtonVariant::Outline;
        self
    }

    /// Sets ghost borderless style.
    pub fn ghost(mut self) -> Self {
        self.variant = ButtonVariant::Ghost;
        self
    }

    /// Sets destructive danger red style.
    pub fn destructive(mut self) -> Self {
        self.variant = ButtonVariant::Destructive;
        self
    }

    /// Sets the button size tier.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets compact size (height 32px).
    pub fn compact(mut self) -> Self {
        self.size = ButtonSize::Compact;
        self
    }

    /// Sets standard size (height 38px).
    pub fn standard(mut self) -> Self {
        self.size = ButtonSize::Standard;
        self
    }

    /// Sets large size (height 46px).
    pub fn large(mut self) -> Self {
        self.size = ButtonSize::Large;
        self
    }

    /// Sets the corner shape.
    pub fn shape(mut self, shape: ButtonShape) -> Self {
        self.shape = shape;
        self
    }

    /// Sets full pill / capsule corner radius (radius = height / 2).
    pub fn pill(mut self) -> Self {
        self.shape = ButtonShape::Pill;
        self
    }

    /// Sets rounded rectangle corner radius in pixels.
    pub fn rounded(mut self, radius: f32) -> Self {
        self.shape = ButtonShape::Rounded(radius);
        self
    }

    /// Sets the theme mode.
    pub fn theme(mut self, theme: ThemeMode) -> Self {
        self.theme = theme;
        self
    }

    /// Sets a custom foreground accent or tint color (e.g. orange for Clock).
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    /// Sets whether the button is interactive.
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets fixed width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets fixed height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets custom padding.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets custom border width.
    pub fn border_width(mut self, width: f32) -> Self {
        self.border_width = width;
        self
    }

    /// Sets custom border color.
    pub fn border_color(mut self, color: impl Into<Color>) -> Self {
        self.border_color = Some(color.into());
        self
    }

    /// Builds and renders the button into an iced [`Element`].
    pub fn view(self) -> Element<'a, Message>
    where
        Message: Clone + 'a,
    {
        let h = self.size.height();
        let font_size = self.size.font_size();
        let icon_size = self.size.icon_size();
        let is_icon_only = self.icon.is_some() && self.text.is_none() && self.custom.is_none();

        let radius = match self.shape {
            ButtonShape::Pill => h / 2.0,
            ButtonShape::Rounded(r) => r,
        };

        let accent_color = self.color.unwrap_or(self.theme.accent());
        let theme = self.theme;

        // Determine colors based on variant and theme
        let (normal_bg, pressed_bg, text_color, frame_border_color) = match self.variant {
            ButtonVariant::Secondary => {
                let (nbg, pbg) = match theme {
                    ThemeMode::Dark => (
                        Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                        Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                    ),
                    ThemeMode::Light => (
                        Color::from_rgba(0.0, 0.0, 0.0, 0.06),
                        Color::from_rgba(0.0, 0.0, 0.0, 0.14),
                    ),
                };
                let fg = self.color.unwrap_or_else(|| theme.text_primary());
                let b_col = self.border_color.unwrap_or_else(|| match theme {
                    ThemeMode::Dark => Color::from_rgba(1.0, 1.0, 1.0, 0.18),
                    ThemeMode::Light => Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                });
                (nbg, pbg, fg, b_col)
            }
            ButtonVariant::Tinted => {
                let nbg = Color::from_rgba(accent_color.r, accent_color.g, accent_color.b, 0.12);
                let pbg = Color::from_rgba(accent_color.r, accent_color.g, accent_color.b, 0.24);
                let b_col = self.border_color.unwrap_or_else(|| {
                    Color::from_rgba(accent_color.r, accent_color.g, accent_color.b, 0.35)
                });
                (nbg, pbg, accent_color, b_col)
            }
            ButtonVariant::Primary => {
                let nbg = accent_color;
                let pbg = Color::from_rgb(
                    accent_color.r * 0.85,
                    accent_color.g * 0.85,
                    accent_color.b * 0.85,
                );
                let b_col = self.border_color.unwrap_or(accent_color);
                (nbg, pbg, Color::WHITE, b_col)
            }
            ButtonVariant::Destructive => {
                let danger = theme.danger();
                let nbg = danger;
                let pbg = Color::from_rgb(danger.r * 0.85, danger.g * 0.85, danger.b * 0.85);
                let b_col = self.border_color.unwrap_or(danger);
                (nbg, pbg, Color::WHITE, b_col)
            }
            ButtonVariant::Outline => {
                let nbg = Color::TRANSPARENT;
                let pbg = match theme {
                    ThemeMode::Dark => Color::from_rgba(1.0, 1.0, 1.0, 0.10),
                    ThemeMode::Light => Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                };
                let fg = self.color.unwrap_or_else(|| theme.text_primary());
                let b_col = self.border_color.unwrap_or_else(|| accent_color);
                (nbg, pbg, fg, b_col)
            }
            ButtonVariant::Ghost => {
                let nbg = Color::TRANSPARENT;
                let pbg = match theme {
                    ThemeMode::Dark => Color::from_rgba(1.0, 1.0, 1.0, 0.10),
                    ThemeMode::Light => Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                };
                let fg = self.color.unwrap_or_else(|| theme.text_primary());
                (nbg, pbg, fg, Color::TRANSPARENT)
            }
        };

        // Determine content layout
        let inner_element: Element<'a, Message> = if let Some(custom) = self.custom {
            custom
        } else if let (Some(icon), Some(text_label)) = (self.icon, &self.text) {
            Row::new()
                .spacing(8)
                .align_y(Alignment::Center)
                .push(
                    crate::icon(icon)
                        .size(icon_size)
                        .color(text_color),
                )
                .push(
                    text(text_label.clone())
                        .size(font_size)
                        .color(text_color),
                )
                .into()
        } else if let Some(icon) = self.icon {
            crate::icon(icon)
                .size(icon_size)
                .color(text_color)
                .into()
        } else if let Some(text_label) = &self.text {
            text(text_label.clone())
                .size(font_size)
                .color(text_color)
                .into()
        } else {
            text("").into()
        };

        let padding = self.padding.unwrap_or_else(|| {
            if is_icon_only {
                Padding::ZERO
            } else {
                Padding {
                    top: 0.0,
                    bottom: 0.0,
                    left: self.size.horizontal_padding(),
                    right: self.size.horizontal_padding(),
                }
            }
        });

        // Determine container dimensions
        let mut inner_container = container(inner_element)
            .padding(padding)
            .center_y(Length::Fixed(h));

        if let Some(w) = self.width {
            inner_container = inner_container.center_x(w);
        } else if is_icon_only {
            let icon_btn_w = if self.size == ButtonSize::Capsule {
                crate::sub_page::CAPSULE_BUTTON_WIDTH
            } else {
                h
            };
            inner_container = inner_container.center_x(Length::Fixed(icon_btn_w));
        }

        let border_width = if self.variant == ButtonVariant::Ghost {
            0.0
        } else {
            self.border_width
        };

        let mut btn = button(inner_container)
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
                        color: frame_border_color,
                        width: border_width,
                        radius: radius.into(),
                    },
                    shadow: Shadow::default(),
                    snap: false,
                    ..button::Style::default()
                }
            });

        if let Some(w) = self.width {
            btn = btn.width(w);
        } else if is_icon_only {
            let icon_btn_w = if self.size == ButtonSize::Capsule {
                crate::sub_page::CAPSULE_BUTTON_WIDTH
            } else {
                h
            };
            btn = btn.width(Length::Fixed(icon_btn_w));
        }

        if let Some(h_len) = self.height {
            btn = btn.height(h_len);
        } else {
            btn = btn.height(Length::Fixed(h));
        }

        if self.enabled {
            if let Some(msg) = self.on_press {
                btn.on_press(msg).into()
            } else {
                btn.into()
            }
        } else {
            btn.into()
        }
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Element<'a, Message> {
    fn from(btn: Button<'a, Message>) -> Self {
        btn.view()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn button_text_secondary_builds() {
        let btn = Button::<usize>::new("编辑")
            .on_press(1)
            .theme(ThemeMode::Dark);
        let _elem: Element<'_, usize> = btn.view();
    }

    #[test]
    fn button_icon_pill_builds() {
        let btn = Button::<usize>::icon(Icon::ADD)
            .on_press(2)
            .color(Color::from_rgb(1.0, 0.5, 0.0))
            .theme(ThemeMode::Dark);
        let _elem: Element<'_, usize> = btn.view();
    }

    #[test]
    fn button_variants_and_sizes_build() {
        let _p: Element<'_, usize> = Button::new("Primary").primary().large().view();
        let _t: Element<'_, usize> = Button::new("Tinted").tinted().compact().view();
        let _d: Element<'_, usize> = Button::new("Delete").destructive().rounded(8.0).view();
        let _o: Element<'_, usize> = Button::new("Outline").outline().view();
        let _g: Element<'_, usize> = Button::new("Ghost").ghost().view();
    }
}
