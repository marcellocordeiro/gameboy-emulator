use eframe::egui;
use gb_core::constants::Button;

pub fn map_button(button: Button) -> egui::Key {
    use egui::Key;

    match button {
        Button::A => Key::A,
        Button::B => Key::S,
        Button::Select => Key::Backspace,
        Button::Start => Key::Enter,
        Button::Right => Key::ArrowRight,
        Button::Left => Key::ArrowLeft,
        Button::Up => Key::ArrowUp,
        Button::Down => Key::ArrowDown,
    }
}
