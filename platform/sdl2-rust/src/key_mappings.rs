use gb_core::constants::Button;
use sdl2::keyboard::Keycode;

#[allow(dead_code)]
pub fn map_button(button: Button) -> Keycode {
    match button {
        Button::A => Keycode::A,
        Button::B => Keycode::S,
        Button::Select => Keycode::Backspace,
        Button::Start => Keycode::Return,
        Button::Right => Keycode::Right,
        Button::Left => Keycode::Left,
        Button::Up => Keycode::Up,
        Button::Down => Keycode::Down,
    }
}

pub fn map_keycode(keycode: Keycode) -> Option<Button> {
    Some(match keycode {
        Keycode::A => Button::A,
        Keycode::S => Button::B,
        Keycode::Backspace => Button::Select,
        Keycode::Return => Button::Start,
        Keycode::Right => Button::Right,
        Keycode::Left => Button::Left,
        Keycode::Up => Button::Up,
        Keycode::Down => Button::Down,

        _ => return None,
    })
}
