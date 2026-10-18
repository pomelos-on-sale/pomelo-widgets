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

pub mod sub_page;
pub use sub_page as action_sheet;
pub mod animation;
pub mod app;
pub mod button;
pub mod dialog;
pub mod gesture;
pub mod item;
pub mod opaque;
pub mod page;
pub mod pager;
pub mod preferences;
pub mod section;
pub mod toast;
pub mod toggle;
pub mod touch_keyboard;
pub mod usage_card;

pub use button::{Button, ButtonShape, ButtonSize, ButtonVariant};
pub use page::{
    page, page_with_modal, page_with_sheet, page_with_sub_page, Page, DEFAULT_PAGE_BODY_ID,
};
pub use usage_card::{usage_bar, UsageCard, BAR_GAP, BAR_HEIGHT, BAR_RADIUS, USAGE_PADDING};

pub use sub_page::{
    capsule_icon_button, ActionItem, ActionSheet, ActionStyle, SubPage, ACTION_SHEET_HEIGHT,
    ACTION_SHEET_HEIGHT_RATIO, BODY_MESSAGE_FONT, BODY_TITLE_FONT, BODY_TITLE_ICON,
    CAPSULE_BUTTON_GLYPH, CAPSULE_BUTTON_HEIGHT, CAPSULE_BUTTON_WIDTH, SUB_PAGE_HEIGHT,
    SUB_PAGE_HEIGHT_RATIO, TOP_BAR_ICON, TOP_BAR_TITLE_FONT,
};
pub use animation::{AnimationController, Curve};
pub use app::{interval_stream, AppIcon, AppMeta, AppPreferences, BitmapIcon};
pub use item::{
    card_separator, detail_row, item_separator, Item, ItemBadge, ItemGroup, NavigationAction,
    OpenPage, CARD_PADDING, DETAIL_FONT, ITEM_PADDING,
};
pub use section::{
    card_surface, section_surface, Section, CARD_BORDER_WIDTH, CARD_RADIUS, SECTION_BORDER_WIDTH,
    SECTION_RADIUS,
};
pub use toggle::{
    toggle_switch, KNOB_RADIUS, KNOB_SIZE, TOGGLE_H, TOGGLE_INSET, TOGGLE_RADIUS, TOGGLE_W,
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
pub use preferences::{
    FontSizeTier, FontTier, Language, ScreenOffTimeout, SleepTimeout, SystemPreferences, ThemeMode,
};
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
