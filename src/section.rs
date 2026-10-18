use std::borrow::Cow;
use iced::widget::{container, text, Column};
use iced::{Border, Element, Length, Padding};

use crate::item::{Item, ItemGroup};
use crate::preferences::ThemeMode;

/// Default corner radius for standard sections and cards (16px).
pub const SECTION_RADIUS: f32 = 16.0;
/// Backward-compatible alias for [`SECTION_RADIUS`].
pub const CARD_RADIUS: f32 = SECTION_RADIUS;

/// Default hairline border width for sections and cards (1px).
pub const SECTION_BORDER_WIDTH: f32 = 1.0;
/// Backward-compatible alias for [`SECTION_BORDER_WIDTH`].
pub const CARD_BORDER_WIDTH: f32 = SECTION_BORDER_WIDTH;

/// A rounded surface container styled according to [`ThemeMode`].
///
/// Wraps arbitrary content in the standard Pomelo OS section surface (rounded corners, background, border).
pub fn section_surface<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: ThemeMode,
) -> iced::widget::Container<'a, Message> {
    container(content)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(theme.card().into()),
            border: Border {
                color: theme.border(),
                width: SECTION_BORDER_WIDTH,
                radius: SECTION_RADIUS.into(),
            },
            ..Default::default()
        })
}

/// Backward-compatible alias for [`section_surface`].
pub fn card_surface<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: ThemeMode,
) -> iced::widget::Container<'a, Message> {
    section_surface(content, theme)
}

/// A general-purpose visual section / card container providing rounded surfaces, borders,
/// and optional header / footer typography.
///
/// Functionally equivalent to Flutter's `Card` / SwiftUI's `Section`:
/// - Can wrap **arbitrary widgets** (`impl Into<Element<'a, Message>>`) via [`Section::new`].
/// - Can wrap a strongly-typed [`ItemGroup`] via [`Section::group`].
/// - Can directly wrap an iterator of [`Item`]s via [`Section::items`].
/// - Can display an informational empty-state notice via [`Section::notice`].
/// - Can display static key-value details via [`Section::details`].
pub struct Section<'a, Message> {
    content: Element<'a, Message>,
    theme: ThemeMode,
    header: Option<Cow<'static, str>>,
    footer: Option<Cow<'static, str>>,
}

impl<'a, Message: Clone + 'a> Section<'a, Message> {
    /// Creates a general-purpose section / card wrapping arbitrary content.
    pub fn new(content: impl Into<Element<'a, Message>>, theme: ThemeMode) -> Self {
        Self {
            content: content.into(),
            theme,
            header: None,
            footer: None,
        }
    }

    /// Creates a section specifically wrapping a strongly-typed [`ItemGroup`].
    ///
    /// The [`ItemGroup`] automatically renders items with dividers between rows.
    pub fn group(group: ItemGroup<Message>, theme: ThemeMode) -> Self {
        Self::new(group.render(theme), theme)
    }

    /// Creates a section directly from an iterator of strongly-typed [`Item`]s.
    pub fn items(items: impl IntoIterator<Item = Item<Message>>, theme: ThemeMode) -> Self {
        Self::group(ItemGroup::with_items(items), theme)
    }

    /// Creates an informational / notice card displaying a single status message.
    pub fn notice(label: impl Into<Cow<'static, str>>, theme: ThemeMode) -> Self {
        let content = container(
            text(label.into())
                .size(crate::item::DETAIL_FONT)
                .color(theme.text_secondary()),
        )
        .width(Length::Fill)
        .padding(crate::item::ITEM_PADDING);

        Self::new(content, theme)
    }

    /// Creates a card displaying key-value detail rows.
    pub fn details(
        rows: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
        theme: ThemeMode,
    ) -> Self {
        Self::group(ItemGroup::from_pairs(rows), theme)
    }

    /// Sets an optional header title above the card surface.
    pub fn header(mut self, header: impl Into<Cow<'static, str>>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Sets an optional footer caption below the card surface.
    pub fn footer(mut self, footer: impl Into<Cow<'static, str>>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Renders this section to an [`Element`].
    pub fn view(self) -> Element<'a, Message> {
        let theme = self.theme;
        let surface = section_surface(self.content, theme);

        if self.header.is_none() && self.footer.is_none() {
            return surface.into();
        }

        let mut col = Column::new().width(Length::Fill).spacing(6.0);

        if let Some(h) = self.header {
            col = col.push(
                container(
                    text(h)
                        .size(16.0)
                        .color(theme.text_secondary()),
                )
                .padding(Padding {
                    left: 16.0,
                    right: 16.0,
                    top: 2.0,
                    bottom: 0.0,
                }),
            );
        }

        col = col.push(surface);

        if let Some(f) = self.footer {
            col = col.push(
                container(
                    text(f)
                        .size(14.0)
                        .color(theme.text_secondary()),
                )
                .padding(Padding {
                    left: 16.0,
                    right: 16.0,
                    top: 0.0,
                    bottom: 2.0,
                }),
            );
        }

        col.into()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::{Item, OpenPage};

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestMsg {
        Navigate,
    }

    #[test]
    fn section_with_group_builds() {
        let group = ItemGroup::new()
            .push(Item::key_value("A", "B"))
            .push(Item::nav("C", None::<&str>, OpenPage(TestMsg::Navigate)));
        let section = Section::group(group, ThemeMode::Dark)
            .header("Header")
            .footer("Footer");

        let _element: Element<'_, TestMsg> = section.view();
    }

    #[test]
    fn section_with_arbitrary_component_builds() {
        let custom_text = text("Hello arbitrary content");
        let section = Section::new(custom_text, ThemeMode::Light)
            .header("Custom Card")
            .footer("Custom Footer");

        let _element: Element<'_, ()> = section.view();
    }

    #[test]
    fn section_notice_and_details_build() {
        let notice = Section::<()>::notice("Wi-Fi Off", ThemeMode::Dark);
        let _elem = notice.view();

        let details = Section::<()>::details(
            vec![("Firmware", "v1.2.0"), ("Board", "ESP32-S3")],
            ThemeMode::Light,
        );
        let _elem = details.view();
    }
}
