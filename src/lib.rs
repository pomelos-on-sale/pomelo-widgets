//! The widgets Pomelo OS's apps share, in one crate.
//!
//! Every widget two or more apps draw lives here, so that one table, one palette and one geometry
//! exist for all of them. Each widget is a module of this crate — today [`touch_keyboard`] — and an
//! app depends on the crate and names the module it wants:
//!
//! ```ignore
//! use pomelo_widgets::touch_keyboard::{band, KeyAction, KeyboardMode};
//! ```
//!
//! # Why a crate rather than a module in each app
//!
//! Two apps drawing the same widget from two copies of its geometry is how a password keyboard
//! drifts two pixels away from the terminal's: nobody reports that, everybody notices it.
//!
//! # What it depends on, and what it does not
//!
//! iced's facade, and nothing else: no platform layer, no renderer, no board — a widget builds
//! `Element`s and never draws them, so which renderer draws is the graph root's business. The
//! apps put the switch on their own manifests (`desktop = ["iced/tiny-skia"]`) and the board's
//! roots turn it off.
//!
//! # Icons are not here
//!
//! They were, and they moved to `pomelo-material-symbols`: a font and a table of `const`s, with no
//! widgets in it at all. An app that draws an icon depends on that crate as well — the launcher
//! does, and it has no business with a keyboard.

pub mod action_sheet;
pub mod animation;
pub mod app;
pub mod card;
pub mod dialog;
pub mod gesture;
pub mod pager;
pub mod preferences;
pub mod opaque;
pub mod toast;
pub mod touch_keyboard;

pub use action_sheet::{
    ActionItem, ActionSheet, ActionStyle, ACTION_SHEET_HEIGHT, ACTION_SHEET_HEIGHT_RATIO,
    BODY_MESSAGE_FONT, BODY_TITLE_FONT, BODY_TITLE_ICON, TOP_BAR_ICON, TOP_BAR_TITLE_FONT,
};
pub use animation::{AnimationController, Curve};
pub use app::{interval_stream, AppIcon, AppMeta, AppPreferences, BitmapIcon};
pub use card::{
    card_separator, card_surface, detail_row, CARD_BORDER_WIDTH, CARD_PADDING, CARD_RADIUS,
    DETAIL_FONT,
};
pub use dialog::{
    confirm_dialog, dialog_box, dialog_button, dialog_cancel_button, dialog_card,
    dialog_confirm_button, dialog_danger_button, modal_dialog, DialogButtonStyle,
};
pub use gesture::{
    drag_disambiguator, gesture_detector, DragDisambiguator, GestureDetector, PanEndDetails,
    PanStartDetails, PanUpdateDetails, SwipeDirection,
};
pub use opaque::opaque;
pub use pager::{pager, Pager};
pub use pomelo_material_symbols;
pub use preferences::{FontSizeTier, FontTier, Language, SleepTimeout, SystemPreferences, ThemeMode};
pub use toast::{
    toast_view, Toast, TOAST_BOTTOM_MARGIN, TOAST_DURATION, TOAST_FONT_SIZE, TOAST_MAX_WIDTH,
    TOAST_PADDING, TOAST_RADIUS,
};

/// Builds an icon text widget from [`pomelo_material_symbols::Icon`] styled with Material Symbols.
///
/// ```ignore
/// use pomelo_widgets::icon;
/// use pomelo_material_symbols::Icon;
///
/// icon(Icon::PLAY_ARROW).size(24)
/// ```
pub fn icon<'a>(icon: pomelo_material_symbols::Icon) -> iced::widget::Text<'a> {
    iced::widget::text(icon.glyph()).font(pomelo_material_symbols::font())
}
