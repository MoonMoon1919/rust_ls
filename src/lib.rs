pub mod list;

use std::{
    fs::{self},
    path::Path,
    fmt,
};

// Presentation
#[derive(Debug)]
pub struct DirectoryContent {
    name: String,
    contents: Vec<list::INode>,
}

impl DirectoryContent {
    pub fn new(name: String) -> Self {
        DirectoryContent { name, contents: vec![] }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_content(&mut self, content: list::INode) {
        self.contents.push(content);
    }

    pub fn content(&self) -> &Vec<list::INode> {
        &self.contents
    }
}

impl fmt::Display for DirectoryContent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let contents_friendly: Vec<&String> = self.contents
                                                    .iter()
                                                    .map(|f| &f.name)
                                                    .collect();
        write!(f, "{}:\n{:?}\n", self.name, contents_friendly)
    }
}

// Filesystem operations
pub fn read_dir(path: &Path) -> Vec<list::INode> {
    let mut entries: Vec<list::INode> = vec![];

    if let Ok(dir_entry) = fs::read_dir(path) {
        dir_entry.into_iter().for_each(|f| {
            let entry = f.unwrap();
            let path = entry.path();

            let fname = entry.file_name().to_str().unwrap().to_owned();
            let is_dir = path.is_dir();

            entries.push(list::INode::new(fname, is_dir, path));
        })
    }

    entries
}

pub fn visit_dir(path: &Path, filter: &impl list::Filter) -> DirectoryContent {
    let entries = read_dir(path);

    let filtered_entries = entries.into_iter().filter(|i| filter.filter(i)).collect();

    DirectoryContent { name: path.to_string_lossy().into_owned(), contents: filtered_entries }
}


pub fn visit_recursive(path: &Path, filter: &impl list::Filter) -> Vec<DirectoryContent> {
    let mut directory_contents: Vec<DirectoryContent> = vec![];

    let dc = visit_dir(path, filter);

    for child in dc.content() {
        if child.is_dir {
            let subpath_content = visit_recursive(child.path.as_path(), filter);
            for entry in subpath_content {
                directory_contents.push(entry);
            }
        }
    }

    directory_contents.push(dc);

    directory_contents
}
