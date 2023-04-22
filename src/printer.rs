use crate::arguments::Arguments;
use crate::arguments::VALID_ARGUMENTS;
use crate::VERSION;
use chrono::Datelike;
use colored::{Color, Colorize};
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;

const DIRECTORY_COLOR: Color = Color::Blue;
const SYMLINK_COLOR: Color = Color::BrightBlue;
const FILE_COLOR: Color = Color::BrightWhite;
const ERROR_COLOR: Color = Color::BrightRed;
const DATE_COLOR: Color = Color::BrightGreen;
const PERMISSIONS_COLOR: Color = Color::BrightWhite;
const SIZE_COLOR: Color = Color::BrightWhite;
const PATH_COLOR: Color = Color::BrightBlue;
const ARGUMENT_COLOR: Color = Color::Blue;

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

pub fn printer(entries: Vec<DirEntry>, arguments: Arguments, path: String) {
    println!(
        "The content of the directory: {} is:",
        path.color(PATH_COLOR)
    );
    for entry in entries {
        print_file(&entry, arguments.clone());
    }
}

pub fn error(message: &str, e: &str) {
    println!("{}: {}", message, e.color(ERROR_COLOR));
    std::process::exit(1);
}

pub fn helper() {
    println!("How to use: list + directory(optional) + argument(optional)");
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
    println!("Version: {}", VERSION);
    std::process::exit(0);
}
