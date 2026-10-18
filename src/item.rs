//! Strongly-typed row items and item groups conforming to Pomelo OS design language.

use std::borrow::Cow;
use iced::widget::{button, container, rule, text, Column, Row, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding, Shadow};
use pomelo_material_symbols::Icon;

use crate::preferences::{FontSizeTier, ThemeMode};
use crate::toggle::toggle_switch;

/// Default font size for key-value detail rows (standard Small tier: 20px).
pub const DETAIL_FONT: f32 = FontSizeTier::Small.base_size();

/// Default vertical and horizontal padding for items.
pub const ITEM_PADDING: Padding = Padding {
    top: 12.0,
    bottom: 12.0,
    left: 16.0,
    right: 16.0,
};

/// Backward-compatible alias for [`ITEM_PADDING`].
pub const CARD_PADDING: Padding = ITEM_PADDING;

/// A standard horizontal separator rule for use between items.
pub fn item_separator<'a, Message: 'a>(theme: ThemeMode) -> iced::widget::Rule<'a> {
    rule::horizontal(1)
        .style(move |_| rule::Style {
            color: theme.separator(),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: false,
        })
}

/// Backward-compatible alias for [`item_separator`].
pub fn card_separator<'a, Message: 'a>(theme: ThemeMode) -> iced::widget::Rule<'a> {
    item_separator::<Message>(theme)
}

/// A standard key-value detail row inside an item group.
pub fn detail_row<'a, Message: 'a>(
    key: impl Into<Cow<'static, str>>,
    value: impl Into<Cow<'static, str>>,
    theme: ThemeMode,
) -> Element<'a, Message> {
    let key_str = key.into();
    let val_str = value.into();
    let row = Row::new()
        .width(Length::Fill)
        .align_y(Alignment::Center)
        .push(
            text(key_str)
                .size(DETAIL_FONT)
                .color(theme.text_primary()),
        )
        .push(Space::new().width(Length::Fill))
        .push(
            text(val_str)
                .size(DETAIL_FONT)
                .color(theme.text_secondary()),
        );

    container(row)
        .width(Length::Fill)
        .padding(ITEM_PADDING)
        .into()
}

/// Trait representing an explicit page navigation action.
///
/// Ensures at compile time that a navigation row ([`Item::nav`] / [`ItemGroup::nav`]) — which displays a
/// forward-pointing chevron icon (`>`) to the user — can strictly only be bound
/// to page navigation transitions, preventing in-place state mutations from being
/// accidentally disguised as page navigations.
pub trait NavigationAction<Message> {
    /// Converts this navigation target into the application's message.
    fn into_message(self) -> Message;
}

/// A strongly-typed token explicitly declaring that an action navigates to a new page.
///
/// Use this wrapper when you have an ad-hoc message that performs page navigation,
/// or implement [`NavigationAction`] directly on your application's route enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OpenPage<Message>(pub Message);

impl<Message> NavigationAction<Message> for OpenPage<Message> {
    fn into_message(self) -> Message {
        self.0
    }
}

/// An icon badge displayed at the leading side of an item row.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ItemBadge {
    pub icon: Icon,
    pub color: Color,
}

impl ItemBadge {
    pub const fn new(icon: Icon, color: Color) -> Self {
        Self { icon, color }
    }
}

fn render_badge<'a, Message: 'a>(badge: ItemBadge) -> Element<'a, Message> {
    const BADGE_SIDE: f32 = 28.0;
    const BADGE_GLYPH: f32 = 20.0;
    const BADGE_RADIUS: f32 = 6.0;

    container(
        text(badge.icon.glyph())
            .font(pomelo_material_symbols::font())
            .size(BADGE_GLYPH)
            .color(Color::WHITE),
    )
    .width(Length::Fixed(BADGE_SIDE))
    .height(Length::Fixed(BADGE_SIDE))
    .center_x(Length::Fixed(BADGE_SIDE))
    .center_y(Length::Fixed(BADGE_SIDE))
    .style(move |_| container::Style {
        background: Some(badge.color.into()),
        border: Border {
            radius: BADGE_RADIUS.into(),
            ..Border::default()
        },
        ..Default::default()
    })
    .into()
}

