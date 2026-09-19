use gb_core::utils::button::Button as CoreButton;

#[repr(C)]
pub enum Button {
    A = 0,
    B = 1,
    SELECT = 2,
    START = 3,
    RIGHT = 4,
    LEFT = 5,
    UP = 6,
    DOWN = 7,
}

impl From<Button> for CoreButton {
    fn from(value: Button) -> Self {
        match value {
            Button::A => Self::A,
            Button::B => Self::B,
            Button::SELECT => Self::Select,
            Button::START => Self::Start,
            Button::RIGHT => Self::Right,
            Button::LEFT => Self::Left,
            Button::UP => Self::Up,
            Button::DOWN => Self::Down,
        }
    }
}
