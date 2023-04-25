mod arguments;
mod printer;
use crate::arguments::Arguments;
use crate::printer::printer;
pub const PROGRAM_NAME: &str = "list";
pub const VERSION: &str = "0.2.1";

fn main() {
    let (arguments, path) = Arguments::new().interpreter();
    printer(arguments.analyze_entries(&path), arguments, path)
}
