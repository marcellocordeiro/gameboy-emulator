use gb_core::Button;
use sdl2::keyboard::Keycode;

#[allow(dead_code)]
pub fn map_button(button: Button) -> Keycode {
    match button {
        Button::A => Keycode::X,
        Button::B => Keycode::Z,
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
        Keycode::X => Button::A,
        Keycode::Z => Button::B,
        Keycode::Backspace => Button::Select,
        Keycode::Return => Button::Start,
        Keycode::Right => Button::Right,
        Keycode::Left => Button::Left,
        Keycode::Up => Button::Up,
        Keycode::Down => Button::Down,

        _ => return None,
    })
}
