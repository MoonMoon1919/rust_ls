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

#[derive(Debug)]
pub struct DirectoryContent {
    name: String,
    contents: Vec<DirEntry>,
}

impl DirectoryContent {
    pub fn new(name: String) -> Self {
        DirectoryContent { name, contents: vec![] }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_content(&mut self, content: Vec<DirEntry>) {
        self.contents.extend(content);
    }

    pub fn add_content(&mut self, content: DirEntry) {
        self.contents.push(content);
    }

    pub fn content(&self) -> &Vec<DirEntry> {
        &self.contents
    }
}

pub fn visit_recursive(path: &Path) -> Vec<DirectoryContent> {
    let mut directory_contents: Vec<DirectoryContent> = vec![];

    for p in visit_dir(path) {
        println!("Checking dir entry {:?}", p.path());

        let mut dc = DirectoryContent::new(p.file_name().to_string_lossy().into_owned());

        if p.path().is_dir() {
            // TODO:

        } else {
            let contents = visit_dir(&p.path());
            dc.set_content(contents);
        }

        directory_contents.push(dc);
    }

    directory_contents
}