/// A strongly-typed row item conforming to Pomelo OS design language.
///
/// Unlike generic UI widgets, [`Item`] enforces structured semantics for list and card rows,
/// preventing arbitrary widgets from being accidentally inserted into an [`ItemGroup`].
#[derive(Debug, Clone, PartialEq)]
pub enum Item<Message> {
    /// Navigation row with label, optional right-side value, and trailing chevron (`>`).
    Nav {
        badge: Option<ItemBadge>,
        label: Cow<'static, str>,
        value: Option<Cow<'static, str>>,
        on_navigate: Message,
    },
    /// Option row with label and optional trailing checkmark (`Icon::CHECK`).
    Option {
        badge: Option<ItemBadge>,
        label: Cow<'static, str>,
        is_selected: bool,
        on_select: Message,
    },
    /// Switch row with label on the left and an iOS-style toggle switch on the right.
    Switch {
        badge: Option<ItemBadge>,
        label: Cow<'static, str>,
        on: bool,
        on_toggle: Message,
    },
    /// Read-only static key-value detail row (no click feedback, guaranteed immutable on click).
    KeyValue {
        badge: Option<ItemBadge>,
        label: Cow<'static, str>,
        value: Cow<'static, str>,
    },
    /// Centered or row action button (e.g. destructive red delete button or row action).
    Action {
        badge: Option<ItemBadge>,
        label: Cow<'static, str>,
        value: Option<Cow<'static, str>>,
        color: Option<Color>,
        on_press: Message,
    },
}

impl<Message: Clone> Item<Message> {
    /// Creates a strongly-typed navigation item.
    ///
    /// The action parameter is constrained to types implementing [`NavigationAction<Message>`]
    /// (such as [`OpenPage`] or a dedicated page route enum), guaranteeing at compile time
    /// that clicking this row performs a page transition rather than an in-place mutation.
    pub fn nav<Nav: NavigationAction<Message>>(
        label: impl Into<Cow<'static, str>>,
        value: Option<impl Into<Cow<'static, str>>>,
        action: Nav,
    ) -> Self {
        Self::Nav {
            badge: None,
            label: label.into(),
            value: value.map(|v| v.into()),
            on_navigate: action.into_message(),
        }
    }

    /// Creates a strongly-typed option selection item with an optional checkmark.
    pub fn option(
        label: impl Into<Cow<'static, str>>,
        is_selected: bool,
        on_select: Message,
    ) -> Self {
        Self::Option {
            badge: None,
            label: label.into(),
            is_selected,
            on_select,
        }
    }

    /// Creates a strongly-typed toggle switch item.
    pub fn switch(
        label: impl Into<Cow<'static, str>>,
        on: bool,
        on_toggle: Message,
    ) -> Self {
        Self::Switch {
            badge: None,
            label: label.into(),
            on,
            on_toggle,
        }
    }

    /// Creates a strongly-typed read-only key-value detail item.
    pub fn key_value(
        label: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self::KeyValue {
            badge: None,
            label: label.into(),
            value: value.into(),
        }
    }

    /// Creates a strongly-typed action item (e.g. destructive red delete button).
    pub fn action(
        label: impl Into<Cow<'static, str>>,
        color: Color,
        on_press: Message,
    ) -> Self {
        Self::Action {
            badge: None,
            label: label.into(),
            value: None,
            color: Some(color),
            on_press,
        }
    }

    /// Creates a strongly-typed button item with standard theme text color.
    pub fn button(
        label: impl Into<Cow<'static, str>>,
        on_press: Message,
    ) -> Self {
        Self::Action {
            badge: None,
            label: label.into(),
            value: None,
            color: None,
            on_press,
        }
    }

    /// Attaches an icon badge to the leading side of this item.
    pub fn with_badge(mut self, badge: ItemBadge) -> Self {
        match &mut self {
            Self::Nav { badge: b, .. }
            | Self::Option { badge: b, .. }
            | Self::Switch { badge: b, .. }
            | Self::KeyValue { badge: b, .. }
            | Self::Action { badge: b, .. } => *b = Some(badge),
        }
        self
    }

    /// Attaches or updates the secondary right-hand value text of this item.
    pub fn with_value(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        match &mut self {
            Self::Nav { value: v, .. } | Self::Action { value: v, .. } => *v = Some(val.into()),
            Self::KeyValue { value: v, .. } => *v = val.into(),
            _ => {}
        }
        self
    }

