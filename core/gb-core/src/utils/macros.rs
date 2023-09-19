// TODO: brush up my macro game

macro_rules! pure_read_write_methods_u8 {
    ($($field:ident),+) => {
        $(
            paste::paste! {
                pub fn [<read_ $field>](&self) -> u8 {
                    self.$field
                }

                pub fn [<write_ $field>](&mut self, value: u8) {
                    self.$field = value;
                }
            }
        )+
    };
}

pub(crate) use pure_read_write_methods_u8;
