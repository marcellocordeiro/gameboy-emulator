use egui::Key;
use gb_core::utils::button::Button;

pub trait EguiKeyMappings {
    fn mapped_to(self) -> Key;
}

impl EguiKeyMappings for Button {
    fn mapped_to(self) -> Key {
        match self {
            Self::A => Key::X,
            Self::B => Key::Z,
            Self::Select => Key::Backspace,
            Self::Start => Key::Enter,
            Self::Right => Key::ArrowRight,
            Self::Left => Key::ArrowLeft,
            Self::Up => Key::ArrowUp,
            Self::Down => Key::ArrowDown,
        }
    }
}
