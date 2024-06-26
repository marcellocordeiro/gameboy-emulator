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
        $self.device_model == crate::DeviceModel::Cgb
    };
}

macro_rules! in_cgb_mode {
    ($self:ident) => {
        if $self.cgb_mode {
            // #[cfg(test)]
            assert!(crate::utils::macros::device_is_cgb!($self));
            true
        } else {
            false
        }
    };
}

pub(crate) use device_is_cgb;
pub(crate) use in_cgb_mode;
pub(crate) use pure_read_write_methods_u8;
