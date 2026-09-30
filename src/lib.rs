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

pub mod preferences;
pub mod touch_keyboard;

pub use pomelo_material_symbols;
pub use preferences::{FontSizeTier, Language, SystemPreferences, ThemeMode};
