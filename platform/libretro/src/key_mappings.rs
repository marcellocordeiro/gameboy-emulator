use gb_core::Button;
use libretro_rs::RetroJoypadButton;

pub fn map_button(button: Button) -> RetroJoypadButton {
    match button {
        Button::A => RetroJoypadButton::A,
        Button::B => RetroJoypadButton::B,
        Button::Select => RetroJoypadButton::Select,
        Button::Start => RetroJoypadButton::Start,
        Button::Right => RetroJoypadButton::Right,
        Button::Left => RetroJoypadButton::Left,
        Button::Up => RetroJoypadButton::Up,
        Button::Down => RetroJoypadButton::Down,
    }
}
