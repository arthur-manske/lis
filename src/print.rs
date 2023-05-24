use crate::arguments::Arguments;
use crate::lib::entries::formating::{order_entries, format_entry_name};
use crate::lib::entries::metadata::read_metadata;
use crate::lib::color::{Color, format, Format};
use std::fs::DirEntry;
use term_grid::{Cell, Direction, Filling, Grid, GridOptions};
use term_size::dimensions;

const ERROR_COLOR: Color = Color::Red;
const ARGUMENTS_COLOR: Color = Color::White;

pub fn error<T: std::fmt::Display>(error_message: T, error_description: T) -> i32 {
    //Call this as: std::process::exit(error(message, description))
    println!(
        "{}: {}",
        error_message,
        format(error_description, ERROR_COLOR, Format::Bold)
    );
    1
}

pub fn helper() {
    println!(
        "Use: lis {}(Optional) {}(Optional)",
        format("[DIRECTORY]", Color::Blue, Format::Bold),
        format("[ARGUMENTS]", Color::Blue, Format::Bold)
    );
    println!("Arguments:");
    println!("--help        │ -h  │ Explain the program use");
    println!("--date        │ -d  │ Show the last mod date of the files");
    println!("--lines       │ -l  │ Force to print the files in lines");
    println!("--no-order    │ -no │ Print the files without ordering");
    println!("--permissions │ -p  │ Shows the files permissions");
    println!("--size        │ -s  │ Shows the files size");
    println!("--show-hidden │ -sh │ Shows the hidden files");
    println!("--all         │ -a  │ Actives all the params");
    println!("--version     │ -v  │ Prints the program version and quit");
    std::process::exit(0)
}

pub fn version() {
    println!("Version: 0.2.3");
    println!("Edition: Development");
    std::process::exit(0)
}

fn print_entries_as_one_per_line(entries: Vec<String>) {
    for entry in entries {
        println!("{}", entry); 
    }
}

fn print_entries_as_grid(higher_len: usize, entries: Vec<String>) {
    let width: usize = match dimensions() {
        Some((size, _)) => size,
        None => 50, //Suposses that the terminal size is 50.
    };
    let spacing = 2;
    let mut grid = Grid::new(GridOptions {
        direction: Direction::LeftToRight,
        filling: Filling::Spaces(spacing),
    });
    for entry in entries {
        grid.add(Cell::from(entry));
    }
    let columns = width / (higher_len + spacing);
    match columns {
        0 => print!("{}", grid.fit_into_columns(1)),
        _ => print!("{}", grid.fit_into_columns(columns)),
    }
}

fn format_entries_names(entries: Vec<DirEntry>, arguments: &Arguments) -> (usize, Vec<String>) {
    const ASCII_FORMATING_CHARS: usize = 11;
    let mut entries_names: Vec<String> = vec![];
    let mut higher_len: usize = 0;
    for entry in &entries {
        let entry_name = format_entry_name(entry);
        let entry_name_len = entry_name.chars().count() - ASCII_FORMATING_CHARS;
        if entry_name_len > higher_len {
            higher_len = entry_name_len;
        }
        entries_names.push(entry_name);
    }
    if arguments.date() || arguments.size() || arguments.permissions() {
        let mut entries_vec: Vec<String> = vec![];
        for (i, entry) in entries.iter().enumerate() {
            let name = &mut entries_names[i];
            let [date, permissions, size] = read_metadata(&entry, &arguments);
            let spacing = {
                let mut result = String::from("");
                let min_len = higher_len - (name.chars().count() - ASCII_FORMATING_CHARS);
                while result.len() < min_len {
                    result.push(' ');
                }
                result
            };
            entries_vec.push(format!(
                "{}{}{}{}{}",
                format(permissions, ARGUMENTS_COLOR, Format::Darker),
                name,
                spacing,
                format(date, ARGUMENTS_COLOR, Format::Darker),
                format(size, ARGUMENTS_COLOR, Format::Darker)
            ));
        }
        (higher_len, entries_vec)
    } else {
        (higher_len, entries_names)
    }
}

pub fn printer(arguments: Arguments, entries: Vec<DirEntry>) {
    let entries = match arguments.no_order() {
        true => entries,
        false => order_entries(entries),
    };
    if entries.is_empty() {
        return println!("This directory is empty");
    }
    let (higher_len, entries_names) = format_entries_names(entries, &arguments);
    match [arguments.size(), arguments.permissions(), arguments.date(), arguments.force_print_as_lines()] {
        [false, false, false, false] => print_entries_as_grid(higher_len, entries_names),
        [_, _, _, _] => print_entries_as_one_per_line(entries_names) 
    }
}
