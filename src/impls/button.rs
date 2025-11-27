use iced::widget::button::{Status, Style, StyleFn};
use crate::ex::Chameleon;
use crate::{lua_status, Theme};

impl iced::widget::button::Catalog for Theme{
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(|theme, status| {
            theme.button(status.morph()).morph()
        })
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

lua_status![
    active: Active,
    disabled: Disabled,
    hovered: Hovered,
    pressed: Pressed,
];