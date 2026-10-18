//! iOS-style toggle switch widget conforming to Pomelo OS design language.

use iced::widget::{button, container, Row, Space};
use iced::{Alignment, Border, Color, Element, Length, Shadow};

use crate::preferences::ThemeMode;

/// Width of the standard iOS toggle switch (52px).
pub const TOGGLE_W: f32 = 52.0;
/// Height of the standard iOS toggle switch (28px).
pub const TOGGLE_H: f32 = 28.0;
/// Corner radius of the toggle switch track (14px).
pub const TOGGLE_RADIUS: f32 = 14.0;
/// Inner margin / inset for the knob within the switch track (2px).
pub const TOGGLE_INSET: f32 = 2.0;
/// Diameter of the toggle switch knob (24px).
pub const KNOB_SIZE: f32 = 24.0;
/// Corner radius of the knob (12px).
pub const KNOB_RADIUS: f32 = 12.0;

/// Standard iOS-style toggle switch widget.
pub fn toggle_switch<'a, Message: Clone + 'a>(
    on: bool,
    on_toggle: Message,
    _theme: ThemeMode,
) -> Element<'a, Message> {
    let knob = container(Space::new())
        .width(Length::Fixed(KNOB_SIZE))
        .height(Length::Fixed(KNOB_SIZE))
        .style(|_| container::Style {
            background: Some(Color::WHITE.into()),
            border: Border {
                radius: KNOB_RADIUS.into(),
                ..Border::default()
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: iced::Vector::new(0.0, 2.0),
                blur_radius: 4.0,
            },
            ..container::Style::default()
        });

    let mut parts: Vec<Element<'a, Message>> = Vec::new();
    if on {
        parts.push(Space::new().width(Length::Fill).into());
        parts.push(knob.into());
        parts.push(Space::new().width(Length::Fixed(TOGGLE_INSET)).into());
    } else {
        parts.push(Space::new().width(Length::Fixed(TOGGLE_INSET)).into());
        parts.push(knob.into());
        parts.push(Space::new().width(Length::Fill).into());
    }

    button(
        Row::with_children(parts)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(Alignment::Center),
    )
    .width(Length::Fixed(TOGGLE_W))
    .height(Length::Fixed(TOGGLE_H))
    .padding(0)
    .style(move |_, status| {
        let fill = if on {
            match status {
                button::Status::Pressed => Color::from_rgb(45.0 / 255.0, 170.0 / 255.0, 76.0 / 255.0),
                _ => Color::from_rgb(52.0 / 255.0, 199.0 / 255.0, 89.0 / 255.0),
            }
        } else {
            match status {
                button::Status::Pressed => Color::from_rgb(44.0 / 255.0, 44.0 / 255.0, 46.0 / 255.0),
                _ => Color::from_rgb(57.0 / 255.0, 57.0 / 255.0, 61.0 / 255.0),
            }
        };
        button::Style {
            background: Some(fill.into()),
            text_color: Color::WHITE,
            border: Border {
                radius: TOGGLE_RADIUS.into(),
                ..Border::default()
            },
            shadow: Shadow::default(),
            snap: false,
        }
    })
    .on_press(on_toggle)
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toggle_switch_builds() {
        let _on: Element<'_, ()> = toggle_switch(true, (), ThemeMode::Dark);
        let _off: Element<'_, ()> = toggle_switch(false, (), ThemeMode::Light);
    }
}
