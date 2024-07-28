macro_rules! c_array {
    ($name:ident) => {
        #[repr(C)]
        pub struct $name {
            data: *const u8,
            size: usize,
        }

        impl CArray for $name {
            fn data(&self) -> *const u8 {
                self.data
            }

            fn size(&self) -> usize {
                self.size
            }
        }

        impl ToSlice for $name {}
    };
}

pub trait CArray {
    fn data(&self) -> *const u8;
    fn size(&self) -> usize;
}

pub trait ToSlice: CArray {
    /// # Safety
    ///
    /// The allocated size for the data pointer has to be equal to `size`.
    unsafe fn to_slice(&self) -> Option<&[u8]> {
        let data = self.data();
        let size = self.size();

        if data.is_null() {
            return None;
        }

        let slice = unsafe { std::slice::from_raw_parts(data, size) };

        Some(slice)
    }
}

c_array!(Bootrom);
c_array!(Rom);
