use crate::printer::error;
use crate::printer::helper;
use crate::PROGRAM_NAME;
use std::env;
use std::fs;
use std::fs::DirEntry;
pub const VALID_ARGUMENTS: [&str; 7] = ["-h", "-a", "-d", "-no", "-p", "-s", "-sh"];

#[derive(Clone)]
pub struct Arguments {
    pub show_hidden: bool,
    pub perm: bool,
    pub date: bool,
    pub size: bool,
    pub no_ordering: bool,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            show_hidden: false,
            perm: false,
            date: false,
            size: false,
            no_ordering: false,
        }
    }
    pub fn analyze_entries(&self, path: &str) -> Vec<DirEntry> {
        let entries: Vec<_> = match fs::read_dir(path) {
            Ok(entries) => entries.filter_map(|entry| entry.ok()).collect(),
            Err(e) => {
                error("Can't read directory", &e.to_string());
                std::process::exit(1);
            }
        };
        match self.no_ordering {
            true => entries,
            false => self.order_files(entries),
        }
    }
    fn order_files(&self, mut entries: Vec<DirEntry>) -> Vec<DirEntry> {
        entries.sort_by_cached_key(|entry| {
            let file_type = entry.file_type().expect("");
            let is_dir = file_type.is_dir();
            let is_symlink = file_type.is_symlink();
            let is_hidden = entry
                .path()
                .file_name()
                .map_or(false, |name| name.to_string_lossy().starts_with('.'));
            let priority = match (is_dir, is_symlink, is_hidden) {
                (true, _, false) => 0,
                (_, true, false) => 1,
                (_, _, false) => 2,
                (true, _, true) => 3,
                (_, true, true) => 4,
                (_, _, true) => 5,
            };
            (
                priority,
                entry
                    .path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                is_hidden,
            )
        });
        entries
    }
    fn parse_path(&self, mut path: String) -> String {
        if !path.ends_with('/') {
            path.push('/');
        }
        path
    }
    fn parse_arguments(mut self, argument: &str) -> Self {
        match VALID_ARGUMENTS.iter().position(|&x| x == argument) {
            Some(0) => helper(),
            Some(1) => {
                self.perm = true;
                self.date = true;
                self.show_hidden = true;
                self.size = true;
            }
            Some(2) => self.date = true,
            Some(3) => self.no_ordering = true,
            Some(4) => self.perm = true,
            Some(5) => self.size = true,
            Some(6) => self.show_hidden = true,
            _ => error(
                "Invalid argument",
                &format!(
                    "type '{} {}' to see all the arguments",
                    PROGRAM_NAME, VALID_ARGUMENTS[0]
                ),
            ),
        }
        self
    }
    pub fn interpreter(mut self) -> (Arguments, String) {
        let command_arguments = env::args().skip(1);
        let mut path: String = env::current_dir().unwrap().to_str().unwrap().to_owned();
        for argument in command_arguments {
            match !argument.starts_with('-') {
                true => path = self.parse_path(argument),
                false => self = self.parse_arguments(&argument),
            }
        }
        (self, path)
    }
}
