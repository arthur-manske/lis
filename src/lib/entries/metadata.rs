use crate::lib::entries::types::get_entry_type;
use crate::lib::color::{Color, Format, format};
use std::fs::DirEntry;

const DIRECTORY_COLOR: Color = Color::Blue;
const SYMLINK_COLOR: Color = Color::BrightBlue;
const FILE_COLOR: Color = Color::White;
const EXECUTABLE_COLOR: Color = Color::Green;

pub fn format_entry_name(entry: &DirEntry) -> String {
    let entry_name: String = String::from(entry.file_name().to_string_lossy());
    let entry_type = get_entry_type(&entry);
    let mut entry_color: Color = FILE_COLOR;
    let mut entry_format: Format = Format::Default;
    match entry_type {
        [true, _, _] => {
            //Directory
            entry_color = DIRECTORY_COLOR;
            entry_format = Format::Bold;
        }
        [_, true, _] => {
            //Symlink
            entry_color = SYMLINK_COLOR;
            entry_format = Format::Bold;
        }
        [_, _, true] => {
            //Executable
            entry_color = EXECUTABLE_COLOR;
            entry_format = Format::Bold;
        }
        [_, _, _] => (), //File
    }
    format(entry_name, entry_color, entry_format)
}

fn entry_priority(entry: &DirEntry) -> u8 {
    let [is_dir, is_symlink, is_exec] = get_entry_type(&entry);
    let name = entry
        .path()
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    match [is_dir, is_symlink, is_exec, name.starts_with('.')] {
        [true, _, _, false] => 1,  //Dir
        [_, true, _, false] => 2,  //Symlink
        [_, _, true, false] => 3,  //Executable
        [_, _, false, false] => 4, //File
        [true, _, _, true] => 5,   //Hidden Dir
        [_, true, _, true] => 6,   //Hidden Symlink
        [_, _, true, true] => 7,   //Hidden Executable
        [_, _, false, true] => 8,  //Hidden File
    }
}

pub fn order_entries(mut entries: Vec<DirEntry>) -> Vec<DirEntry> {
    entries.sort_by_cached_key(|entry| {
        let priority = entry_priority(entry);
        let name = entry
            .path()
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        (priority, name)
    });
    entries
}
