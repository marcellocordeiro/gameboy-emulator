// TODO: brush up my macro game

macro_rules! pure_read_write_methods_u8 {
    ($($field:ident),+) => {
        $(
            paste::paste! {
                pub(crate) fn [<read_ $field>](&self) -> u8 {
                    self.$field
                }

                #[allow(dead_code)]
                pub(crate) fn [<write_ $field>](&mut self, value: u8) {
                    self.$field = value;
                }
            }
        )+
    };
}

macro_rules! device_is_cgb {
    ($self:ident) => {
        $self.device_config.is_cgb()
    };
}

macro_rules! in_cgb_mode {
    ($self:ident) => {
        $self.device_config.in_cgb_mode()
    };
}

pub(crate) use device_is_cgb;
pub(crate) use in_cgb_mode;
pub(crate) use pure_read_write_methods_u8;
