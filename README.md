# Lis - Rust implementation of ls command
Lis is a simple Rust project created as an alternative to the ls command, capable of ordering files and directories, differentiating symbolic links, files with the permissions to execute and directories by color.

## Usage
`
$ lis [FLAGS] [OPTIONS]
`
## Flags
    -h or --help: Prints help information and then quit
    -v or --version: Prints the version of the program and then quit
    -l or --lines: Force to print the files in lines instead of in a grid
    -no or --no-order: Shows files and directories without sorting them
    -sh or --show-hidden: Shows hidden files and directories
    -s or --size: Sorts files and directories by size
    -d or --date: Sorts files and directories by date
    -p or --permissions: Shows file permissions
    -a or --all: Same as '--permissions' + '--date' + '--size' + '--show-hidden'

## Examples

### Show no hidden files and directories:
`$ lis`

### Shows all files and directories:
`$ lis -sh`

### Shows all files and directories with size:
`$ lis -s -sh`

### Shows all files and directories, including hidden ones, with date, in the ~/Downloads directory:
`$ lis -sh -d ~/Downloads`

## Installation

To install the program on your system, you need to have Rust installed. If you don't have it yet, you can download it from the official website at https://www.rust-lang.org/tools/install.
Once Rust is installed, you can download the source code and navigate to the directory where it is located. From there, you can run the following command to install the program with a guide:

`$ ./Script/Installation.sh`

This what you need to know to properly compile the program and to install it with this script. You can then use the program by running its name in the terminal.

## Contributing
If you want to contribute to Lis, feel free to submit a pull request. Any contributions are greatly appreciated!

## Warning
This project is not my main project, so be warned that new versions may take longer than usual to be released.

## License
### BSD 3-Clause License. See the complete [license](LICENSE) for details.
