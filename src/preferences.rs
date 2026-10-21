//! System-wide preferences and theme tokens shared across Pomelo OS applications.
//!
//! Provides the core types for:
//! - [`Language`]: interface language (Chinese or English)
//! - [`ThemeMode`]: display appearance (AMOLED Dark or Light)
//! - [`FontSizeTier`]: typography scale with three tiers (20px, 24px, 30px)
//! - [`SystemPreferences`]: aggregated system preference state

use std::io;
use std::time::Duration;

use iced::Color;
use serde::{Deserialize, Serialize};
use pomelo_hal::PersistentConfig;

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
    /// All available language options.
    pub const ALL: [Self; 2] = [Self::Chinese, Self::English];
    /// Alias for [`ALL`].
    pub const OPTIONS: [Self; 2] = Self::ALL;

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

    /// Pressed card row / tile background color.
    pub fn card_pressed(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(44, 44, 46),
            Self::Light => Color::from_rgb8(229, 229, 234),
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

    /// System accent / action color (iOS system blue).
    pub fn accent(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(10, 132, 255),
            Self::Light => Color::from_rgb8(0, 122, 255),
        }
    }

    /// System danger / destructive action color (iOS system red).
    pub fn danger(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(255, 69, 58),
            Self::Light => Color::from_rgb8(255, 59, 48),
        }
    }

    /// System success / positive state color (iOS system green).
    pub fn success(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(48, 209, 88),
            Self::Light => Color::from_rgb8(52, 199, 89),
        }
    }

    /// System warning / attention color (iOS system orange).
    pub fn warning(self) -> Color {
        match self {
            Self::Dark => Color::from_rgb8(255, 159, 10),
            Self::Light => Color::from_rgb8(255, 149, 0),
        }
    }
}

/// The font size tier for primary body text.
///
/// Provides three clean tiers:
/// - [`FontSizeTier::Small`]: 20 px
/// - [`FontSizeTier::Standard`]: 24 px
/// - [`FontSizeTier::Large`]: 30 px
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum FontSizeTier {
    /// Small scale (base: 20 px).
    Small,
    /// Default standard scale (base: 24 px).
    #[default]
    Standard,
    /// Large scale (base: 30 px).
    Large,
}

/// Alias for [`FontSizeTier`].
pub type FontTier = FontSizeTier;

impl FontSizeTier {
    /// The three font size tier values in pixels: [20.0, 24.0, 30.0].
    pub const SIZES: [f32; 3] = [20.0, 24.0, 30.0];

    /// All three font size tiers in ascending order: [Small, Standard, Large].
    pub const TIERS: [FontSizeTier; 3] = [
        FontSizeTier::Small,
        FontSizeTier::Standard,
        FontSizeTier::Large,
    ];

    /// Returns the three font size tier values in pixels: [20.0, 24.0, 30.0].
    #[inline]
    pub const fn sizes() -> [f32; 3] {
        Self::SIZES
    }

    /// Cycles to the next font size tier (Small -> Standard -> Large -> Small).
    #[inline]
    pub fn cycle(self) -> Self {
        match self {
            Self::Small => Self::Standard,
            Self::Standard => Self::Large,
            Self::Large => Self::Small,
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
            (Self::Small, Language::Chinese) => "小号 (20px)",
            (Self::Small, Language::English) => "Small (20px)",
            (Self::Standard, Language::Chinese) => "标准 (24px)",
            (Self::Standard, Language::English) => "Standard (24px)",
            (Self::Large, Language::Chinese) => "大号 (30px)",
            (Self::Large, Language::English) => "Large (30px)",
        }
    }

