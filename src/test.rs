use crate::Theme;
use crate::ex::Button;
use anyhow::Ok;

#[test]
fn load_lua() -> anyhow::Result<()> {
    #[allow(unused_variables)]
    let theme = Theme::open("test/blueberry")?;
    let _button: Button = theme.button(8);
    let _var = theme.appearance();

    println!("{}", size_of_val(&theme));

    Ok(())
}
