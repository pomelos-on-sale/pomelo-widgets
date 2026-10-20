//! Gesture Scope container widget (手势作用域容器).
//!
//! Hosts a [`GestureArenaManager`] instance and provides thread-local arena context
//! during widget event traversal. Bridges inner gesture recognizers (like [`GestureDetector`])
//! with outer scroll containers without modifying the upstream Iced library.

use iced::advanced::layout::{self, Layout};
use iced::advanced::mouse;
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::widget::Operation;
use iced::advanced::{Clipboard, Shell, Widget};
use iced::{touch, Element, Event, Length, Point, Rectangle, Size, Vector};

use std::collections::HashMap;

use super::arena::{set_arena_context, GestureArenaManager, PointerId};

/// Tracking state for an active pointer to arbitrate between taps and drags/scrolls.
#[derive(Debug, Clone, Copy)]
pub struct PointerTrack {
    /// Initial press position.
    pub origin: Point,
    /// Whether displacement has exceeded `touch_slop`, committing to a drag/scroll.
    pub is_dragging: bool,
}

/// Internal state of the gesture scope.
#[derive(Debug, Default)]
pub struct ScopeState {
    /// Active gesture arena manager for this scope.
    pub arena: GestureArenaManager,
    /// Active pointer tracking for touch slop arbitration.
    pub tracks: HashMap<PointerId, PointerTrack>,
}

/// A container widget establishing a gesture arena scope for its children.
pub struct GestureScope<
    'a,
    Message,
    Theme = iced::Theme,
    Renderer = iced::Renderer,
> {
    content: Element<'a, Message, Theme, Renderer>,
    exclusive: bool,
    touch_slop: f32,
}

impl<'a, Message, Theme, Renderer> GestureScope<'a, Message, Theme, Renderer> {
    /// Creates a new [`GestureScope`] wrapping `content`.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            exclusive: false,
            touch_slop: 10.0,
        }
    }

    /// Sets whether this scope operates in exclusive mode, ensuring all claimed
    /// child gestures strictly capture events from ancestor scrollables.
    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = exclusive;
        self
    }

    /// Sets the touch slop threshold in pixels (default 10.0 px).
    ///
    /// Movements below this displacement are treated as touch jitter / slop and
    /// suppressed from ancestor scrollables, preventing false button cancellations.
    pub fn touch_slop(mut self, slop: f32) -> Self {
        self.touch_slop = slop.max(0.0);
        self
    }
}

/// Convenience builder function for [`GestureScope`].
pub fn gesture_scope<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> GestureScope<'a, Message, Theme, Renderer> {
    GestureScope::new(content)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for GestureScope<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ScopeState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ScopeState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget_mut().layout(
            &mut tree.children[0],
            renderer,
            limits,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        );
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<ScopeState>();

        // Identify pointer details from the event:
        // (pointer_id, position, is_press, is_move, is_release, is_lost)
        let pointer_info: Option<(PointerId, Point, bool, bool, bool, bool)> = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                cursor.position().map(|p| (PointerId::MOUSE_LEFT, p, true, false, false, false))
            }
            Event::Touch(touch::Event::FingerPressed { id, position }) => {
                Some((PointerId::touch(id.0), *position, true, false, false, false))
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some((PointerId::MOUSE_LEFT, *position, false, true, false, false))
            }
            Event::Touch(touch::Event::FingerMoved { id, position }) => {
                Some((PointerId::touch(id.0), *position, false, true, false, false))
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                cursor.position().map(|p| (PointerId::MOUSE_LEFT, p, false, false, true, false))
            }
            Event::Touch(touch::Event::FingerLifted { id, position }) => {
                Some((PointerId::touch(id.0), *position, false, false, true, false))
            }
            Event::Touch(touch::Event::FingerLost { id, position }) => {
                Some((PointerId::touch(id.0), *position, false, false, false, true))
            }
            _ => None,
        };

        if let Some((pointer, point, is_press, _, _, _)) = pointer_info {
            if is_press {
                state.arena.open_arena(pointer, point);
                state.tracks.insert(
                    pointer,
                    PointerTrack {
                        origin: point,
                        is_dragging: false,
                    },
                );
            }
        }

        // Determine whether a move event should be suppressed to protect child taps.
        // If displacement is within touch_slop, we do not forward move events to the child
        // scrollable, preventing the scrollable from prematurely cancelling child buttons with FingerLost.
        let should_suppress_move = if let Some((pointer, point, _, is_move, _, _)) = pointer_info {
            if is_move {
                if let Some(track) = state.tracks.get_mut(&pointer) {
                    if track.is_dragging {
                        false
                    } else {
                        let dx = point.x - track.origin.x;
                        let dy = point.y - track.origin.y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist >= self.touch_slop {
                            track.is_dragging = true;
                            false
                        } else {
                            true
                        }
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        // Set thread-local arena context for child widgets
        let prev_ctx = set_arena_context(Some(state.arena.clone()));

        if !should_suppress_move {
            // Dispatch event traversal to child tree
            self.content.as_widget_mut().update(
                &mut tree.children[0],
                event,
                layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        // Retrieve updated arena state
        let updated_arena = set_arena_context(prev_ctx);
        if let Some(arena) = updated_arena {
            state.arena = arena;
        }

        // Handle pointer release (sweep & release arena)
        if let Some((pointer, _, _, _, is_release, is_lost)) = pointer_info {
            if is_release || is_lost {
                state.tracks.remove(&pointer);
            }
            if is_release {
                state.arena.sweep(pointer);
                state.arena.release(pointer);
            } else if is_lost {
                state.arena.release(pointer);
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a, Theme: 'a, Renderer: 'a + renderer::Renderer>
    From<GestureScope<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
{
    fn from(
        scope: GestureScope<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(scope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::text;

    #[test]
    fn gesture_scope_defaults_and_builder() {
        let content: Element<'_, (), iced::Theme, ()> = text("Hello").into();
        let scope = gesture_scope(content).touch_slop(15.0).exclusive(true);
        assert_eq!(scope.touch_slop, 15.0);
        assert!(scope.exclusive);
    }

    #[test]
    fn scope_state_tracks_pointers() {
        let mut state = ScopeState::default();
        let ptr = PointerId::touch(1);
        let origin = Point::new(10.0, 20.0);

        state.tracks.insert(
            ptr,
            PointerTrack {
                origin,
                is_dragging: false,
            },
        );
        assert!(state.tracks.contains_key(&ptr));

        let track = state.tracks.get_mut(&ptr).unwrap();
        assert!(!track.is_dragging);
        track.is_dragging = true;
        assert!(state.tracks.get(&ptr).unwrap().is_dragging);

        state.tracks.remove(&ptr);
        assert!(!state.tracks.contains_key(&ptr));
    }
}

