#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub size: usize,
}

impl Buffer {
    /// # Safety
    ///
    /// The allocated size for the data pointer has to be equal to `size`.
    pub unsafe fn to_slice(&self) -> Option<&[u8]> {
        let data = self.data;
        let size = self.size;

        if data.is_null() {
            return None;
        }

        let slice = unsafe { std::slice::from_raw_parts(data, size) };

        Some(slice)
    }
}
