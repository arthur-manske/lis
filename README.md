# Lis - Rust implementation of ls command

Lis is a simple Rust project created as an alternative to the ls command, capable of ordering files and directories, differentiating symbolic links and directories by color, and adding '@' to symbolic links and '/' to directories.
## Usage
`
$ lis [FLAGS] [OPTIONS]
`
## Flags
    -h or --help: Prints help information
    -no or --no-order: Shows files and directories without sorting them
    -sh or --show-hidden: Shows hidden files and directories
    -s or --size: Sorts files and directories by size
    -d or --date: Sorts files and directories by date
    -p or --permissions: Shows file permissions
    -a same as '--permissions' + '--date' + '--size' + '--show-hidden'

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
Once Rust is installed, you can download the source code and navigate to the directory where it is located. From there, you can run the following command to install the program(I recommend you to read all the parameters valid for the 'install.sh' with '--help'): 

`$ ./install.sh --system`

`$ ./install.sh --auto-complete-zsh` (This one is only if you use ZSH)


This what you need to know to properly compile the program and to install it with this script. You can then use the program by running its name in the terminal.

## Contributing

If you want to contribute to List, feel free to submit a pull request. Any contributions are greatly appreciated!
## Warning

This project is not my main project, so be warned that new versions may take longer than usual to be released.
If you wanna see the plans for our next release, please check it out on [todo](ToDo.md)

## License

### BSD 3-Clause License. See the complete [license](LICENSE) for details.
