use gb_core::Button;
use libretro_rs::RetroJoypadButton;

pub trait LibretroKeyMappings {
    fn mapped_to(self) -> RetroJoypadButton;
}

impl LibretroKeyMappings for Button {
    fn mapped_to(self) -> RetroJoypadButton {
        match self {
            Self::A => RetroJoypadButton::A,
            Self::B => RetroJoypadButton::B,
            Self::Select => RetroJoypadButton::Select,
            Self::Start => RetroJoypadButton::Start,
            Self::Right => RetroJoypadButton::Right,
            Self::Left => RetroJoypadButton::Left,
            Self::Up => RetroJoypadButton::Up,
            Self::Down => RetroJoypadButton::Down,
        }
    }
}
