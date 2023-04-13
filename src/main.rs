use chrono::Datelike;
use colored::{Color, Colorize};
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;

const DIRECTORY_COLOR: Color = Color::Blue;
const SYMLINK_COLOR: Color = Color::BrightBlue;
const FILE_COLOR: Color = Color::BrightWhite;
const ERROR_COLOR: Color = Color::BrightRed;
const PATH_COLOR: Color = Color::BrightGreen;
const ARGUMENT_COLOR: Color = Color::Blue;
const DATE_COLOR: Color = Color::BrightGreen;
const PERMISSIONS_COLOR: Color = Color::BrightWhite;
const SIZE_COLOR: Color = Color::BrightWhite;
const VALID_ARGUMENTS: [&str; 7] = ["-a", "-p", "-d", "-h", "-sh", "-s", "-no"];

struct Arguments {
    path: String,
    show_hidden: bool,
    perm: bool,
    date: bool,
    size: bool,
    no_ordering: bool,
}

impl Arguments {
    fn analyze_entries(&self) -> Vec<DirEntry> {
        let entries: Vec<_> = match fs::read_dir(&self.path) {
            Ok(entries) => entries.filter_map(|entry| entry.ok()).collect(),
            Err(e) => {
                eprintln!(
                    "{} failed on opening directory:'{}'",
                    self.path.color(ERROR_COLOR),
                    e.to_string().color(ERROR_COLOR)
                );
                std::process::exit(0);
            }
        };
        entries
        //To-do transfer some functions to here, this project is not my current main project, so this can be a little bit late, if you can help the code make a pull request or take a issue
    }
    fn order_files(&self, mut entries: Vec<DirEntry>) -> Vec<DirEntry> {
        match self.no_ordering {
            true => entries.sort_by_cached_key(|entry| {
                let file_type = entry_type(entry);
                let is_dir = file_type.0;
                let is_symlink = file_type.1;
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
            }),
            false => (),
        }
        entries
    }
}

fn human_readable_size(size: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * KIB;
    const GIB: u64 = KIB * MIB;
    const TIB: u64 = KIB * GIB;
    const SIZE: [u64; 4] = [KIB, MIB, GIB, TIB];
    for level in SIZE {
        match size >= level {
            true => {
                let size = size as f64 / level as f64;
                match size >= level as f64 {
                    true => continue,
                    false => (),
                }
                let unit = match level {
                    KIB => "KiB",
                    MIB => "MiB",
                    GIB => "GiB",
                    TIB => "TiB",
                    _ => unreachable!(),
                };
                return format!(" - size: {:.2} {}", size, unit);
            }
            false => return format!(" - size: {}B", size),
        }
    }
    String::from(" - size 0B")
}

fn human_readable_permissions(mode: u32) -> String {
    let permissions: [&str; 3] = ["r", "w", "x"];
    let groups: [&str; 3] = ["User", "Group", "Other"];
    let mut result = String::new();
    for i in (0..9).step_by(3) {
        let bits = (mode >> i) & 0b111;
        result.push_str(groups[(i / 3) as usize]);
        result.push(':');
        for (i, _) in permissions.iter().enumerate() {
            result.push_str(match bits & (1 << (2 - i)) != 0 {
                true => permissions[i],
                false => "-",
            });
        }
        result.push(' ');
    }
    result.push_str(" - ");
    result
}

fn human_readable_date(timestamp: u64) -> String {
    let date = match chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0) {
        Some(date) => date,
        _ => return String::from(" - Error: Can not obtain the file date"),
    };
    format!(
        " - Mod. Date: {:2}/{}/{:02}",
        date.day(),
        date.month() - 1,
        date.year() % 100
    )
}

fn entry_type(entry: &DirEntry) -> (bool, bool) {
    let file_type = entry.file_type().expect("");
    (file_type.is_dir(), file_type.is_symlink())
}

