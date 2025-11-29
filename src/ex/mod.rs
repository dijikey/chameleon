use crate::ex::misc::{Border, Color, Rounded, Shadow};
use crate::{iced_wrap, Chameleon};
use iced::widget;
use iced::widget::rule::FillMode;
use mlua::{FromLua, Lua, Value};
use serde::Deserialize;

pub(crate) mod misc;



iced_wrap!(
    name: Appearance,
    fields: {
        background: Color,
        text: Color,
        primary: Color
    }
);

impl Chameleon<iced::application::Appearance> for Appearance {
    fn morph(&self) -> iced::application::Appearance {
        iced::application::Appearance {
            background_color: self.background.morph(),
            text_color: self.text.morph(),
        }
    }
}

iced_wrap!(
    name: Text,
    fields: {
        color: Option<Color>
    }
);

impl Chameleon<widget::text::Style> for Text {
    fn morph(&self) -> widget::text::Style {
        widget::text::Style {
            color: self.color.morph(),
        }
    }
}

iced_wrap!(
    name: TextInput,
    fields: {
        background: Color,
        border: Border,
        icon: Color,
        placeholder: Color,
        value: Color,
        selection: Color
    }
);

impl Chameleon<widget::text_input::Style> for TextInput {
    fn morph(&self) -> widget::text_input::Style {
        widget::text_input::Style {
            background: self.background.morph(),
            border: self.border.morph(),
            icon: self.icon.morph(),
            placeholder: self.placeholder.morph(),
            value: self.value.morph(),
            selection: self.selection.morph(),
        }
    }
}

iced_wrap!(
    name: Svg,
    fields: {
        color: Option<Color>
    }
);

impl Chameleon<widget::svg::Style> for Svg {
    fn morph(&self) -> widget::svg::Style {
        widget::svg::Style {
            color: self.color.morph(),
        }
    }
}

iced_wrap!(
    name: Button,
    fields: {
        background: Option<Color>,
        text_color: Color,
        border: Border,
        shadow: Shadow
    }
);

impl Chameleon<widget::button::Style> for Button {
    fn morph(&self) -> widget::button::Style {
        widget::button::Style {
            background: self.background.morph(),
            text_color: self.text_color.morph(),
            border: self.border.morph(),
            shadow: self.shadow.morph(),
        }
    }
}

iced_wrap!(
    name: Container,
    fields: {
        background: Option<Color>,
        text_color: Option<Color>,
        border: Border,
        shadow: Shadow
    }
);

impl Chameleon<widget::container::Style> for Container {
    fn morph(&self) -> widget::container::Style {
        widget::container::Style {
            background: self.background.morph(),
            text_color: self.text_color.morph(),
            border: self.border.morph(),
            shadow: self.shadow.morph(),
        }
    }
}

iced_wrap!(
    name: Highlight,
    fields: {
        background: Color,
        border: Border
    }
);

impl Chameleon<widget::pane_grid::Highlight> for Highlight {
    fn morph(&self) -> widget::pane_grid::Highlight {
        widget::pane_grid::Highlight {
            background: self.background.morph(),
            border: Default::default(),
        }
    }
}

iced_wrap!(
    name: Line,
    fields: {
        color: Color,
        width: f32
    }
);

impl Chameleon<widget::pane_grid::Line> for Line {
    fn morph(&self) -> widget::pane_grid::Line {
        widget::pane_grid::Line {
            color: self.color.morph(),
            width: self.width,
        }
    }
}

iced_wrap!(
    name: PaneGrid,
    fields: {
        hovered_region: Highlight,
        picked_split: Line,
        hovered_split: Line
    }
);

impl Chameleon<widget::pane_grid::Style> for PaneGrid {
    fn morph(&self) -> widget::pane_grid::Style {
        widget::pane_grid::Style {
            hovered_region: self.hovered_region.morph(),
            picked_split: self.picked_split.morph(),
            hovered_split: self.hovered_split.morph(),
        }
    }
}

iced_wrap!(
    name: Rule,
    fields: {
        color: Color,
        width: u16,
        rounded: Rounded
    }
);

impl Chameleon<widget::rule::Style> for Rule {
    fn morph(&self) -> widget::rule::Style {
        widget::rule::Style {
            color: self.color.morph(),
            width: self.width,
            radius: self.rounded.morph(),
            fill_mode: FillMode::Full,
        }
    }
}

iced_wrap!(
    name: Scroller,
    fields: {
        color: Color,
        border: Border
    }
);

impl Chameleon<widget::scrollable::Scroller> for Scroller {
    fn morph(&self) -> widget::scrollable::Scroller {
        widget::scrollable::Scroller {
            color: self.color.morph(),
            border: self.border.morph(),
        }
    }
}

iced_wrap!(
    name: Rail,
    fields: {
        background: Option<Color>,
        border: Border,
        scroller: Scroller
    }
);

impl Chameleon<widget::scrollable::Rail> for Rail {
    fn morph(&self) -> widget::scrollable::Rail {
        widget::scrollable::Rail {
            background: self.background.morph(),
            border: self.border.morph(),
            scroller: self.scroller.morph(),
        }
    }
}

iced_wrap!(
    name: Scrollable,
    fields: {
        gap: Option<Color>,
        container: Container,
        rail: Rail
    }
);

impl Chameleon<widget::scrollable::Style> for Scrollable {
    fn morph(&self) -> widget::scrollable::Style {
        let rail = self.rail.morph();
        widget::scrollable::Style {
            container: self.container.morph(),
            vertical_rail: rail,
            horizontal_rail: rail,
            gap: self.gap.morph(),
        }
    }
}
