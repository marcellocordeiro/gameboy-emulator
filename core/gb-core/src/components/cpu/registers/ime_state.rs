#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ImeState {
    #[default]
    Disabled,
    Enabled,
    Pending,
}

impl ImeState {
    pub fn is_enabled(self) -> bool {
        match self {
            Self::Disabled | Self::Pending => false,
            Self::Enabled => true,
        }
    }

    /// Delayed by one instruction.
    pub fn request_enable(&mut self) {
        if *self == Self::Disabled {
            *self = Self::Pending;
        }
    }

    pub fn force_enable(&mut self) {
        *self = Self::Enabled;
    }

    pub fn disable(&mut self) {
        *self = Self::Disabled;
    }

    /// Processes a pending request and returns the status.
    pub fn is_enabled_mut(&mut self) -> bool {
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
        assert!(!ime_state.is_enabled());
        assert!(!ime_state.is_enabled_mut());
        assert_eq!(ime_state, ImeState::Disabled);

        ime_state.request_enable();
        assert_eq!(ime_state, ImeState::Pending);

        assert!(!ime_state.is_enabled());
        assert!(!ime_state.is_enabled_mut()); // Mutates the value.
        assert_eq!(ime_state, ImeState::Enabled);

        ime_state.request_enable();
        assert_eq!(ime_state, ImeState::Enabled);

        assert!(ime_state.is_enabled());
        assert!(ime_state.is_enabled_mut());
        assert_eq!(ime_state, ImeState::Enabled);
    }
}
