use std::time::Instant;
use crate::animation::AnimationController;

/// Internal state of the pager.
#[derive(Debug, Default)]
pub(crate) struct State {
    pub(crate) pointer_down: bool,
    pub(crate) start_x: f32,
    pub(crate) drag_offset: f32,
    pub(crate) is_dragging: bool,
    pub(crate) has_dragged: bool,
    pub(crate) animator: AnimationController,
    pub(crate) settle_target_page: Option<usize>,
    pub(crate) last_x: f32,
    pub(crate) last_move_time: Option<Instant>,
    pub(crate) velocity_x: f32,
    pub(crate) last_page: Option<usize>,
}
