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

use super::arena::{set_arena_context, GestureArenaManager, PointerId};

/// Internal state of the gesture scope.
#[derive(Debug, Default)]
pub struct ScopeState {
    /// Active gesture arena manager for this scope.
    pub arena: GestureArenaManager,
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
}

impl<'a, Message, Theme, Renderer> GestureScope<'a, Message, Theme, Renderer> {
    /// Creates a new [`GestureScope`] wrapping `content`.
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            content: content.into(),
            exclusive: false,
        }
    }

    /// Sets whether this scope operates in exclusive mode, ensuring all claimed
    /// child gestures strictly capture events from ancestor scrollables.
    pub fn exclusive(mut self, exclusive: bool) -> Self {
        self.exclusive = exclusive;
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

        // Identify pointer details from the event
        let pointer_info: Option<(PointerId, Point, bool, bool, bool)> = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                cursor.position().map(|p| (PointerId::MOUSE_LEFT, p, true, false, false))
            }
            Event::Touch(touch::Event::FingerPressed { id, position }) => {
                Some((PointerId::touch(id.0), *position, true, false, false))
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some((PointerId::MOUSE_LEFT, *position, false, false, false))
            }
            Event::Touch(touch::Event::FingerMoved { id, position }) => {
                Some((PointerId::touch(id.0), *position, false, false, false))
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                cursor.position().map(|p| (PointerId::MOUSE_LEFT, p, false, true, false))
            }
            Event::Touch(touch::Event::FingerLifted { id, position }) => {
                Some((PointerId::touch(id.0), *position, false, true, false))
            }
            Event::Touch(touch::Event::FingerLost { id, position }) => {
                Some((PointerId::touch(id.0), *position, false, false, true))
            }
            _ => None,
        };

        if let Some((pointer, point, is_press, _, _)) = pointer_info {
            if is_press {
                state.arena.open_arena(pointer, point);
            }
        }

        // Set thread-local arena context for child widgets
        let prev_ctx = set_arena_context(Some(state.arena.clone()));

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

        // Retrieve updated arena state
        let updated_arena = set_arena_context(prev_ctx);
        if let Some(arena) = updated_arena {
            state.arena = arena;
        }

        // Handle pointer release (sweep & release arena)
        if let Some((pointer, _, _, is_release, is_lost)) = pointer_info {
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
