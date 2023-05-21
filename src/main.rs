pub mod arguments;
pub mod print;
pub mod lib {
    pub mod color;
    pub mod entries {
        pub mod formating;
        pub mod metadata;
        pub mod types;
    }
}
use arguments::Arguments;
use print::printer;

fn main() {
    let (arguments, entries) = Arguments::interpreter();
    printer(arguments, entries);
}
