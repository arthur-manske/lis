mod arguments;
mod printer;
use crate::arguments::Arguments;
use crate::printer::print_file;
use colored::{Color, Colorize};
use std::fs::DirEntry;
const PATH_COLOR: Color = Color::BrightBlue;


fn main() {
    let all_arguments: (Arguments, String) = Arguments {
        show_hidden: false,
        perm: false,
        date: false,
        size: false,
        no_ordering: false,
    }
    .interpreter();
    let arguments: Arguments = all_arguments.0;
    let path: String = all_arguments.1;
    let entries: Vec<DirEntry> = arguments.analyze_entries(&path);
    println!(
        "The content of the directory: {} is:",
        path.color(PATH_COLOR)
    );
    for entry in entries {
        print_file(&entry, arguments.clone());
    }
}
