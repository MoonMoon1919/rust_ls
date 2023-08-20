use std::path;

#[derive(Debug, PartialEq, Clone)]
pub struct INode {
    pub name: String,
    children: Vec<INode>,
    pub path: path::PathBuf,
    pub is_dir: bool,
}

impl INode {
    pub fn new(name: String, is_dir: bool, path: path::PathBuf) -> Self {
        let children = vec![];

        INode { name, is_dir, path, children }
    }

    pub fn add_child(&mut self, child: INode) {
        self.children.push(child)
    }
}

pub trait Filter {
    fn filter(&self, item: &INode) -> bool {
        false
    }
}

pub struct IncludeDotFiles;

impl Filter for IncludeDotFiles {
    fn filter(&self, item: &INode) -> bool {
        let name = &item.name;

        if name.starts_with(".") {
            false
        } else {
            true
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter() {
        // Given
        let path = path::PathBuf::from(".git");
        let item = INode::new(String::from(".git"), true, path);
        let filter = IncludeDotFiles{};

        // When
        let result = filter.filter(&item);

        // Then
        assert_eq!(true, result)
    }
}
