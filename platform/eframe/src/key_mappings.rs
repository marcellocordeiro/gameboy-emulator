use egui::Key;
use gb_core::Button;

pub trait EguiKeyMappings {
    fn mapped_to(self) -> Key;
}

impl EguiKeyMappings for Button {
    fn mapped_to(self) -> Key {
        match self {
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
}
