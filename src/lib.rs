use std::{
    fs::{self, DirEntry},
    path::Path,
};

pub fn visit_dir(path: &Path) -> Vec<DirEntry> {
    if let Ok(dir_entry) = fs::read_dir(path) {
        let entries: Vec<DirEntry> = dir_entry
            .into_iter()
            .filter_map(|f| f.ok())
            .collect();

        entries
    } else {
        let vec: Vec<DirEntry> = Vec::new();
        vec
    }
}
