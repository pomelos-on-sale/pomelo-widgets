//! Standard stacked page container and layout helpers conforming to Pomelo OS design language.

use iced::widget::scrollable::{Direction, Scrollbar};
use iced::widget::{scrollable, Column, Stack};
use iced::{Element, Length};

/// The default ID the page body's `scrollable` carries to maintain scroll offset across redraws.
pub const DEFAULT_PAGE_BODY_ID: &str = "pomelo-page-body";

/// A top-level page widget whose root is a stable [`Stack`].
///
/// Layer 0: The scrollable page content (`head` + `body`).
/// Additional layers (e.g. modals, dialogs, toasts, action sheets) can be pushed on top
/// without altering the widget tree root tag, preserving child scroll state.
pub struct Page<'a, Message> {
    stack: Stack<'a, Message>,
}

impl<'a, Message: 'a> Page<'a, Message> {
    /// Creates a new page whose base layer is a scrollable with `head` and `body`.
    pub fn new(
        head: impl Into<Element<'a, Message>>,
        body: impl Into<Element<'a, Message>>,
    ) -> Self {
        Self::with_id(head, body, DEFAULT_PAGE_BODY_ID)
    }

    /// Creates a new page with a custom scrollable ID.
    pub fn with_id(
        head: impl Into<Element<'a, Message>>,
        body: impl Into<Element<'a, Message>>,
        scrollable_id: impl Into<iced::widget::Id>,
    ) -> Self {
        let base = scrollable(
            Column::with_children(vec![head.into(), body.into()]).width(Length::Fill),
        )
        .id(scrollable_id)
        .direction(Direction::Vertical(Scrollbar::hidden()))
        .width(Length::Fill)
        .height(Length::Fill);

        Self {
            stack: Stack::with_children(vec![base.into()]),
        }
    }

    /// Pushes an overlay layer (e.g. modal, dialog, toast) on top of the page.
    pub fn push(mut self, layer: impl Into<Element<'a, Message>>) -> Self {
        self.stack = self.stack.push(layer.into());
        self
    }

    /// Pushes an optional overlay layer on top of the page.
    pub fn push_opt(mut self, layer: Option<impl Into<Element<'a, Message>>>) -> Self {
        if let Some(l) = layer {
            self.stack = self.stack.push(l.into());
        }
        self
    }

    /// Consumes the page builder and returns the root [`Element`].
    pub fn view(self) -> Element<'a, Message> {
        self.stack.width(Length::Fill).height(Length::Fill).into()
    }
}

impl<'a, Message: 'a> From<Page<'a, Message>> for Element<'a, Message> {
    fn from(page: Page<'a, Message>) -> Self {
        page.view()
    }
}

/// Convenience helper to create a basic stacked page.
pub fn page<'a, Message: 'a>(
    head: impl Into<Element<'a, Message>>,
    body: impl Into<Element<'a, Message>>,
) -> Page<'a, Message> {
    Page::new(head, body)
}

/// Convenience helper to create a stacked page with an optional modal dialog.
pub fn page_with_modal<'a, Message: 'a>(
    head: impl Into<Element<'a, Message>>,
    body: impl Into<Element<'a, Message>>,
    modal: Option<impl Into<Element<'a, Message>>>,
) -> Page<'a, Message> {
    Page::new(head, body).push_opt(modal)
}

/// Convenience helper to create a stacked page with an optional modal [`SubPage`].
pub fn page_with_sub_page<'a, Message: Clone + 'a>(
    head: impl Into<Element<'a, Message>>,
    body: impl Into<Element<'a, Message>>,
    sub_page: Option<crate::sub_page::SubPage<'a, Message>>,
) -> Page<'a, Message> {
    Page::new(head, body).push_opt(sub_page)
}

/// Backward-compatible alias for [`page_with_sub_page`].
pub fn page_with_sheet<'a, Message: Clone + 'a>(
    head: impl Into<Element<'a, Message>>,
    body: impl Into<Element<'a, Message>>,
    sheet: Option<crate::sub_page::SubPage<'a, Message>>,
) -> Page<'a, Message> {
    page_with_sub_page(head, body, sheet)
}

#[cfg(test)]
mod tests {
    use super::*;
    use iced::widget::{space, text};

    #[derive(Debug, Clone)]
    enum TestMsg {
        _Tick,
    }

    #[test]
    fn page_builds_without_panic() {
        let p = Page::<TestMsg>::new(text("head"), text("body"));
        let _elem: Element<'_, TestMsg> = p.view();
    }

    #[test]
    fn page_with_modal_and_sheet_builds() {
        let p = page_with_modal::<TestMsg>(
            space(),
            space(),
            Some(text("modal")),
        );
        let _elem: Element<'_, TestMsg> = p.view();
    }
}
