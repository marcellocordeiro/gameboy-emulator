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

    /// Delayed by one cycle.
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

    pub fn process_request(&mut self) {
        if *self == Self::Pending {
            *self = Self::Enabled;
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

        write!(f, "IME: {str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let mut ime_state = ImeState::default();

        assert_eq!(ime_state, ImeState::Disabled);
        assert!(!ime_state.is_enabled());

        // Shouldn't do anything.
        ime_state.process_request();

        assert_eq!(ime_state, ImeState::Disabled);
        assert!(!ime_state.is_enabled());
    }

    #[test]
    fn test_request() {
        let mut ime_state = ImeState::default();

        // Shouldn't do anything
        ime_state.process_request();

        assert_eq!(ime_state, ImeState::Disabled);
        assert!(!ime_state.is_enabled());

        // Requesting (Disabled -> Pending)
        ime_state.request_enable();

        assert_eq!(ime_state, ImeState::Pending);
        assert!(!ime_state.is_enabled());

        // Requesting again shouldn't do anything
        ime_state.request_enable();

        assert_eq!(ime_state, ImeState::Pending);
        assert!(!ime_state.is_enabled());
    }

    #[test]
    fn test_process() {
        let mut ime_state = ImeState::default();

        ime_state.request_enable();

        assert_eq!(ime_state, ImeState::Pending);
        assert!(!ime_state.is_enabled());

        // Processing (Pending -> Enabled)
        ime_state.process_request();

        assert!(ime_state.is_enabled());
        assert_eq!(ime_state, ImeState::Enabled);

        // Processing again shouldn't do anything.
        ime_state.process_request();

        assert!(ime_state.is_enabled());
        assert_eq!(ime_state, ImeState::Enabled);

        // Requesting again shouldn't do anything.
        ime_state.request_enable();

        assert!(ime_state.is_enabled());
        assert_eq!(ime_state, ImeState::Enabled);
    }

    #[test]
    fn test_e2e() {
        let mut ime_state = ImeState::default();

        // Cycle 1
        assert!(!ime_state.is_enabled());
        assert_eq!(ime_state, ImeState::Disabled);
        ime_state.request_enable();

        // Cycle 2
        assert!(!ime_state.is_enabled());
        assert_eq!(ime_state, ImeState::Pending);
        ime_state.process_request();

        // Cycle 3
        assert!(ime_state.is_enabled());
        assert_eq!(ime_state, ImeState::Enabled);
    }
}