    /// Renders this item into an [`Element`].
    pub fn render<'a>(self, theme: ThemeMode) -> Element<'a, Message>
    where
        Message: 'a,
    {
        match self {
            Self::Nav {
                badge,
                label,
                value,
                on_navigate,
            } => {
                let mut left_items: Vec<Element<'a, Message>> = Vec::new();
                if let Some(b) = badge {
                    left_items.push(render_badge(b));
                    left_items.push(Space::new().width(Length::Fixed(12.0)).into());
                }
                left_items.push(
                    text(label)
                        .size(DETAIL_FONT)
                        .color(theme.text_primary())
                        .into(),
                );

                let mut right_items: Vec<Element<'a, Message>> = Vec::new();
                if let Some(val) = value {
                    right_items.push(
                        text(val)
                            .size(DETAIL_FONT)
                            .color(theme.text_secondary())
                            .into(),
                    );
                    right_items.push(Space::new().width(Length::Fixed(8.0)).into());
                }
                right_items.push(
                    crate::icon(Icon::KEYBOARD_ARROW_RIGHT)
                        .size(22.0)
                        .color(theme.text_secondary())
                        .into(),
                );

                let row_content = Row::new()
                    .width(Length::Fill)
                    .align_y(Alignment::Center)
                    .push(Row::with_children(left_items).align_y(Alignment::Center))
                    .push(Space::new().width(Length::Fill))
                    .push(Row::with_children(right_items).align_y(Alignment::Center));

                button(
                    container(row_content)
                        .width(Length::Fill)
                        .padding(ITEM_PADDING),
                )
                .width(Length::Fill)
                .padding(0)
                .style(move |_, status| {
                    let bg = match status {
                        button::Status::Pressed => theme.card_pressed(),
                        _ => Color::TRANSPARENT,
                    };
                    button::Style {
                        background: Some(bg.into()),
                        text_color: theme.text_primary(),
                        border: Border::default(),
                        shadow: Shadow::default(),
                        snap: false,
                    }
                })
                .on_press(on_navigate)
                .into()
            }
            Self::Option {
                badge,
                label,
                is_selected,
                on_select,
            } => {
                let mut left_items: Vec<Element<'a, Message>> = Vec::new();
                if let Some(b) = badge {
                    left_items.push(render_badge(b));
                    left_items.push(Space::new().width(Length::Fixed(12.0)).into());
                }
                left_items.push(
                    text(label)
                        .size(DETAIL_FONT)
                        .color(theme.text_primary())
                        .into(),
                );

                let mut right_items: Vec<Element<'a, Message>> = Vec::new();
                if is_selected {
                    right_items.push(
                        crate::icon(Icon::CHECK)
                            .size(24.0)
                            .color(theme.accent())
                            .into(),
                    );
                }

                let row_content = Row::new()
                    .width(Length::Fill)
                    .align_y(Alignment::Center)
                    .push(Row::with_children(left_items).align_y(Alignment::Center))
                    .push(Space::new().width(Length::Fill))
                    .push(Row::with_children(right_items).align_y(Alignment::Center));

                button(
                    container(row_content)
                        .width(Length::Fill)
                        .padding(ITEM_PADDING),
                )
                .width(Length::Fill)
                .padding(0)
                .style(move |_, status| {
                    let bg = match status {
                        button::Status::Pressed => theme.card_pressed(),
                        _ => Color::TRANSPARENT,
                    };
                    button::Style {
                        background: Some(bg.into()),
                        text_color: theme.text_primary(),
                        border: Border::default(),
                        shadow: Shadow::default(),
                        snap: false,
                    }
                })
                .on_press(on_select)
                .into()
            }
            Self::Switch {
                badge,
                label,
                on,
                on_toggle,
            } => {
                let mut left_items: Vec<Element<'a, Message>> = Vec::new();
                if let Some(b) = badge {
                    left_items.push(render_badge(b));
                    left_items.push(Space::new().width(Length::Fixed(12.0)).into());
                }
                left_items.push(
                    text(label)
                        .size(DETAIL_FONT)
                        .color(theme.text_primary())
                        .into(),
                );

                let row_content = Row::new()
                    .width(Length::Fill)
                    .align_y(Alignment::Center)
                    .push(Row::with_children(left_items).align_y(Alignment::Center))
                    .push(Space::new().width(Length::Fill))
                    .push(toggle_switch(on, on_toggle, theme));

                container(row_content)
                    .width(Length::Fill)
                    .padding(ITEM_PADDING)
                    .into()
            }
            Self::KeyValue {
                badge,
                label,
                value,
            } => {
                if let Some(b) = badge {
                    let mut left_items: Vec<Element<'a, Message>> = Vec::new();
                    left_items.push(render_badge(b));
                    left_items.push(Space::new().width(Length::Fixed(12.0)).into());
                    left_items.push(
                        text(label)
                            .size(DETAIL_FONT)
                            .color(theme.text_primary())
                            .into(),
                    );
                    let row = Row::new()
                        .width(Length::Fill)
                        .align_y(Alignment::Center)
                        .push(Row::with_children(left_items).align_y(Alignment::Center))
                        .push(Space::new().width(Length::Fill))
                        .push(
                            text(value)
                                .size(DETAIL_FONT)
                                .color(theme.text_secondary()),
                        );
                    container(row).width(Length::Fill).padding(ITEM_PADDING).into()
                } else {
                    detail_row(label, value, theme)
                }
            }
            Self::Action {
                badge,
                label,
                value,
                color,
                on_press,
            } => {
                let text_color = color.unwrap_or_else(|| theme.text_primary());
                let is_centered = badge.is_none() && value.is_none();

                if is_centered {
                    let row_content = container(
                        text(label)
                            .size(DETAIL_FONT)
                            .color(text_color),
                    )
                    .width(Length::Fill)
                    .align_x(Alignment::Center);

                    button(
                        container(row_content)
                            .width(Length::Fill)
                            .padding(ITEM_PADDING),
                    )
                    .width(Length::Fill)
                    .padding(0)
                    .style(move |_, status| {
                        let bg = match status {
                            button::Status::Pressed => theme.card_pressed(),
                            _ => Color::TRANSPARENT,
                        };
                        button::Style {
                            background: Some(bg.into()),
                            text_color,
                            border: Border::default(),
                            shadow: Shadow::default(),
                            snap: false,
                        }
                    })
                    .on_press(on_press)
                    .into()
                } else {
                    let mut left_items: Vec<Element<'a, Message>> = Vec::new();
                    if let Some(b) = badge {
                        left_items.push(render_badge(b));
                        left_items.push(Space::new().width(Length::Fixed(12.0)).into());
                    }
                    left_items.push(
                        text(label)
                            .size(DETAIL_FONT)
                            .color(text_color)
                            .into(),
                    );

                    let mut right_items: Vec<Element<'a, Message>> = Vec::new();
                    if let Some(val) = value {
                        right_items.push(
                            text(val)
                                .size(DETAIL_FONT)
                                .color(theme.text_secondary())
                                .into(),
                        );
                    }

                    let row_content = Row::new()
                        .width(Length::Fill)
                        .align_y(Alignment::Center)
                        .push(Row::with_children(left_items).align_y(Alignment::Center))
                        .push(Space::new().width(Length::Fill))
                        .push(Row::with_children(right_items).align_y(Alignment::Center));

                    button(
                        container(row_content)
                            .width(Length::Fill)
                            .padding(ITEM_PADDING),
                    )
                    .width(Length::Fill)
                    .padding(0)
                    .style(move |_, status| {
                        let bg = match status {
                            button::Status::Pressed => theme.card_pressed(),
                            _ => Color::TRANSPARENT,
                        };
                        button::Style {
                            background: Some(bg.into()),
                            text_color,
                            border: Border::default(),
                            shadow: Shadow::default(),
                            snap: false,
                        }
                    })
                    .on_press(on_press)
                    .into()
                }
            }
        }
    }
}

