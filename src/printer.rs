use crate::arguments::Arguments;
use crate::arguments::VALID_ABREVIATIONS;
use crate::arguments::VALID_ARGUMENTS;
use crate::VERSION;
use crate::color::Color;
use crate::color::colorize;
use crate::terminal_info::get_terminal_size;
use std::fs::DirEntry;
use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;

const DIRECTORY_COLOR: Color = Color::Blue;
const SYMLINK_COLOR: Color = Color::BrightBlue;
const FILE_COLOR: Color = Color::BrightWhite;
const ERROR_COLOR: Color = Color::Red;
const DATE_COLOR: Color = Color::Green;
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
                    KIB => "Kb",
                    MIB => "Mb",
                    GIB => "Gb",
                    TIB => "Tb",
                    _ => unreachable!(),
                };
                return format!(" - {:.2} {}", size, unit);
            }
            false => return format!(" - {}b", size),
        }
    }
    String::from(" - 0b")
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
    result.push_str("| ");
    result.insert(0, ' ');
    result
}

fn human_readable_date(timestamp: u64) -> String {
    let secs_per_day = 86400;
    let mut days_since_epoch = timestamp / secs_per_day;
    let mut year = 1970;
    let mut days_in_year = 365;
    while days_since_epoch >= days_in_year as u64 {
        days_since_epoch -= days_in_year as u64;
        year += 1;
        days_in_year = if is_leap_year(year) { 366 } else { 365 };
    }
    let mut days = days_since_epoch as u64;
    let mut month = 0;
    while days >= days_in_month(month, year) {
        days -= days_in_month(month, year);
        month += 1;
    }
    let day = days + 1;
    format!(" - {:02}/{:02}/{:02}", day, month + 1, year % 100)
}

fn days_in_month(month: u32, year: u32) -> u64 {
    match month {
        0 => 31,
        1 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        2 => 31,
        3 => 30,
        4 => 31,
        5 => 30,
        6 => 31,
        7 => 31,
        8 => 30,
        9 => 31,
        10 => 30,
        11 => 31,
        _ => unreachable!(),
    }
}

fn is_leap_year(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
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

fn print_files(entries: Vec<DirEntry>, arguments: Arguments) {
    for entry in entries {
        let metadata = match entry.metadata() {
            Ok(_) => entry.metadata().unwrap(),
            Err(e) => {
                error("Can't obtain metadata", &e.to_string());
                std::process::exit(1);
            }
        };
        let date: String = match arguments.date {
            false => String::from(""),
            true => human_readable_date(
                metadata
                    .modified()
                    .unwrap()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            ),
        };
        let permissions: String = match arguments.perm {
            false => "".to_owned(),
            true => human_readable_permissions(metadata.permissions().mode()),
        };
        let size: String = match arguments.size {
            false => String::from(""),
            true => human_readable_size(metadata.size()),
        };
        let (entry_name, entry_color) = format_entry_name(&entry);
        println!(
            "{}{}{}{}",
            colorize(&permissions, PERMISSIONS_COLOR),
            colorize(&entry_name, entry_color),
            colorize(&size, SIZE_COLOR),
            colorize(&date, DATE_COLOR)
        );
    }
}

fn print_files_in_columns(entries: Vec<DirEntry>) {
    let width = get_terminal_size().unwrap()[0];
    let mut higher_len = 0;
    for entry in &entries {
        let entry_name = format_entry_name(&entry).0;
        let len = entry_name.len();
        if len > higher_len {
            higher_len = len;
        }
    }
    let num_cols = match width as usize / (higher_len + 2) {
        0 => 1,
        _ => width as usize / (higher_len + 2),
    };
    for (i, entry) in entries.iter().enumerate() {
        let (entry_name, entry_color) = format_entry_name(&entry);
        print!("{:len$}", colorize(&entry_name, entry_color), len = higher_len + 11); //Added 11 because of the formating chars
        if (i + 1) % num_cols == 0 || (i + 1) == entries.len() {
            println!()
        } else {
            print!("   ")
        }
    }
}

pub fn printer(arguments: Arguments, path: &str) {
    let entries = arguments.analyze_entries(path);
    if entries.len() > 0 {
        println!("The directory: {} contains:", colorize(path, PATH_COLOR));
    } else {
        println!("The directory: {} is empty", colorize(path, PATH_COLOR));
        return;
    }
    if arguments.size | arguments.perm | arguments.date {
        print_files(entries, arguments);
    } else {
        print_files_in_columns(entries);
    }
}

pub fn error(message: &str, e: &str) {
    println!("{}: {}", message, colorize(&e, ERROR_COLOR));
    std::process::exit(1);
}

pub fn helper() {
    println!("How to use: list + directory(optional) + argument(optional)");
    println!("The {} arguments are:", colorize("valids", SYMLINK_COLOR));
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
        println!("{} - {}", colorize(valid_arg, ARGUMENT_COLOR), arg_func[i]);
    }
    println!(
        "The {} abreviations for arguments are: ", colorize("valid", SYMLINK_COLOR)
    );
    for (i, &valid_arg) in VALID_ABREVIATIONS.iter().enumerate() {
        println!(
            "{} for {}",
            colorize(valid_arg, SYMLINK_COLOR),
            colorize(binding_for[i], ARGUMENT_COLOR)
        );
    }
    println!("Version: {}", VERSION);
    std::process::exit(0);
}
