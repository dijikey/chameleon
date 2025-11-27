use crate::ex::Chameleon;
use crate::iced_impl;
use crate::{Theme, lua_status};
use iced::widget::{pane_grid, scrollable};

iced_impl!(
    module: container,
    theme_field: container
);

iced_impl!(
    module: rule,
    theme_field: rule
);

iced_impl!(
    module: text,
    theme_field: text
);

iced_impl!(
    module: text_input,
    theme_field: text_input,
    status_true
);

lua_status!(
    module: text_input,
    active: Active,
    disabled: Disabled,
    hovered: Hovered,
    focused: Focused,
);

iced_impl!(
    module: svg,
    theme_field: svg,
    status_true
);

lua_status![
    module: svg,
    hovered: Hovered,
    idle: Idle,
];

iced_impl!(
    module: button,
    theme_field: button,
    status_true
);

lua_status![
    module: button,
    active: Active,
    disabled: Disabled,
    hovered: Hovered,
    pressed: Pressed,
];

iced_impl!(
    module: scrollable,
    theme_field: scrollable,
    status_true
);

impl Chameleon<i8> for scrollable::Status {
    fn morph(&self) -> i8 {
        match self {
            scrollable::Status::Active => crate::STATUS_ACTIVE,
            scrollable::Status::Hovered { .. } => crate::STATUS_HOVERED,
            scrollable::Status::Dragged { .. } => crate::STATUS_DRAGGED,
        }
    }
}

impl pane_grid::Catalog for Theme {
    type Class<'a> = pane_grid::StyleFn<'a, Theme>;

    fn default<'a>() -> <Self as pane_grid::Catalog>::Class<'a> {
        Box::new(|theme| theme.pane_grid().morph())
    }

    fn style(&self, class: &<Self as pane_grid::Catalog>::Class<'_>) -> pane_grid::Style {
        class(self)
    }
}
