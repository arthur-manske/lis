use colored::{Color, Colorize};
use std::env;
use std::fs;
use std::fs::DirEntry;

const SYMLINK_COLOR: Color = Color::BrightBlue;
const ERROR_COLOR: Color = Color::BrightRed;
const PATH_COLOR: Color = Color::BrightGreen;
const ARGUMENT_COLOR: Color = Color::Blue;
const VALID_ARGUMENTS: [&str; 7] = ["-a", "-p", "-d", "-h", "-sh", "-s", "-no"];

#[derive(Clone)]
pub struct Arguments {
    pub show_hidden: bool,
    pub perm: bool,
    pub date: bool,
    pub size: bool,
    pub no_ordering: bool,
}

impl Arguments {
    pub fn analyze_entries(&self, path: &str) -> Vec<DirEntry> {
        let entries: Vec<_> = match fs::read_dir(path) {
            Ok(entries) => entries.filter_map(|entry| entry.ok()).collect(),
            Err(e) => {
                eprintln!(
                    "{} failed on opening directory:'{}'",
                    path.color(ERROR_COLOR),
                    e.to_string().color(ERROR_COLOR)
                );
                std::process::exit(0);
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
    fn print_and_explain_valid_arguments(&self) {
        println!("The {} arguments are:", "valids".color(SYMLINK_COLOR));
        let arg_func: [&str; VALID_ARGUMENTS.len()] = [
        "all, made all the parameters active, expect by the help parameter and the no ordering parameter",
        "show the permissions of the file",
        "show the last modification date of the file",
        "explain how to use the program",
        "show hidden files",
        "show the files size",
        "don't order the files",
    ];
        for (i, &valid_arg) in VALID_ARGUMENTS.iter().enumerate() {
            println!("{}  -  {}", valid_arg.color(ARGUMENT_COLOR), arg_func[i]);
        }
    }
    fn parse_path(&self, mut path: String) -> String {
        if !path.ends_with('/') {
            path.push('/');
        }
        path
    }

    fn parse_arguments(mut self, argument: &str) -> Self {
        match VALID_ARGUMENTS.iter().position(|&x| x == argument) {
            Some(0) => {
                self.perm = true;
                self.date = true;
                self.show_hidden = true;
                self.size = true;
            }
            Some(1) => self.perm = true,
            Some(2) => self.date = true,
            Some(3) => {
                println!("How to use: list + directory(optional) + argument(optional)");
                self.print_and_explain_valid_arguments();
                println!("Version: {}", "0.2.01".color(PATH_COLOR));
                std::process::exit(0);
            }
            Some(4) => self.show_hidden = true,
            Some(5) => self.size = true,
            Some(6) => self.no_ordering = true,
            _ => {
                println!(
                    "{} invalid argument, type 'list {}' to see all the arguments",
                    "Error:".color(ERROR_COLOR),
                    VALID_ARGUMENTS[3]
                );
                std::process::exit(0);
            }
        }
        self
    }
    pub fn interpreter(mut self) -> (Arguments, String) {
        let arguments = env::args().skip(1);
        let mut path: String = env::current_dir().unwrap().to_str().unwrap().to_owned();
        for argument in arguments {
            match !argument.starts_with('-') {
                true => {
                    path = self.parse_path(argument);
                }
                false => {
                    self = self.parse_arguments(&argument);
                }
            }
        }
        (self, path)
    }
}
