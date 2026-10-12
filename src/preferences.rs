//! System-wide preferences and theme tokens shared across Pomelo OS applications.
//!
//! Provides the core types for:
//! - [`Language`]: interface language (Chinese or English)
//! - [`ThemeMode`]: display appearance (AMOLED Dark or Light)
//! - [`FontSizeTier`]: typography scale with three tiers (20px, 24px, 30px)
//! - [`SystemPreferences`]: aggregated system preference state

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::Duration;

use iced::Color;
use serde::{Deserialize, Serialize};

/// The interface language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Language {
    /// 简体中文 (Simplified Chinese). Default for Pomelo OS.
    #[default]
    Chinese,
    /// English.
    English,
}

impl Language {
    /// Returns the other language, for toggle actions.
    #[inline]
    pub fn other(self) -> Self {
        match self {
            Self::Chinese => Self::English,
            Self::English => Self::Chinese,
        }
    }

    /// What this language calls itself.
    #[inline]
    pub fn name(self) -> &'static str {
        match self {
            Self::Chinese => "中文",
            Self::English => "English",
        }
    }

    /// Label for navigation / back actions in this language.
    #[inline]
    pub fn back_label(self) -> &'static str {
        match self {
            Self::Chinese => "返回",
            Self::English => "Back",
        }
    }
}

/// The visual theme mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ThemeMode {
    /// AMOLED pure black (#000000) for zero power on unlit pixels.
    #[default]
    Dark,
    /// Light mode with clean contrast.
    Light,
}

impl ThemeMode {
    /// Returns whether this is dark mode.
    #[inline]
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    /// Returns whether this is light mode.
    #[inline]
    pub fn is_light(self) -> bool {
        matches!(self, Self::Light)
    }

    /// Returns the other theme mode, for toggle actions.
    #[inline]
    pub fn other(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    /// Human-readable name in `language`.
    pub fn name(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Dark, Language::Chinese) => "深色模式",
            (Self::Dark, Language::English) => "Dark",
            (Self::Light, Language::Chinese) => "浅色模式",
            (Self::Light, Language::English) => "Light",
        }
    }

    /// Base background color.
    pub fn background(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(0, 0, 0),
            Self::Light => Color::from_rgb8(242, 242, 247),
        }
    }

    /// Surface / Card background color.
    pub fn card(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(28, 28, 30),
            Self::Light => Color::WHITE,
        }
    }

    /// Navigation bar background color.
    pub fn nav_bar(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(18, 18, 18),
            Self::Light => Color::from_rgb8(248, 248, 248),
        }
    }

    /// Primary text color.
    pub fn text_primary(self) -> Color {
        match self {
            Self::Dark => Color::WHITE,
            Self::Light => Color::from_rgb8(0, 0, 0),
        }
    }

    /// Secondary / muted text color.
    pub fn text_secondary(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(150, 150, 155),
            Self::Light => Color::from_rgb8(142, 142, 147),
        }
    }

    /// Card border / separator hairline.
    pub fn border(self) -> Color {
        match self {
            Self::Dark => Color::from_rgba8(255, 255, 255, 25.0 / 255.0),
            Self::Light => Color::from_rgba8(0, 0, 0, 20.0 / 255.0),
        }
    }

    /// Subtle row separator hairline.
    pub fn separator(self) -> Color {
        match self {
            Self::Dark => Color::from_rgba8(255, 255, 255, 18.0 / 255.0),
            Self::Light => Color::from_rgba8(0, 0, 0, 15.0 / 255.0),
        }
    }
}

/// The font size tier for primary body text.
///
/// Provides four clean tiers:
/// - [`FontSizeTier::ExtraSmall`]: 18 px (compact tier, app-launcher label scale)
/// - [`FontSizeTier::Small`]: 20 px
/// - [`FontSizeTier::Standard`]: 24 px
/// - [`FontSizeTier::Large`]: 30 px
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FontSizeTier {
    /// Extra small scale (base: 18 px).
    ExtraSmall,
    /// Small scale (base: 20 px).
    Small,
    /// Default standard scale (base: 24 px).
    #[default]
    Standard,
    /// Large scale (base: 30 px).
    Large,
}

impl FontSizeTier {
    pub const COMPACT: Self = Self::ExtraSmall;

