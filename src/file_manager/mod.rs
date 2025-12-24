use std::fs;
use std::io;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Files {
    name: String,
    is_dir: bool,
    size: u64,
    path: String,
    modified: String,
}

/// 列出指定目录下的所有文件和文件夹
pub fn list_files(dir_path: &str) -> io::Result<Vec<Files>> {
    let mut files = Vec::new();

    let entries = fs::read_dir(dir_path)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let metadata = entry.metadata()?;

        let name = entry.file_name().to_string_lossy().to_string();
        let is_dir = metadata.is_dir();
        let size = if is_dir { 0 } else { metadata.len() };
        let full_path = path.to_string_lossy().to_string();

        // 处理修改时间
        let modified = match metadata.modified() {
            Ok(time) => match time.duration_since(SystemTime::UNIX_EPOCH) {
                Ok(dur) => dur.as_secs().to_string(),
                Err(_) => "0".to_string(),
            },
            Err(_) => "0".to_string(),
        };

        files.push(Files {
            name,
            is_dir,
            size,
            path: full_path,
            modified,
        });
    }

    Ok(files)
}
