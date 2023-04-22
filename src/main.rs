mod arguments;
mod printer;
use crate::arguments::Arguments;
use crate::printer::printer;
pub const VERSION: &str = "0.2.1";

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
    printer(arguments.analyze_entries(&path), arguments, path)
}
