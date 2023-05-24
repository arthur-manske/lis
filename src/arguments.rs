use crate::print::{error, helper, version};
use std::env::{args, current_dir};
use std::fs::{read_dir, DirEntry};

#[derive(Default)]
pub struct Arguments {
    show_hidden: bool,
    perm: bool,
    date: bool,
    size: bool,
    no_order: bool,
    force_print_as_lines: bool,
}

impl Arguments {
    pub fn show_hidden(&self) -> bool {
        self.show_hidden
    }
    pub fn permissions(&self) -> bool {
        self.perm
    }
    pub fn date(&self) -> bool {
        self.date
    }
    pub fn size(&self) -> bool {
        self.size
    }
    pub fn no_order(&self) -> bool {
        self.no_order
    }
    pub fn force_print_as_lines(&self) -> bool {
        self.force_print_as_lines   
    }
    fn analyze_entries(&self, path: &str) -> Vec<DirEntry> {
        let entries: Vec<DirEntry> = match read_dir(path) {
            Ok(entries) if self.show_hidden => entries.filter_map(|entry| entry.ok()).collect(),
            Ok(entries) => entries
                .filter_map(|entry| {
                    if entry.as_ref().expect("").file_name().to_str().unwrap().starts_with('.')
                    {
                        None
                    } else {
                        entry.ok()
                    }
                })
                .collect(),
            Err(e) => {
                std::process::exit(error("Can't read dir", &e.to_string()));
            }
        };
        entries
    }
        
    fn parse_arguments(&mut self, argument: &str) {
        match argument {
            "--help"        | "-h"  => helper(),
            "--version"     | "-v"  => version(),
            "--date"        | "-d"  => self.date = true,
            "--no-order"    | "-no" => self.no_order = true,
            "--permissions" | "-p"  => self.perm = true,
            "--size"        | "-s"  => self.size = true,
            "--show-hidden" | "-sh" => self.show_hidden = true,
            "--lines"       | "-l"  => self.force_print_as_lines = true,
            "--all"         | "-a"  => {
                self.date = true;
                self.perm = true;
                self.size = true;
                self.show_hidden = true;
            }
            _ => std::process::exit(error(
                "Invalid argument",
                "to see the valid arguments type lis --help",
            )),
        }
    }

    pub fn interpreter() -> (Arguments, Vec<DirEntry>) {
        let mut arguments = Arguments::default();
        let command_arguments = args().skip(1);
        let mut path = String::from(current_dir().unwrap().to_str().unwrap());
        for argument in command_arguments {
            match !argument.starts_with('-') {
                true => path = argument,
                false => arguments.parse_arguments(&argument),
            }
        }
        let entries = arguments.analyze_entries(&path);
        (arguments, entries)
    }
}
