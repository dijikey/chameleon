use crate::ex::Chameleon;
use crate::iced_wrap;
use iced::Background;
use mlua::{FromLua, Lua, Value};
use serde::{Deserialize, Serialize};

/// A color in the `sRGB` color space.
#[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize, Serialize)]
pub struct Color {
    /// Red component, 0.0 - 1.0
    pub r: f32,
    /// Green component, 0.0 - 1.0
    pub g: f32,
    /// Blue component, 0.0 - 1.0
    pub b: f32,
    /// Transparency, 0.0 - 1.0
    pub a: f32,
}

impl FromLua for Color {
    fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
        let table = match value {
            Value::Table(t) => t,
            _ => return Err(mlua::Error::MismatchedRegistryKey),
        };

        Ok(Self {
            r: (table.get::<f32>("r")? / 255.0).clamp(0.0, 1.0),
            g: (table.get::<f32>("g")? / 255.0).clamp(0.0, 1.0),
            b: (table.get::<f32>("b")? / 255.0).clamp(0.0, 1.0),
            a: (table.get::<f32>("a")? / 100.0).clamp(0.0, 1.0),
        })
    }
}

impl Chameleon<Background> for Color {
    fn morph(&self) -> Background {
        Chameleon::<iced::Color>::morph(self).into()
    }
}

impl Chameleon<Option<Background>> for Option<Color> {
    fn morph(&self) -> Option<Background> {
        self.map(|c| Chameleon::<Background>::morph(&c))
    }
}

impl Chameleon<Option<iced::Color>> for Option<Color> {
    fn morph(&self) -> Option<iced::Color> {
        self.map(|c| Chameleon::<iced::Color>::morph(&c))
    }
}

impl Chameleon<iced::Color> for Color {
    fn morph(&self) -> iced::Color {
        iced::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

iced_wrap!(
    name: Vector,
    fields: {
        x: f32,
        y: f32
    }
);

impl Chameleon<iced::Vector<f32>> for Vector {
    fn morph(&self) -> iced::Vector<f32> {
        iced::Vector {
            x: self.x,
            y: self.y,
        }
    }
}

iced_wrap!(
    name: Shadow,
    fields: {
        color: Color,
        offset: Vector,
        blur_radius: f32
    }
);

impl Chameleon<iced::Shadow> for Shadow {
    fn morph(&self) -> iced::Shadow {
        iced::Shadow {
            color: self.color.morph(),
            offset: self.offset.morph(),
            blur_radius: self.blur_radius,
        }
    }
}

iced_wrap!(
    name: Rounded,
    fields: {
        top_left: f32,
        top_right: f32,
        bottom_right: f32,
        bottom_left: f32
    }
);

impl Chameleon<iced::border::Radius> for Rounded {
    fn morph(&self) -> iced::border::Radius {
        iced::border::Radius {
            top_left: self.top_left,
            top_right: self.top_right,
            bottom_right: self.bottom_right,
            bottom_left: self.bottom_left,
        }
    }
}

iced_wrap!(
    name: Border,
    fields: {
        color: Color,
        width: f32,
        rounded: Rounded
    }
);

impl Chameleon<iced::Border> for Border {
    fn morph(&self) -> iced::Border {
        iced::Border {
            color: self.color.morph(),
            width: self.width,
            radius: self.rounded.morph(),
        }
    }
}