fn format_file_name(entry: &DirEntry) -> (String, Color) {
    let result: (String, Color) = {
        let mut name: String = entry.file_name().to_str().expect("").to_owned();
        let mut color: Color = FILE_COLOR;
        let file_type = entry_type(entry);
        match file_type {
            (true, _) => {
                color = DIRECTORY_COLOR;
                name.push('/')
            }
            (_, true) => {
                color = SYMLINK_COLOR;
                name.push('@')
            }
            (_, _) => (),
        }
        (name, color)
    };
    result
}

fn print_file(entry: &DirEntry, mut arguments: Arguments) {
    match entry.file_name().to_string_lossy().starts_with('.') && !arguments.show_hidden {
        true => return,
        false => (),
    }
    match entry.metadata() {
        Ok(_) => (),
        Err(e) => match arguments {
            Arguments {
                path: _,
                show_hidden: _,
                perm: false,
                date: false,
                size: false,
                no_ordering: _,
            } => (),
            _ => {
                arguments.perm = false;
                arguments.date = false;
                arguments.size = false;
                println!(
                    "{} can't obtain metadata: {}",
                    "Error:".color(ERROR_COLOR),
                    e.to_string().color(ERROR_COLOR)
                )
            }
        },
    }
    let date: String = match arguments.date {
        false => String::from(""),
        true => human_readable_date(
            entry
                .path()
                .metadata()
                .unwrap()
                .modified()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ),
    };
    let permissions: String = match arguments.perm {
        false => "".to_owned(),
        true => human_readable_permissions(entry.metadata().expect("").permissions().mode()),
    };
    let size: String = match arguments.size {
        false => String::from(""),
        true => human_readable_size(entry.metadata().expect("").len()),
    };
    let file_info = format_file_name(entry);
    let name = file_info.0;
    let color = file_info.1;
    println!(
        "{}{}{}{}",
        permissions.color(PERMISSIONS_COLOR),
        name.color(color),
        size.color(SIZE_COLOR),
        date.color(DATE_COLOR)
    );
}

fn print_and_explain_valid_arguments() {
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

fn parse_path(mut path: String) -> String {
    if !path.ends_with('/') {
        path.push('/');
    }
    path
}

fn parse_arguments(argument: &str, mut result: Arguments) -> Arguments {
    match VALID_ARGUMENTS.iter().position(|&x| x == argument) {
        Some(0) => {
            result.perm = true;
            result.date = true;
            result.show_hidden = true;
            result.size = true;
        }
        Some(1) => result.perm = true,
        Some(2) => result.date = true,
        Some(3) => {
            println!("How to use: list + directory(optional) + argument(optional)");
            print_and_explain_valid_arguments();
            println!("Version: {}", "0.2.01".color(PATH_COLOR));
            std::process::exit(0);
        }
        Some(4) => result.show_hidden = true,
        Some(5) => result.size = true,
        Some(6) => result.no_ordering = true,
        _ => {
            println!(
                "{} invalid argument, type 'list {}' to see all the arguments",
                "Erro:".color(ERROR_COLOR),
                VALID_ARGUMENTS[3]
            );
            std::process::exit(0);
        }
    }
    result
}

fn interpreter() -> Arguments {
    let arguments = env::args().skip(1);
    let mut result: Arguments = Arguments {
        path: env::current_dir().unwrap().to_str().unwrap().to_owned(),
        show_hidden: false,
        perm: false,
        date: false,
        size: false,
        no_ordering: false,
    };
    for argument in arguments {
        match !argument.starts_with('-') {
            true => result.path = parse_path(argument),
            false => result = parse_arguments(&argument, result),
        }
    }
    result
}

fn main() {
    let arguments: Arguments = interpreter();
    let entries: Vec<DirEntry> = match arguments.no_ordering {
        true => arguments.analyze_entries(),
        false => arguments.order_files(arguments.analyze_entries()),
    };
    println!(
        "The content of the directory: {} is:",
        arguments.path.color(Color::BrightBlue)
    );
    for entry in entries {
        print_file(
            &entry,
            Arguments {
                path: arguments.path.to_owned(),
                show_hidden: arguments.show_hidden,
                perm: arguments.perm,
                date: arguments.date,
                size: arguments.size,
                no_ordering: arguments.no_ordering,
            },
        );
    }
}