/// A logical group of items.
///
/// `ItemGroup` acts as a pure layout and structure coordinator:
/// - It has **no visual drawing effects of its own** (no card background, no corner radius, no border).
/// - It strictly only accepts [`Item`]s, preventing arbitrary widgets from breaking the row design contract.
/// - It automatically coordinates hairlines (separators) between adjacent items.
#[derive(Debug, Clone, Default)]
pub struct ItemGroup<Message> {
    items: Vec<Item<Message>>,
}

impl<Message: Clone> ItemGroup<Message> {
    /// Creates an empty item group.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Creates an [`ItemGroup`] from an existing collection or iterator of items.
    pub fn with_items(items: impl IntoIterator<Item = Item<Message>>) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }

    /// Creates an [`ItemGroup`] of key-value items from key-value pairs.
    pub fn from_pairs(
        pairs: impl IntoIterator<Item = (impl Into<Cow<'static, str>>, impl Into<Cow<'static, str>>)>,
    ) -> Self {
        let items = pairs
            .into_iter()
            .map(|(k, v)| Item::key_value(k, v))
            .collect();
        Self { items }
    }

    /// Pushes a strongly-typed [`Item`] into the group.
    pub fn push(mut self, item: Item<Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Alias for [`push`](Self::push).
    pub fn item(self, item: Item<Message>) -> Self {
        self.push(item)
    }

    /// Appends a navigation item directly.
    pub fn nav<Nav: NavigationAction<Message>>(
        self,
        label: impl Into<Cow<'static, str>>,
        value: Option<impl Into<Cow<'static, str>>>,
        action: Nav,
    ) -> Self {
        self.push(Item::nav(label, value, action))
    }

    /// Appends an option selection item directly.
    pub fn option(
        self,
        label: impl Into<Cow<'static, str>>,
        is_selected: bool,
        on_select: Message,
    ) -> Self {
        self.push(Item::option(label, is_selected, on_select))
    }

    /// Appends a switch toggle item directly.
    pub fn switch(
        self,
        label: impl Into<Cow<'static, str>>,
        on: bool,
        on_toggle: Message,
    ) -> Self {
        self.push(Item::switch(label, on, on_toggle))
    }

    /// Appends a read-only key-value item directly.
    pub fn key_value(
        self,
        label: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.push(Item::key_value(label, value))
    }

    /// Appends an action button item directly.
    pub fn action(
        self,
        label: impl Into<Cow<'static, str>>,
        color: Color,
        on_press: Message,
    ) -> Self {
        self.push(Item::action(label, color, on_press))
    }

    /// Returns the number of items in the group.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns whether the group is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Renders the items into a column with hairlines between rows.
    ///
    /// Note: Does NOT wrap in card surface (that is the responsibility of [`crate::Section`]).
    pub fn render<'a>(self, theme: ThemeMode) -> Element<'a, Message>
    where
        Message: 'a,
    {
        let mut children: Vec<Element<'a, Message>> = Vec::with_capacity(self.items.len() * 2);
        for (i, item) in self.items.into_iter().enumerate() {
            if i > 0 {
                children.push(item_separator::<Message>(theme).into());
            }
            children.push(item.render(theme));
        }
        Column::with_children(children).width(Length::Fill).into()
    }
}

