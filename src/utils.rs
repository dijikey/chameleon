#[macro_export]
macro_rules! lua_getter {
    [
        $( $i:ident($($id:ident: $t:ty),*) -> $rt:ty ),+
    ] => {
    $(
        pub fn $i(&self$(,$id: $t)*) -> $rt {

            return self.inner.$i.call(($($id),*)).expect(stringify!($rt));
        }
    )+
    }
}

#[macro_export]
macro_rules! iced_wrap {
    (
        name: $name:ident,
        fields: { $($id:ident: $t:ty),* }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Default, Deserialize)]
        pub struct $name {
            $(pub $id: $t,)*
        }

        impl FromLua for $name {
            #[allow(unused_variables)]
            fn from_lua(value: Value, _: &Lua) -> mlua::Result<Self> {
                let table = match value.into() {
                    Value::Table(t) => t,
                    _ => return Err(mlua::Error::MismatchedRegistryKey),
                };

            Ok(Self {
                    $($id: table.get::<$t>(stringify!($id))?,)*
            })
            }
        }
    }
}

#[macro_export]
macro_rules! iced_impl {
    (
        module: $module:ident,
        theme_field: $field:ident
    ) => {
        impl iced::widget::$module::Catalog for Theme {
            type Class<'a> = iced::widget::$module::StyleFn<'a, Self>;

            fn default<'a>() -> Self::Class<'a> {
                Box::new(|theme| theme.$field().morph())
            }

            fn style(&self, class: &Self::Class<'_>) -> iced::widget::$module::Style {
                class(self)
            }
        }
    };
    (
        module: $module:ident,
        theme_field: $field:ident,
        status_true
    ) => {
        impl iced::widget::$module::Catalog for Theme {
            type Class<'a> = iced::widget::$module::StyleFn<'a, Self>;

            fn default<'a>() -> Self::Class<'a> {
                Box::new(|theme, status| theme.$field(status.morph()).morph())
            }

            fn style(
                &self,
                class: &Self::Class<'_>,
                status: iced::widget::$module::Status,
            ) -> iced::widget::$module::Style {
                class(self, status)
            }
        }
    };
}

#[macro_export]
macro_rules! lua_status {
    [
        module: $module:ident,
        $(active: $active:ident,)?
        $(disabled: $disabled:ident,)?
        $(hovered: $hovered:ident,)?
        $(pressed: $pressed:ident,)?
        $(dragged: $dragged:ident,)?
        $(idle: $idle:ident,)?
        $(focused: $focused:ident,)?
    ] => {
        impl Chameleon<i8> for iced::widget::$module::Status{
            fn morph(&self) -> i8 {
                match self {
                    $(Self::$active => $crate::STATUS_ACTIVE,)*
                    $(Self::$disabled => $crate::STATUS_DISABLED,)*
                    $(Self::$hovered => $crate::STATUS_HOVERED,)*
                    $(Self::$pressed => $crate::STATUS_PRESSED,)*
                    $(Self::$dragged => $crate::STATUS_DRAGGED,)*
                    $(Self::$idle => $crate::STATUS_IDLE,)*
                    $(Self::$focused => $crate::STATUS_FOCUSED,)*
                }
            }
        }
    }
}
