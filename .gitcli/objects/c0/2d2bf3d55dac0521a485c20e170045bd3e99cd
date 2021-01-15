use super::error::GitCliError;
use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

pub struct Index {
    pub path: PathBuf,
    pub hash_tree: BTreeMap<String, String>,
}

impl Index {
    pub fn new(root_dir: &PathBuf) -> Result<Index, GitCliError> {
        let mut index = Index {
            path: root_dir.join(".gitcli").join("index"),
            hash_tree: BTreeMap::new(),
        };

        if !index.path.exists() {
            return Ok(index);
        }

        let file = BufReader::new(File::open(&index.path)?);

        for line in file.lines() {
            let ln = line?;
            let blob: Vec<&str> = ln.split(' ').collect();
            if blob.len() != 2 {
                return Err(GitCliError::InvalidIndex);
            }

            index.update(blob[1], blob[0])
        }

        Ok(index)
    }

    pub fn update(&mut self, path: &str, hash: &str) {
        self.hash_tree.insert(path.to_string(), hash.to_string());
    }

    pub fn print(&self) {
        for (ref hash, ref path) in self.hash_tree.iter() {
            println!("{} {}", hash, path);
        }
    }

    pub fn write(&self) -> io::Result<()> {
        let mut index = File::create(&self.path)?;

        for (ref hash, ref path) in self.hash_tree.iter() {
            writeln!(&mut index, "{} {}", hash, path);
        }

        Ok(())
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.hash_tree = BTreeMap::new();
        self.write()
    }
}
