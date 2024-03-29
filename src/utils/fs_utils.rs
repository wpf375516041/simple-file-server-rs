use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tracing::error;

#[derive(Debug, Serialize)]
pub struct EntryInfo {
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: SystemTime,
}

// 实现获取某个目录下文件及目录列表 需要包含文件名，目录名，文件大小，文件类型，文件修改时间
pub fn get_directory_listing(dir_path: &Path) -> Vec<EntryInfo> {
    let mut entries = Vec::new();

    for entry_result in fs::read_dir(dir_path).expect("无法读取目录") {
        let entry = entry_result.expect("无法解析目录条目");
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                error!("无法读取目录条目: {} {:?}", err, entry.path());
                continue;
            }
        };
        let is_dir = if metadata.is_symlink() {
            let symlink_p = fs::canonicalize(entry.path()).expect("无法解析的软连接");
            let meta_p = fs::metadata(&symlink_p).unwrap();
            meta_p.is_dir()
        } else {
            metadata.is_dir()
        };
        let size = metadata.len();
        let modified = metadata.modified().unwrap();
        let entry_info = EntryInfo {
            path: entry.path(),
            is_dir,
            size,
            modified,
        };

        entries.push(entry_info);
    }
    entries
}
