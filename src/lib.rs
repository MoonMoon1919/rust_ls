use std::{
    fs::{self, DirEntry},
    path::Path,
    fmt,
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

impl fmt::Display for DirectoryContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let contents_friendly: Vec<String> = self.contents
                                                    .iter()
                                                    .map(|f| f.file_name().to_string_lossy().into_owned())
                                                    .collect();
        write!(f, "{}:\n{:?}\n", self.name, contents_friendly)
    }
}

pub fn visit_recursive(path: &Path) -> Vec<DirectoryContent> {
    let mut directory_contents: Vec<DirectoryContent> = vec![];

    let mut dc = DirectoryContent::new(path.as_os_str().to_string_lossy().into_owned());

    for p in visit_dir(path) {
        if p.path().is_dir() {
            let subpath_content = visit_recursive(&p.path());
            for entry in subpath_content {
                directory_contents.push(entry);
            }
        } else {
            dc.add_content(p);
        }
    }

    directory_contents.push(dc);

    directory_contents
}
