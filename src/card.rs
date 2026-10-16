//! Standard card containers, surfaces, and row elements conforming to Pomelo OS design language.

use iced::widget::{container, rule, text, Row, Space};
use iced::{Alignment, Border, Element, Length, Padding};
use crate::preferences::ThemeMode;

/// Default corner radius for standard cards (16px).
pub const CARD_RADIUS: f32 = 16.0;
/// Default hairline border width for cards (1px).
pub const CARD_BORDER_WIDTH: f32 = 1.0;

/// Default vertical and horizontal padding for card items.
pub const CARD_PADDING: Padding = Padding {
    top: 12.0,
    bottom: 12.0,
    left: 16.0,
    right: 16.0,
};

/// A rounded surface container styled according to [`ThemeMode`].
pub fn card_surface<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    theme: ThemeMode,
) -> iced::widget::Container<'a, Message> {
    container(content)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(theme.card().into()),
            border: Border {
                color: theme.border(),
                width: CARD_BORDER_WIDTH,
                radius: CARD_RADIUS.into(),
            },
            ..Default::default()
        })
}

/// A standard horizontal separator rule for use inside cards.
pub fn card_separator<'a, Message: 'a>(theme: ThemeMode) -> iced::widget::Rule<'a> {
    rule::horizontal(1)
        .style(move |_| rule::Style {
            color: theme.separator(),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: false,
        })
}

/// A standard key-value detail row inside a card.
pub fn detail_row<'a, Message: 'a>(
    key: impl Into<String>,
    value: impl Into<String>,
    theme: ThemeMode,
) -> Element<'a, Message> {
    let key_str = key.into();
    let val_str = value.into();
    let row = Row::new()
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .push(
            text(key_str)
                .size(16.0)
                .color(theme.text_secondary()),
        )
        .push(Space::new().width(Length::Fill))
        .push(
            text(val_str)
                .size(16.0)
                .color(theme.text_primary()),
        );

    container(row)
        .width(Length::Fill)
        .padding(CARD_PADDING)
        .into()
}
