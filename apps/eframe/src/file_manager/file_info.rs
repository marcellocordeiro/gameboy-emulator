use std::sync::Arc;

#[derive(Debug, Default, Clone)]
pub struct FileInfo {
    pub data: Arc<[u8]>,
    pub path: std::path::PathBuf,
}
