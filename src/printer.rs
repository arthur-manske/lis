use crate::arguments::Arguments;
use crate::arguments::VALID_ARGUMENTS;
use crate::arguments::VALID_ABREVIATIONS;
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
                return format!(" - {:.2} {}", size, unit);
            }
            false => return format!(" - {}B", size),
        }
    }
    String::from(" - 0B")
}

fn human_readable_permissions(mode: u32) -> String {
    let permissions: [&str; 3] = ["r", "w", "x"];
    let groups: [&str; 3] = ["U", "G", "O"];
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
    result.insert(0, ' ');
    result
}

fn human_readable_date(timestamp: u64) -> String {
    let date = match chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0) {
        Some(date) => date,
        _ => return String::from(" - Error: Can not obtain the file date"),
    };
    format!(
        " - {:2}/{}/{:02}",
        date.day(),
        date.month() - 1,
        date.year() % 100
    )
}

fn entry_type(entry: &DirEntry) -> (bool, bool) {
    let file_type = entry.file_type().expect("");
    (file_type.is_dir(), file_type.is_symlink())
}

fn format_entry_name(entry: &DirEntry) -> (String, Color) {
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

fn print_file(entry: &DirEntry, arguments: &Arguments) {
    if entry.file_name().to_string_lossy().starts_with('.') && !arguments.show_hidden {
        return;
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
    let (entry_name, entry_color) = format_entry_name(entry);
    println!(
        "{}{}{}{}",
        permissions.color(PERMISSIONS_COLOR),
        entry_name.color(entry_color),
        size.color(SIZE_COLOR),
        date.color(DATE_COLOR)
    );
}

pub fn printer(entries: Vec<DirEntry>, mut arguments: Arguments, path: String) {
    println!(
        "The content of the directory: {} is:",
        path.color(PATH_COLOR)
    );
    for entry in entries {
        match entry.metadata() {
            Ok(_) => (),
            Err(e) => {
                if arguments.size | arguments.perm | arguments.date {
                    arguments.perm = false;
                    arguments.date = false;
                    arguments.size = false;
                    error("Can't obtain metadata", &e.to_string());
                }
            }
        }
        print_file(&entry, &arguments);
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
        "explain how to use the program",
        "show the last modification date of the file",
        "don't order the files",
        "show the permissions of the file",
        "show the files size",
        "show hidden files",
    ];
    let binding_for: [&str; VALID_ABREVIATIONS.len()] = [
        "--help",
        "--date",
        "--no-order",
        "--permissions",
        "--size",
        "--show-hidden",
        "--show-hidden + --size + --permissions + --date",
    ];
    for (i, &valid_arg) in VALID_ARGUMENTS.iter().enumerate() {
        println!("{} - {}", valid_arg.color(ARGUMENT_COLOR), arg_func[i]);
    }
    println!(
        "The {} abreviations for arguments are: ",
        "valids".color(SYMLINK_COLOR)
    );
    for (i, &valid_arg) in VALID_ABREVIATIONS.iter().enumerate() {
        println!(
            "{} for {}",
            valid_arg.color(SYMLINK_COLOR),
            binding_for[i].color(ARGUMENT_COLOR)
        );
    }
    println!("Version: {}", VERSION);
    std::process::exit(0);
}
