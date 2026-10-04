//! Application metadata shared between sub-apps and system launchers.

use iced::Color;
use pomelo_material_symbols::Icon;

use crate::preferences::Language;

/// Metadata describing an application's identity in the launcher and shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppMeta {
    /// Shown under the tile (canonical English name).
    pub name: &'static str,
    /// Simplified Chinese name.
    pub name_zh: &'static str,
    /// Drawn inside the tile.
    pub icon: Icon,
    /// The tile's accent, as RGB bytes so this table stays a plain `const`.
    pub accent: (u8, u8, u8),
}

impl AppMeta {
    /// Constructs a new [`AppMeta`] with the given names, icon, and accent color.
    pub const fn new(
        name: &'static str,
        name_zh: &'static str,
        icon: Icon,
        accent: (u8, u8, u8),
    ) -> Self {
        Self {
            name,
            name_zh,
            icon,
            accent,
        }
    }

    /// The localized name in `language`.
    pub const fn localized_name(&self, language: Language) -> &'static str {
        match language {
            Language::Chinese => self.name_zh,
            Language::English => self.name,
        }
    }

    /// Canonical English name.
    pub const fn name_en(&self) -> &'static str {
        self.name
    }

    /// The accent, as an iced [`Color`].
    pub fn color(&self) -> Color {
        let (r, g, b) = self.accent;
        Color::from_rgb8(r, g, b)
    }
}
