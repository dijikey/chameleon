use mlua::{Lua, Table};
use std::path::{Path, PathBuf};

pub const STYLE_FUNCTION: &str = "Style";
pub const LUA_FOLDER: &str = "lua";

#[inline]
pub fn load_modules<P: AsRef<Path>>(lua: &Lua, folder: P) -> anyhow::Result<()> {
    use mlua::Table;
    use serde::Deserialize;
    use std::path::PathBuf;

    use crate::CONFIGURE_FILE;

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct Configure {
        pub lua: Lua,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct Lua {
        pub module_folder: PathBuf,
        pub modules: Vec<String>,
    }

    let cfg: Configure = {
        let slice = std::fs::read(folder.as_ref().join(CONFIGURE_FILE))?;
        toml::from_slice(&slice)?
    };
    let modules = folder.as_ref().join(cfg.lua.module_folder);

    let values: Vec<String> = cfg.lua.modules;
    for modname in values {
        let file_name = format!("{modname}.lua");
        let path = modules.join(file_name);

        let func = lua.create_function(move |lua, ()| {
            let input = std::fs::read_to_string(path.clone())?;
            let table = lua.load(&input).eval::<Table>()?;
            Ok(table)
        })?;

        lua.preload_module(&modname, func)?;
    }

    Ok(())
}

pub fn load_toolkit(lua: &Lua) -> anyhow::Result<()> {
    use crate::{
        STATUS_ACTIVE, STATUS_DISABLED, STATUS_DRAGGED, STATUS_HOVERED, STATUS_IDLE, STATUS_PRESSED,
    };
    macro_rules! eq_func {
        ($name:literal, $table:ident, $eq_value:ident) => {
            $table.raw_set(
                $name,
                lua.create_function(|_, status: i8| Ok(status == $eq_value))?,
            )?
        };
    }

    let toolkit: Table = lua.create_table()?;
    toolkit.set("color", color(lua)?)?;

    eq_func!("is_active", toolkit, STATUS_ACTIVE);
    eq_func!("is_disabled", toolkit, STATUS_DISABLED);
    eq_func!("is_pressed", toolkit, STATUS_PRESSED);
    eq_func!("is_hovered", toolkit, STATUS_HOVERED);
    eq_func!("is_dragged", toolkit, STATUS_DRAGGED);
    eq_func!("is_idle", toolkit, STATUS_IDLE);

    lua.globals().set("toolkit", toolkit)?;
    Ok(())
}

// pub fn printable(table: &Table) -> anyhow::Result<()> {
//     for pair in table.pairs::<Value, Value>() {
//         let (key, value) = pair?;
//         let key: String = match key {
//             Value::Nil => "nil".to_string(),
//             Value::Boolean(b) => b.to_string(),
//             Value::Integer(i) => i.to_string(),
//             Value::Number(n) => n.to_string(),
//             Value::String(s) => s.to_string_lossy().to_string(),
//             Value::Table(t) => {
//                 println!("Table: ");
//                 printable(&t)?;
//                 continue;
//             }
//
//             _ => "Other".to_string(),
//         };
//
//         let value: String = match value {
//             Value::Nil => "nil".to_string(),
//             Value::Boolean(b) => b.to_string(),
//             Value::Integer(i) => i.to_string(),
//             Value::Number(n) => n.to_string(),
//             Value::String(s) => s.to_string_lossy().to_string(),
//             Value::Table(t) => {
//                 println!("Table: ");
//                 printable(&t)?;
//                 continue;
//             }
//
//             _ => "Other".to_string(),
//         };
//
//         println!("{key}: {value}");
//     }
//
//     Ok(())
// }

fn color(lua: &Lua) -> anyhow::Result<Table> {
    let color = lua.create_table()?;
    color.set(
        "rgba",
        lua.create_function(|lua, (r, g, b, a): (i32, i32, i32, i32)| rgba(lua, r, g, b, a))?,
    )?;
    color.set(
        "rgb",
        lua.create_function(|lua, (r, g, b): (i32, i32, i32)| rgb(lua, r, g, b))?,
    )?;

    return Ok(color);

    fn rgb(lua: &Lua, r: i32, g: i32, b: i32) -> mlua::Result<Table> {
        rgba(lua, r, g, b, 100)
    }

    fn rgba(lua: &Lua, r: i32, g: i32, b: i32, a: i32) -> mlua::Result<Table> {
        let color = lua.create_table()?;
        color.set("r", r)?;
        color.set("g", g)?;
        color.set("b", b)?;
        color.set("a", a)?;

        Ok(color)
    }
}

#[macro_export]
macro_rules! chunkloader {
    (
        state: $state:ident,
        folder: $folder:ident,
        scripts: [$($i:ident),*]
    ) => {
        use $crate::lua::STYLE_FUNCTION;
        use $crate::lua::LUA_FOLDER;

        $(
        let $i = {
            let input = match std::fs::read_to_string( $folder.as_ref().join(LUA_FOLDER).join( format!("{}.lua", stringify!($i))) ){
                Result::Ok(v) => v,
                Err(e) => return Err(anyhow::anyhow!("{}: {}", stringify!($i), e)),
            };

            $state.load(input).exec()?;
            $state.globals().get(STYLE_FUNCTION)?
        };
        )*
    };
}