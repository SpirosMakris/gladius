use std::ffi;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[source] io::Error),
    #[error("Failed to read CString from file. File contains null(0)")]
    FileContainsNil,
    #[error("Failed to get executable path")]
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Resources, Error> {
        // Get the complete path to the program's executable
        let exe_file_name = ::std::env::current_exe().map_err(|_| Error::FailedToGetExePath)?;

        // We need just the path to the exe, without the exe name at the end
        let exe_path = exe_file_name.parent().ok_or(Error::FailedToGetExePath)?;

        dbg!("Resources: exe_path: {}", exe_path);

        Ok(Resources {
            root_path: exe_path.join(rel_path),
        })
    }

    pub fn from_exe_path() -> Result<Resources, Error> {
        Resources::from_relative_exe_path(Path::new(""))
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let mut file = fs::File::open(resource_name_to_path(&self.root_path, resource_name))?;

        // allocate buffer of the same size as the file
        let mut buffer: Vec<u8> = Vec::with_capacity(file.metadata()?.len() as usize + 1);
        file.read_to_end(&mut buffer)?;

        // check for nul byte
        if buffer.iter().find(|i| **i == 0).is_some() {
            return Err(Error::FileContainsNil);
        }

        Ok(unsafe { ffi::CString::from_vec_unchecked(buffer) })
    }
}

/// Deconstructs the location path and re-adds it to the root_dir
/// using std's path::join() so separators work on all platforms
fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();

    for part in location.split("/") {
        path = path.join(part);
    }

    path
}