impl<Message> FromIterator<Item<Message>> for ItemGroup<Message> {
    fn from_iter<I: IntoIterator<Item = Item<Message>>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

impl<Message> Extend<Item<Message>> for ItemGroup<Message> {
    fn extend<T: IntoIterator<Item = Item<Message>>>(&mut self, iter: T) {
        self.items.extend(iter);
    }
}

impl<Message> IntoIterator for ItemGroup<Message> {
    type Item = Item<Message>;
    type IntoIter = std::vec::IntoIter<Item<Message>>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<Message> std::ops::Deref for ItemGroup<Message> {
    type Target = [Item<Message>];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestMsg {
        Nav,
        Opt,
        Sw,
        Act,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TestRoute {
        Go,
    }

    impl NavigationAction<TestMsg> for TestRoute {
        fn into_message(self) -> TestMsg {
            TestMsg::Nav
        }
    }

    #[test]
    fn item_and_item_group_builds() {
        let group = ItemGroup::new()
            .push(Item::nav("Nav", Some("Val"), TestRoute::Go))
            .push(Item::option("Opt", true, TestMsg::Opt))
            .push(Item::switch("Sw", false, TestMsg::Sw))
            .push(Item::key_value("K", "V"))
            .push(Item::action("Act", Color::WHITE, TestMsg::Act));

        assert_eq!(group.len(), 5);
        let _elem: Element<'_, TestMsg> = group.render(ThemeMode::Dark);
    }
}
