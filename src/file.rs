use super::error::GitCliError;
use super::types::Blob;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

pub struct FileService {
    pub root_dir: PathBuf,
    pub gitcli_dir: PathBuf,
    pub object_dir: PathBuf,
}

impl FileService {
    pub fn new() -> Result<Self, GitCliError> {
        let root_dir = FileService::find_root()?;
        let gitcli_dir = root_dir.join(".gitcli").to_path_buf();
        let object_dir = gitcli_dir.join("objects").to_path_buf();

        Ok(FileService {
            root_dir,
            gitcli_dir,
            object_dir,
        })
    }

    fn find_root() -> Result<PathBuf, GitCliError> {
        let mut current_dir = env::current_dir()?;

        loop {
            if FileService::is_gitcli(&current_dir) {
                return Ok(current_dir);
            }
            if !current_dir.pop() {
                return Err(GitCliError::NoDirectory);
            }
        }

        Ok(current_dir)
    }

    fn is_gitcli<P>(path: P) -> bool
    where
        P: Sized + AsRef<Path>,
    {
        path.as_ref().join(".gitcli").exists()
    }

    pub fn get_head_ref(&self) -> io::Result<PathBuf> {
        let mut head_file = File::open(self.root_dir.join(".gitcli/HEAD"))?;
        let mut ref_path = String::new();
        head_file.read_to_string(&mut ref_path);
        let ref_path = ref_path.split_off(6);

        Ok(self.gitcli_dir.join(ref_path))
    }

    pub fn write_blob(&self, blob: &Blob) -> io::Result<()> {
        self.write_object(&blob.hash, &blob.data)
    }

    pub fn write_object(&self, hash: &str, data: &Vec<u8>) -> io::Result<()> {
        let blob_dir = self.object_dir.join(&hash[..2]);
        if !blob_dir.exists() {
            fs::create_dir(&blob_dir)?;
        }

        let blob_filename = blob_dir.join(&hash[2..]);
        let mut blob_f = File::create(&blob_filename)?;
        blob_f.write_all(data)?;

        Ok(())
    }
}
