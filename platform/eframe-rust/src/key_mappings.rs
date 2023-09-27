use egui::Key;
use gb_core::utils::button::Button;

pub fn map_button(button: Button) -> Key {
    match button {
        Button::A => Key::X,
        Button::B => Key::Z,
        Button::Select => Key::Backspace,
        Button::Start => Key::Enter,
        Button::Right => Key::ArrowRight,
        Button::Left => Key::ArrowLeft,
        Button::Up => Key::ArrowUp,
        Button::Down => Key::ArrowDown,
    }
}
