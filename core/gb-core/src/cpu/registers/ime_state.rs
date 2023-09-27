#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ImeState {
    #[default]
    Disabled,
    Enabled,
    Pending,
}

impl ImeState {
    pub fn get_status(self) -> bool {
        match self {
            Self::Disabled | Self::Pending => false,
            Self::Enabled => true,
        }
    }

    pub fn update_and_get_status(&mut self) -> bool {
        match self {
            Self::Disabled => false,
            Self::Enabled => true,
            Self::Pending => {
                *self = Self::Enabled;

                false
            }
        }
    }
}

impl std::fmt::Display for ImeState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let str = match self {
            Self::Disabled => "☐",
            Self::Enabled => "☑",
            Self::Pending => "~",
        };

        write!(f, "{str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delay() {
        let mut ime_state = ImeState::default();

        assert_eq!(ime_state, ImeState::Disabled);
        assert!(!ime_state.get_status());
        assert!(!ime_state.update_and_get_status());

        ime_state = ImeState::Pending;

        assert_eq!(ime_state, ImeState::Pending);
        assert!(!ime_state.get_status());
        assert!(!ime_state.update_and_get_status()); // Mutates the value.

        assert_eq!(ime_state, ImeState::Enabled);
        assert!(ime_state.get_status());
        assert!(ime_state.update_and_get_status());
    }
}
