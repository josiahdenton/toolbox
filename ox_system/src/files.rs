//use anyhow::Result;
use std::os::unix::fs::MetadataExt;
use std::{fmt::Display, fs, path::Path};

#[derive(Debug)]
pub struct File {
    pub name: String,
    path: String,
    pub file_type: String,
    pub is_dir: bool,
    pub size: u64,
    pub children: Option<Vec<File>>,
}

// ------ how will I suport depth? ------
// I can get the list of files in the directory
// and then if a depth is supplied, I will iterate through
// each item in the directory, find where is_dir is true, and then
// add them as children to that directory.

impl File {
    /// accept a directory item and match to extension, return
    /// nerd font symbol associated with that type.
    pub fn file_type_symbol(&self) -> &str {
        if self.is_dir {
            return "";
        }

        if self.name == ".gitignore" {
            return "";
        }

        match self.file_type.as_str() {
            "py" => "",
            "rs" => "",
            "toml" => "",
            "lock" => "󰌾",
            "go" => "󰟓",
            "json" => "",
            _ => "",
        }
    }

    pub fn size_and_unit(&self) -> (u64, &str) {
        if self.size > 1024 {
            return (self.size / 1024, "KB");
        }

        (self.size, "B")
    }

    /// builder pattern to optionally check for children
    /// will do nothing if is_dir is false.
    /// if the path is invalid is missing, will
    /// leave self.children as None.
    ///
    /// If the recursive option is passed, will
    /// continue to find all possible files.
    pub fn with_children(&mut self, depth: u32) {
        if !self.is_dir {
            return;
        }

        self.children = Some(list_directory(Path::new(&self.path)));

        if depth > 0 {
            if let Some(children) = &mut self.children {
                children
                    .iter_mut()
                    .for_each(|child| child.with_children(depth - 1))
            }
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (size, unit) = self.size_and_unit();
        if let Some(children) = &self.children {
            let children = children
                .iter()
                .map(|entry| format!("\t{entry}"))
                .collect::<Vec<String>>()
                .join("\n");

            return write!(
                f,
                "{}\t{}{}\t{}\n{}",
                self.file_type_symbol(),
                size,
                unit,
                self.name,
                children
            );
        }
        write!(
            f,
            "{}\t{}{}\t{}",
            self.file_type_symbol(),
            size,
            unit,
            self.name
        )
    }
}

/// accepts a path and returns a list of the directory's files.
pub fn list_directory(path: &Path) -> Vec<File> {
    if path.is_file() {
        return vec![];
    }

    if let Ok(read) = fs::read_dir(path) {
        return read
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let path = entry.path();
                let mut file_size = 0;
                let mut is_dir = false;
                let file_name = path.file_name();
                if let Ok(meta) = path.metadata() {
                    file_size = meta.size();
                    is_dir = meta.is_dir();
                }
                let file_name = file_name
                    .map(|file| file.to_str().unwrap_or_default())
                    .unwrap_or_default();
                let file_name = file_name.split("/").last().unwrap_or_default();
                let file_type = file_name.split(".").last().unwrap_or_default();

                File {
                    name: String::from(file_name),
                    path: String::from(path.to_str().unwrap_or_default()),
                    file_type: if !file_name.starts_with(".") {
                        String::from(file_type)
                    } else {
                        String::from("")
                    },
                    size: file_size,
                    children: None,
                    is_dir,
                }
            })
            .collect::<Vec<File>>();
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir_items() {
        let items = list_directory(Path::new("./"));
        println!("{:?}", items);
        if let Some(item) = items.first() {
            assert_eq!(item.name, "Cargo.toml");
        } else {
            panic!();
        }
    }
}
