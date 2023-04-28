mod arguments;
mod printer;
use crate::arguments::Arguments;
use crate::printer::printer;
pub const PROGRAM_NAME: &str = "lis";
pub const VERSION: &str = "0.2.2";

fn main() {
    let (arguments, path) = Arguments::new().interpreter();
    printer(arguments.analyze_entries(&path), arguments, &path)
}
