use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_file: bool,
    pub size: u64,
}

pub struct FileManager {
    pub entries: Vec<FileEntry>,
    pub current_path: PathBuf,
}

impl FileManager {
    pub fn new(path: &Path) -> Result<Self>{
        let mut entries = Vec::new();

        //Add parent directory entry
        if let Some(parent) = path.parent(){
            entries.push(FileEntry {
                name: "..".to_string(),
                path: parent.to_path_buf(),
                is_file: false,
                size: 0,
            });
        }
        //Read directory entries
        let dir_entries = std::fs::read_dir(path)?;
        for entry in dir_entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;

            let file_entry = FileEntry {
                name: path.file_name().unwrap().to_string_lossy().into_owned(),
                path: path.clone(),
                is_file: metadata.is_file(),
                size: metadata.len(),
            };
            entries.push(file_entry);
        }
        //Sort: Directories first, then files, both alphabetically
        entries.sort_by(|a, b| {
            if a.name == ".." {
                std::cmp::Ordering::Less
            } else if b.name == ".." {
                std::cmp::Ordering::Greater
            } else if a.is_file != b.is_file {
                a.is_file.cmp(&b.is_file).reverse()
            } else{
                a.name.to_lowercase().cmp(&b.name.to_lowercase())
            }
        });

        Ok(Self {
            entries,
            current_path: path.to_path_buf(),
        })
    }

    pub fn get_selected(&self, index: usize) -> Option<&FileEntry>{
        self.entries.get(index)
    }
}