    /// The four font size tier values in pixels: [18.0, 20.0, 24.0, 30.0].
    pub const SIZES: [f32; 4] = [18.0, 20.0, 24.0, 30.0];

    /// All four font size tiers in ascending order: [ExtraSmall, Small, Standard, Large].
    pub const TIERS: [FontSizeTier; 4] = [
        FontSizeTier::ExtraSmall,
        FontSizeTier::Small,
        FontSizeTier::Standard,
        FontSizeTier::Large,
    ];

    /// Returns the four font size tier values in pixels: [18.0, 20.0, 24.0, 30.0].
    #[inline]
    pub const fn sizes() -> [f32; 4] {
        Self::SIZES
    }

    /// Cycles to the next font size tier (ExtraSmall -> Small -> Standard -> Large -> ExtraSmall).
    #[inline]
    pub fn cycle(self) -> Self {
        match self {
            Self::ExtraSmall => Self::Small,
            Self::Small => Self::Standard,
            Self::Standard => Self::Large,
            Self::Large => Self::ExtraSmall,
        }
    }

    /// Returns the next font size tier.
    #[inline]
    pub fn other(self) -> Self {
        self.cycle()
    }

    /// Human-readable name in `language`.
    pub fn name(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::ExtraSmall, Language::Chinese) => "紧凑 (18px)",
            (Self::ExtraSmall, Language::English) => "Compact (18px)",
            (Self::Small, Language::Chinese) => "小号 (20px)",
            (Self::Small, Language::English) => "Small (20px)",
            (Self::Standard, Language::Chinese) => "标准 (24px)",
            (Self::Standard, Language::English) => "Standard (24px)",
            (Self::Large, Language::Chinese) => "大号 (30px)",
            (Self::Large, Language::English) => "Large (30px)",
        }
    }

    /// The base font size in pixels (18.0 for ExtraSmall, 20.0 for Small, 24.0 for Standard, 30.0 for Large).
    #[inline]
    pub const fn base_size(self) -> f32 {
        match self {
            Self::ExtraSmall => 18.0,
            Self::Small => 20.0,
            Self::Standard => 24.0,
            Self::Large => 30.0,
        }
    }

    /// Label font size for application grid tiles and lists.
    #[inline]
    pub const fn label_size(self) -> f32 {
        self.base_size()
    }
}

/// Inactivity timeout before the screen automatically sleeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum SleepTimeout {
    /// Screen never turns off automatically. Default for Pomelo OS.
    #[default]
    Never,
    /// 15 seconds.
    FifteenSeconds,
    /// 30 seconds.
    ThirtySeconds,
    /// 1 minute.
    OneMinute,
    /// 5 minutes.
    FiveMinutes,
}

impl SleepTimeout {
    /// All available sleep timeout options.
    pub const OPTIONS: [Self; 5] = [
        Self::Never,
        Self::FifteenSeconds,
        Self::ThirtySeconds,
        Self::OneMinute,
        Self::FiveMinutes,
    ];

    /// Cycles to the next sleep timeout option (Never -> 15s -> 30s -> 1m -> 5m -> Never).
    #[inline]
    pub fn cycle(self) -> Self {
        match self {
            Self::Never => Self::FifteenSeconds,
            Self::FifteenSeconds => Self::ThirtySeconds,
            Self::ThirtySeconds => Self::OneMinute,
            Self::OneMinute => Self::FiveMinutes,
            Self::FiveMinutes => Self::Never,
        }
    }

    /// Duration represented by this option, or `None` if never sleeps.
    #[inline]
    pub fn as_duration(self) -> Option<Duration> {
        match self {
            Self::Never => None,
            Self::FifteenSeconds => Some(Duration::from_secs(15)),
            Self::ThirtySeconds => Some(Duration::from_secs(30)),
            Self::OneMinute => Some(Duration::from_secs(60)),
            Self::FiveMinutes => Some(Duration::from_secs(300)),
        }
    }

    /// Duration in seconds represented by this option, or `None` if never sleeps.
    #[inline]
    pub fn as_secs(self) -> Option<u32> {
        match self {
            Self::Never => None,
            Self::FifteenSeconds => Some(15),
            Self::ThirtySeconds => Some(30),
            Self::OneMinute => Some(60),
            Self::FiveMinutes => Some(300),
        }
    }

