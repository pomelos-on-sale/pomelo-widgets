//! The keyboard's colours, by key role.
//!
//! The iOS dark keyboard palette, copied from the original keyboard's `theme.rs`: a letter key, a
//! modifier key, the return key, and the shift key while caps is on.

use iced::Color;

use super::keys::KeyKind;

/// The keyboard band's own background (#1C1C1E). The keys sit on it; the page behind them does not
/// show through.
pub fn background() -> Color {
    rgb((28, 28, 30))
}

/// The keyboard's key radius.
pub const KEY_RADIUS: f32 = 5.0;

/// The three colours one key paints with.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyPalette {
    /// The key at rest.
    pub fill: Color,
    /// The key under a finger.
    pub pressed: Color,
    /// The key's label.
    pub text: Color,
}

/// The iOS dark keyboard palette, by key role.
pub fn palette(kind: KeyKind) -> KeyPalette {
    let (fill, pressed, text) = match kind {
        // #48484A letter keys, #6E6E73 pressed
        KeyKind::Character => ((72, 72, 74), (110, 110, 115), (255, 255, 255)),
        // #2C2C2E special/modifier, #48484A pressed
        KeyKind::Special => ((44, 44, 46), (72, 72, 74), (255, 255, 255)),
        // #007AFF accent blue return, #3C96FF pressed
        KeyKind::Return => ((0, 122, 255), (60, 150, 255), (255, 255, 255)),
        // Shift while caps is on: white, with black text
        KeyKind::ShiftActive => ((255, 255, 255), (110, 110, 115), (0, 0, 0)),
    };

    KeyPalette {
        fill: rgb(fill),
        pressed: rgb(pressed),
        text: rgb(text),
    }
}

fn rgb((r, g, b): (u8, u8, u8)) -> Color {
    Color::from_rgb8(r, g, b)
}