    /// The base font size in pixels (20.0 for Small, 24.0 for Standard, 30.0 for Large).
    #[inline]
    pub const fn base_size(self) -> f32 {
        match self {
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

/// Inactivity timeout before the screen automatically turns off (Screen Off).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ScreenOffTimeout {
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

impl ScreenOffTimeout {
    /// All available screen-off timeout options.
    pub const OPTIONS: [Self; 5] = [
        Self::Never,
        Self::FifteenSeconds,
        Self::ThirtySeconds,
        Self::OneMinute,
        Self::FiveMinutes,
    ];

    /// Cycles to the next screen-off timeout option (Never -> 15s -> 30s -> 1m -> 5m -> Never).
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

    /// Duration represented by this option, or `None` if never turns off.
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

    /// Duration in seconds represented by this option, or `None` if never turns off.
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


/// Namespace under KV storage where system preferences are stored.
pub const PREFERENCES_NAMESPACE: &str = "SETTINGS";

/// Returns whether the current process is running as a test harness binary.
#[inline]
pub fn is_test_runner() -> bool {
    pomelo_hal::storage::is_test_runner()
}

const fn default_true() -> bool {
    true
}

const fn default_volume() -> u8 {
    75
}

const fn default_brightness() -> u8 {
    100
}

/// Aggregated system preferences.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SystemPreferences {
    /// Active interface language.
    pub language: Language,
    /// Active theme appearance.
    pub theme: ThemeMode,
    /// Active typography scale.
    pub font_tier: FontSizeTier,
    /// Inactivity timeout before screen turns off (Screen Off). Defaults to never turning off.
    #[serde(default)]
    pub screen_off_timeout: ScreenOffTimeout,
    /// Whether to stay awake while connected to USB. Defaults to true.
    #[serde(default = "default_true")]
    pub stay_awake_on_usb: bool,
    /// Audio volume level (0..=100). Defaults to 75.
    #[serde(default = "default_volume")]
    pub volume: u8,
    /// Display brightness level (10..=100). Defaults to 100.
    #[serde(default = "default_brightness")]
    pub brightness: u8,
}

impl Default for SystemPreferences {
    fn default() -> Self {
        Self {
            language: Language::Chinese,
            theme: ThemeMode::Dark,
            font_tier: FontSizeTier::Standard,
            screen_off_timeout: ScreenOffTimeout::Never,
            stay_awake_on_usb: true,
            volume: 75,
            brightness: 100,
        }
    }
}

impl SystemPreferences {
    /// Default audio volume level (75%).
    pub const DEFAULT_VOLUME: u8 = 75;
    /// Default display brightness level (100%).
    pub const DEFAULT_BRIGHTNESS: u8 = 100;

    /// Returns the default volume level.
    #[inline]
    pub const fn default_volume() -> u8 {
        Self::DEFAULT_VOLUME
    }

    /// Returns the default brightness level.
    #[inline]
    pub const fn default_brightness() -> u8 {
        Self::DEFAULT_BRIGHTNESS
    }

    /// Creates a new preference set with default timeouts (Never), stay awake on USB enabled, 75% volume, and 100% brightness.
    pub const fn new(language: Language, theme: ThemeMode, font_tier: FontSizeTier) -> Self {
        Self {
            language,
            theme,
            font_tier,
            screen_off_timeout: ScreenOffTimeout::Never,
            stay_awake_on_usb: true,
            volume: 75,
            brightness: 100,
        }
    }

    /// Sets the screen-off timeout.
    pub const fn with_screen_off_timeout(mut self, screen_off_timeout: ScreenOffTimeout) -> Self {
        self.screen_off_timeout = screen_off_timeout;
        self
    }

    /// Sets whether to stay awake while connected to USB.
    pub const fn with_stay_awake_on_usb(mut self, stay_awake_on_usb: bool) -> Self {
        self.stay_awake_on_usb = stay_awake_on_usb;
        self
    }

    /// Sets the audio volume level (0..=100).
    pub const fn with_volume(mut self, volume: u8) -> Self {
        self.volume = volume;
        self
    }

    /// Sets the display brightness level (10..=100).
    pub const fn with_brightness(mut self, brightness: u8) -> Self {
        self.brightness = if brightness < 10 {
            10
        } else if brightness > 100 {
            100
        } else {
            brightness
        };
        self
    }

    /// Returns the active base font size in pixels.
    #[inline]
    pub const fn font_size(&self) -> f32 {
        self.font_tier.base_size()
    }

    /// The three available system font size tiers: [20.0, 24.0, 30.0].
    #[inline]
    pub const fn font_sizes() -> [f32; 3] {
        FontSizeTier::SIZES
    }

    /// Loads preferences from active persistent storage (NVS KV on hardware, or mock KV on desktop).
    /// Falls back to default preferences if not found or corrupted.
    pub fn load() -> Self {
        if is_test_runner() && std::env::var("POMELO_STORAGE_DIR").is_err() {
            return Self::default();
        }
        <Self as PersistentConfig>::load()
    }

    /// Saves preferences to active persistent storage (NVS KV on hardware, or mock KV on desktop).
    pub fn save(&self) -> io::Result<()> {
        if is_test_runner() && std::env::var("POMELO_STORAGE_DIR").is_err() {
            return Ok(());
        }
        <Self as PersistentConfig>::save(self)
    }
}

impl PersistentConfig for SystemPreferences {
    const APP_ID: &'static str = PREFERENCES_NAMESPACE;
    const KEY: &'static str = "preferences";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

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
        assert_eq!(FontSizeTier::Small.base_size(), 20.0);
        assert_eq!(FontSizeTier::Standard.base_size(), 24.0);
        assert_eq!(FontSizeTier::Large.base_size(), 30.0);
        assert_eq!(FontSizeTier::Small.cycle(), FontSizeTier::Standard);
        assert_eq!(FontSizeTier::Standard.cycle(), FontSizeTier::Large);
        assert_eq!(FontSizeTier::Large.cycle(), FontSizeTier::Small);
    }

    #[test]
    fn screen_off_timeout_cycle_and_duration() {
        assert_eq!(ScreenOffTimeout::default(), ScreenOffTimeout::Never);
        assert_eq!(ScreenOffTimeout::Never.as_duration(), None);
        assert_eq!(ScreenOffTimeout::Never.as_secs(), None);

        assert_eq!(ScreenOffTimeout::Never.cycle(), ScreenOffTimeout::FifteenSeconds);
        assert_eq!(ScreenOffTimeout::FifteenSeconds.as_duration(), Some(Duration::from_secs(15)));
        assert_eq!(ScreenOffTimeout::FifteenSeconds.as_secs(), Some(15));

        assert_eq!(ScreenOffTimeout::FifteenSeconds.cycle(), ScreenOffTimeout::ThirtySeconds);
        assert_eq!(ScreenOffTimeout::ThirtySeconds.as_duration(), Some(Duration::from_secs(30)));
        assert_eq!(ScreenOffTimeout::ThirtySeconds.as_secs(), Some(30));

        assert_eq!(ScreenOffTimeout::ThirtySeconds.cycle(), ScreenOffTimeout::OneMinute);
        assert_eq!(ScreenOffTimeout::OneMinute.as_duration(), Some(Duration::from_secs(60)));
        assert_eq!(ScreenOffTimeout::OneMinute.as_secs(), Some(60));

        assert_eq!(ScreenOffTimeout::OneMinute.cycle(), ScreenOffTimeout::FiveMinutes);
        assert_eq!(ScreenOffTimeout::FiveMinutes.as_duration(), Some(Duration::from_secs(300)));
        assert_eq!(ScreenOffTimeout::FiveMinutes.as_secs(), Some(300));

        assert_eq!(ScreenOffTimeout::FiveMinutes.cycle(), ScreenOffTimeout::Never);
    }

    #[test]
    fn preferences_roundtrip_persistence() {
        let temp_dir = std::env::temp_dir().join(format!("pomelo_test_prefs_{}", std::process::id()));
        let _ = fs::remove_dir_all(&temp_dir);
        std::env::set_var("POMELO_STORAGE_DIR", &temp_dir);

        let initial = SystemPreferences::load();
        assert_eq!(initial.screen_off_timeout, ScreenOffTimeout::Never);
        assert!(initial.stay_awake_on_usb);
        assert_eq!(initial.volume, 75);
        assert_eq!(initial.brightness, 100);

        let modified = SystemPreferences {
            language: Language::English,
            theme: ThemeMode::Light,
            font_tier: FontSizeTier::Large,
            screen_off_timeout: ScreenOffTimeout::ThirtySeconds,
            stay_awake_on_usb: false,
            volume: 60,
            brightness: 80,
        };

        modified.save().expect("save should succeed");

        let loaded = SystemPreferences::load();

        assert_eq!(loaded, modified);
        assert_eq!(loaded.screen_off_timeout, ScreenOffTimeout::ThirtySeconds);
        assert!(!loaded.stay_awake_on_usb);
        assert_eq!(loaded.volume, 60);
        assert_eq!(loaded.brightness, 80);

        std::env::remove_var("POMELO_STORAGE_DIR");
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