    /// Human-readable name in `language`.
    pub fn name(self, language: Language) -> &'static str {
        match (self, language) {
            (Self::Never, Language::Chinese) => "从不",
            (Self::Never, Language::English) => "Never",
            (Self::FifteenSeconds, Language::Chinese) => "15 秒",
            (Self::FifteenSeconds, Language::English) => "15 seconds",
            (Self::ThirtySeconds, Language::Chinese) => "30 秒",
            (Self::ThirtySeconds, Language::English) => "30 seconds",
            (Self::OneMinute, Language::Chinese) => "1 分钟",
            (Self::OneMinute, Language::English) => "1 minute",
            (Self::FiveMinutes, Language::Chinese) => "5 分钟",
            (Self::FiveMinutes, Language::English) => "5 minutes",
        }
    }
}

/// Default internal AppData directory on hardware.
pub const BOARD_APP_DATA: &str = "/internal/AppData";
/// Subdirectory where system settings are stored.
pub const PREFERENCES_DIR: &str = "SETTINGS";
/// Filename for system preferences.
pub const PREFERENCES_FILE: &str = "preferences.toml";

/// Resolves the storage root for system preferences.
pub fn resolve_app_data_root() -> PathBuf {
    if Path::new("/internal/AppData").exists() {
        PathBuf::from("/internal/AppData")
    } else if Path::new("./internal/AppData").exists() {
        PathBuf::from("./internal/AppData")
    } else if Path::new("/internal").exists() {
        PathBuf::from("/internal/AppData")
    } else if Path::new("./internal").exists() {
        PathBuf::from("./internal/AppData")
    } else {
        PathBuf::from(BOARD_APP_DATA)
    }
}

/// Aggregated system preferences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SystemPreferences {
    /// Active interface language.
    pub language: Language,
    /// Active theme appearance.
    pub theme: ThemeMode,
    /// Active typography scale.
    pub font_tier: FontSizeTier,
    /// Screen auto-sleep inactivity timeout. Defaults to never sleeping.
    #[serde(default)]
    pub sleep_timeout: SleepTimeout,
}

impl SystemPreferences {
    /// Creates a new preference set with default sleep timeout (Never).
    pub const fn new(language: Language, theme: ThemeMode, font_tier: FontSizeTier) -> Self {
        Self {
            language,
            theme,
            font_tier,
            sleep_timeout: SleepTimeout::Never,
        }
    }

    /// Sets the sleep timeout.
    pub const fn with_sleep_timeout(mut self, sleep_timeout: SleepTimeout) -> Self {
        self.sleep_timeout = sleep_timeout;
        self
    }

    /// Returns the active base font size in pixels.
    #[inline]
    pub const fn font_size(&self) -> f32 {
        self.font_tier.base_size()
    }

    /// The four available system font size tiers: [18.0, 20.0, 24.0, 30.0].
    #[inline]
    pub const fn font_sizes() -> [f32; 4] {
        FontSizeTier::SIZES
    }

    /// Path to preferences file under `root`.
    pub fn file_path(root: impl AsRef<Path>) -> PathBuf {
        root.as_ref().join(PREFERENCES_DIR).join(PREFERENCES_FILE)
    }

    /// Loads preferences from a specified root directory.
    pub fn load_from(root: impl AsRef<Path>) -> io::Result<Option<Self>> {
        let p = Self::file_path(root);
        let text = match fs::read_to_string(&p) {
            Ok(t) => t,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(e) => return Err(e),
        };
        toml::from_str(&text)
            .map(Some)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    /// Saves preferences to a specified root directory.
    pub fn save_to(&self, root: impl AsRef<Path>) -> io::Result<()> {
        let p = Self::file_path(root);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)?;
        }
        let text = toml::to_string(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(p, text)
    }

    /// Loads preferences from active persistent storage root.
    /// Falls back to default preferences if file is not found or corrupted.
    pub fn load() -> Self {
        Self::load_from(resolve_app_data_root())
            .ok()
            .flatten()
            .unwrap_or_default()
    }

