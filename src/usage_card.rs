//! Progress and capacity meter card component conforming to Pomelo OS design language.

use std::borrow::Cow;
use iced::widget::{container, text, Column, Row, Space};
use iced::{Border, Color, Element, Length, Padding};

use crate::item::{detail_row, item_separator, DETAIL_FONT, ITEM_PADDING};
use crate::preferences::ThemeMode;
use crate::section::Section;

/// Default height of the horizontal progress track bar (6px).
pub const BAR_HEIGHT: f32 = 6.0;
/// Default corner radius of the horizontal progress track bar (3px).
pub const BAR_RADIUS: f32 = 3.0;
/// Gap between the label line and the progress track bar (10px).
pub const BAR_GAP: f32 = 10.0;
/// Padding for the usage bar container.
pub const USAGE_PADDING: Padding = ITEM_PADDING;

/// A card with a labelled progress bar and optional detail rows (e.g. memory usage or storage capacity).
pub struct UsageCard<'a, Message> {
    title: Cow<'static, str>,
    value: Cow<'static, str>,
    value_color: Option<Color>,
    percent: f32,
    bar_color: Color,
    theme: ThemeMode,
    details: Vec<(Cow<'static, str>, Cow<'static, str>)>,
    _phantom: std::marker::PhantomData<&'a Message>,
}

impl<'a, Message: Clone + 'a> UsageCard<'a, Message> {
    /// Creates a new [`UsageCard`].
    pub fn new(
        title: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
        percent: f32,
        bar_color: Color,
        theme: ThemeMode,
    ) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            value_color: None,
            percent,
            bar_color,
            theme,
            details: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Sets an explicit text color for the value text on the right side of the header.
    pub fn value_color(mut self, color: Color) -> Self {
        self.value_color = Some(color);
        self
    }

    /// Appends a key-value detail row to the bottom of the card below a separator.
    pub fn detail(
        mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.details.push((key.into(), value.into()));
        self
    }

    /// Appends multiple key-value detail rows to the bottom of the card.
    pub fn details(
        mut self,
        pairs: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
    ) -> Self {
        for (k, v) in pairs {
            self.details.push((k.into(), v.into()));
        }
        self
    }

    /// Renders the card into an [`Element`].
    pub fn view(self) -> Element<'a, Message> {
        let theme = self.theme;
        let percent = self.percent.clamp(0.0, 100.0);
        let share = (percent * 100.0).round() as u16;
        let rest = 10_000u16.saturating_sub(share);

        let track_content: Element<'a, Message> = if share == 0 {
            Space::new()
                .width(Length::Fill)
                .height(Length::Fixed(BAR_HEIGHT))
                .into()
        } else if share >= 10_000 {
            container(Space::new().width(Length::Fill))
                .width(Length::Fill)
                .height(Length::Fixed(BAR_HEIGHT))
                .style(move |_| container::Style {
                    background: Some(self.bar_color.into()),
                    border: Border {
                        radius: BAR_RADIUS.into(),
                        ..Border::default()
                    },
                    ..Default::default()
                })
                .into()
        } else {
            let filled = container(Space::new().width(Length::Fill))
                .width(Length::FillPortion(share))
                .height(Length::Fixed(BAR_HEIGHT))
                .style(move |_| container::Style {
                    background: Some(self.bar_color.into()),
                    border: Border {
                        radius: BAR_RADIUS.into(),
                        ..Border::default()
                    },
                    ..Default::default()
                });

            let remaining = container(Space::new().width(Length::Fill))
                .width(Length::FillPortion(rest))
                .height(Length::Fixed(BAR_HEIGHT));

            Row::with_children(vec![filled.into(), remaining.into()])
                .width(Length::Fill)
                .height(Length::Fixed(BAR_HEIGHT))
                .into()
        };

        let track = container(track_content)
            .width(Length::Fill)
            .height(Length::Fixed(BAR_HEIGHT))
            .style(move |_| container::Style {
                background: Some(match theme {
                    ThemeMode::Dark => Color::from_rgba(1.0, 1.0, 1.0, 0.12).into(),
                    ThemeMode::Light => Color::from_rgba(0.0, 0.0, 0.0, 0.08).into(),
                }),
                border: Border {
                    radius: BAR_RADIUS.into(),
                    ..Border::default()
                },
                ..Default::default()
            });

        let val_color = self.value_color.unwrap_or_else(|| theme.text_secondary());

        let heading = Row::with_children(vec![
            text(self.title)
                .size(DETAIL_FONT)
                .color(theme.text_primary())
                .into(),
            Space::new().width(Length::Fill).into(),
            text(self.value)
                .size(DETAIL_FONT)
                .color(val_color)
                .into(),
        ])
        .width(Length::Fill);

        let top_section = container(
            Column::with_children(vec![
                heading.into(),
                Space::new().height(Length::Fixed(BAR_GAP)).into(),
                track.into(),
            ])
            .width(Length::Fill),
        )
        .padding(USAGE_PADDING);

        let mut children: Vec<Element<'a, Message>> = vec![top_section.into()];

        if !self.details.is_empty() {
            children.push(item_separator::<Message>(theme).into());
            for (k, v) in self.details {
                children.push(detail_row::<Message>(k, v, theme));
            }
        }

        Section::new(Column::with_children(children).width(Length::Fill), theme).view()
    }
}

/// Convenience helper to create a basic usage bar card.
pub fn usage_bar<'a, Message: Clone + 'a>(
    title: impl Into<Cow<'static, str>>,
    value: impl Into<Cow<'static, str>>,
    value_color: Color,
    percent: f32,
    bar_color: Color,
    theme: ThemeMode,
) -> Element<'a, Message> {
    UsageCard::new(title, value, percent, bar_color, theme)
        .value_color(value_color)
        .view()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    enum TestMsg {}

    #[test]
    fn usage_card_renders_without_details() {
        let card = UsageCard::<TestMsg>::new("Memory", "4.2 MB / 8.0 MB", 52.5, Color::from_rgb(0.2, 0.8, 0.4), ThemeMode::Dark);
        let _elem = card.view();
    }

    #[test]
    fn usage_card_renders_with_details() {
        let card = UsageCard::<TestMsg>::new("Internal Flash", "2.1 MB / 16.0 MB", 13.1, Color::from_rgb(0.2, 0.4, 0.9), ThemeMode::Light)
            .detail("Filesystem", "LittleFS")
            .detail("Partition", "storage");
        let _elem = card.view();
    }
}
