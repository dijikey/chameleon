# Chameleon

A powerful and flexible crate for creating dynamic themes for [Iced](https://github.com/iced-rs/iced) applications using the Lua scripting language. Separate your application's style from its logic and enable runtime theme customization without recompilation.

---

## ✨ Features

- **Lua-Powered Theming**: Define your Iced application's visual style using simple and expressive Lua scripts.
- **Hot-Reloading**: Change your application's theme on the fly without restarting.
- **Separation of Concerns**: Keep your Rust business logic clean and your UI styling separate.
- **Comprehensive Styling**: Style all core Iced widgets (`Button`, `TextInput`, `Slider`, `Container`, etc.).
- **Type-Safe Bridge**: Enjoy a safe interaction between Lua and Rust with proper error handling.

## ✅ Roadmap:
- [x] **Add support for [Iced](https://github.com/iced-rs/iced)**
+ + Implementing the remaining widgets:
+ + + > radio, tooltip, toggler, text_editor, themer, slider


+[ ] **Add support for [egui](https://github.com/emilk/egui)**

## 🚀 Installation

Add this project to your `Cargo.toml`:

```toml
[dependencies]
chameleon-theme = "1.0.0"
```