    /// Saves preferences to active persistent storage root.
    pub fn save(&self) -> io::Result<()> {
        self.save_to(resolve_app_data_root())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_toggles() {
        assert_eq!(Language::Chinese.other(), Language::English);
        assert_eq!(Language::English.other(), Language::Chinese);
    }

    #[test]
    fn theme_toggles() {
        assert_eq!(ThemeMode::Dark.other(), ThemeMode::Light);
        assert_eq!(ThemeMode::Light.other(), ThemeMode::Dark);
    }

    #[test]
    fn font_tier_toggles() {
        assert_eq!(FontSizeTier::ExtraSmall.base_size(), 18.0);
        assert_eq!(FontSizeTier::Small.base_size(), 20.0);
        assert_eq!(FontSizeTier::Standard.base_size(), 24.0);
        assert_eq!(FontSizeTier::Large.base_size(), 30.0);
        assert_eq!(FontSizeTier::ExtraSmall.cycle(), FontSizeTier::Small);
        assert_eq!(FontSizeTier::Small.cycle(), FontSizeTier::Standard);
        assert_eq!(FontSizeTier::Standard.cycle(), FontSizeTier::Large);
        assert_eq!(FontSizeTier::Large.cycle(), FontSizeTier::ExtraSmall);
    }

    #[test]
    fn sleep_timeout_cycle_and_duration() {
        assert_eq!(SleepTimeout::default(), SleepTimeout::Never);
        assert_eq!(SleepTimeout::Never.as_duration(), None);
        assert_eq!(SleepTimeout::Never.as_secs(), None);

        assert_eq!(SleepTimeout::Never.cycle(), SleepTimeout::FifteenSeconds);
        assert_eq!(SleepTimeout::FifteenSeconds.as_duration(), Some(Duration::from_secs(15)));
        assert_eq!(SleepTimeout::FifteenSeconds.as_secs(), Some(15));

        assert_eq!(SleepTimeout::FifteenSeconds.cycle(), SleepTimeout::ThirtySeconds);
        assert_eq!(SleepTimeout::ThirtySeconds.as_duration(), Some(Duration::from_secs(30)));
        assert_eq!(SleepTimeout::ThirtySeconds.as_secs(), Some(30));

        assert_eq!(SleepTimeout::ThirtySeconds.cycle(), SleepTimeout::OneMinute);
        assert_eq!(SleepTimeout::OneMinute.as_duration(), Some(Duration::from_secs(60)));
        assert_eq!(SleepTimeout::OneMinute.as_secs(), Some(60));

        assert_eq!(SleepTimeout::OneMinute.cycle(), SleepTimeout::FiveMinutes);
        assert_eq!(SleepTimeout::FiveMinutes.as_duration(), Some(Duration::from_secs(300)));
        assert_eq!(SleepTimeout::FiveMinutes.as_secs(), Some(300));

        assert_eq!(SleepTimeout::FiveMinutes.cycle(), SleepTimeout::Never);

        assert_eq!(SleepTimeout::Never.name(Language::Chinese), "从不");
        assert_eq!(SleepTimeout::Never.name(Language::English), "Never");
        assert_eq!(SleepTimeout::FifteenSeconds.name(Language::Chinese), "15 秒");
        assert_eq!(SleepTimeout::FifteenSeconds.name(Language::English), "15 seconds");
        assert_eq!(SleepTimeout::ThirtySeconds.name(Language::Chinese), "30 秒");
        assert_eq!(SleepTimeout::ThirtySeconds.name(Language::English), "30 seconds");
        assert_eq!(SleepTimeout::OneMinute.name(Language::Chinese), "1 分钟");
        assert_eq!(SleepTimeout::OneMinute.name(Language::English), "1 minute");
        assert_eq!(SleepTimeout::FiveMinutes.name(Language::Chinese), "5 分钟");
        assert_eq!(SleepTimeout::FiveMinutes.name(Language::English), "5 minutes");
    }

    #[test]
    fn preferences_roundtrip_persistence() {
        let temp_dir = std::env::temp_dir().join(format!("pomelo_test_prefs_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);

        let initial = SystemPreferences::default();
        assert_eq!(initial.sleep_timeout, SleepTimeout::Never);

        let modified = SystemPreferences {
            language: Language::English,
            theme: ThemeMode::Light,
            font_tier: FontSizeTier::Large,
            sleep_timeout: SleepTimeout::ThirtySeconds,
        };

        modified.save_to(&temp_dir).expect("save should succeed");

        let loaded = SystemPreferences::load_from(&temp_dir)
            .expect("load should succeed")
            .expect("file should exist");

        assert_eq!(loaded, modified);
        assert_eq!(loaded.sleep_timeout, SleepTimeout::ThirtySeconds);

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
