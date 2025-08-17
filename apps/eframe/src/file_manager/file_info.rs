use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct FileInfo {
    pub data: Arc<[u8]>,

    #[cfg(not(target_arch = "wasm32"))]
    pub path: std::path::PathBuf,

    #[cfg(target_arch = "wasm32")]
    pub name: String,
}
