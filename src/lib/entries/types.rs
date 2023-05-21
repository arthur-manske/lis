use std::fs::{metadata, DirEntry, FileType};
use std::os::unix::fs::PermissionsExt;

trait IsExec {
    fn is_executable(&self, entry: &DirEntry) -> bool {
        match metadata(entry.path()) {
            Ok(metadata) => metadata.permissions().mode() & 0o111 != 0,
            Err(_) => false,
        }
    }
}

impl IsExec for FileType {
    fn is_executable(&self, entry: &DirEntry) -> bool {
        match metadata(entry.path()) {
            Ok(metadata) => self.is_file() && metadata.permissions().mode() & 0o111 != 0,
            Err(_) => false,
        }
    }
}

pub fn get_entry_type(entry: &DirEntry) -> [bool; 3] {
    match entry.file_type().is_err() {
        true => [false, false, false],
        false => {
            let file_type = entry.file_type().unwrap();
            [
                file_type.is_dir(),
                file_type.is_symlink(),
                file_type.is_executable(entry),
            ]
        }
    }
}
