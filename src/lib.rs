use crate::ex::{
    Appearance, Button, Container, PaneGrid, Rule, Scrollable, Svg, Text, TextInput,
};
use anyhow::Ok;
use iced::daemon::DefaultStyle;
use mlua::{Function, Lua};
use std::path::Path;
use std::sync::Arc;

pub(crate) mod ex;
mod impls;
pub(crate) mod lua;
#[cfg(test)]
mod test;
pub(crate) mod utils;

const CONFIGURE_FILE: &str = "configure.toml";
const STATUS_ACTIVE: i8 = 0;
const STATUS_DISABLED: i8 = 1;
const STATUS_PRESSED: i8 = 2;
const STATUS_HOVERED: i8 = 3;
const STATUS_DRAGGED: i8 = 4;
const STATUS_IDLE: i8 = 5;
const STATUS_FOCUSED: i8 = 6;

#[derive(Clone, Debug)]
pub struct Theme {
    inner: Arc<Iced>,
}

impl Theme {
    pub fn open<P: AsRef<Path> + Copy>(folder: P) -> anyhow::Result<Self> {
        let lua = Lua::new();
        crate::lua::load_modules(&lua, folder)?;
        crate::lua::load_toolkit(&lua)?;

        chunkloader!(
            state: lua,
            folder: folder,
            scripts: [appearance, button, container, rule, scrollable, svg, text, pane_grid, text_input]
        );

        let container = Iced {
            appearance,
            button,
            container,
            rule,
            scrollable,
            svg,
            text,
            text_input,
            pane_grid,
            lua,
        };

        Ok(Self {
            inner: Arc::from(container),
        })
    }

    lua_getter! [
        appearance() -> Appearance,
        container() -> Container,
        rule() -> Rule,
        text() -> Text,
        pane_grid() -> PaneGrid,
        text_input(status: i8) -> TextInput,
        svg(status: i8) -> Svg,
        scrollable(status: i8) -> Scrollable,
        button(status: i8) -> Button
    ];
}

#[derive(Clone, Debug)]
struct Iced {
    appearance: Function,
    button: Function,
    container: Function,
    rule: Function,
    scrollable: Function,
    svg: Function,
    text: Function,
    text_input: Function,
    pane_grid: Function,
    #[allow(dead_code)]
    lua: Lua,
}

#[allow(dead_code)]
pub trait Chameleon<T> {
    fn morph(&self) -> T;
}

impl Default for Theme {
    fn default() -> Theme {
        panic!("Theme cannot create with default")
    }
}

impl DefaultStyle for Theme {
    fn default_style(&self) -> iced::application::Appearance {
        self.appearance().morph()
    }
}
