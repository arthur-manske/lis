# List - Rust implementation of ls command

List is a simple Rust project created for personal use as an alternative to the ls command. It is capable of ordering files and directories, differentiating symbolic links and directories by color, and adding '@' to symbolic links and '/' to directories.

## Usage:
`list [FLAGS] [OPTIONS]`

## Flags:
    -h or --help: Prints help information
    -a or --all: Shows hidden files and directories
    -no or --no-order: Shows files and directories without sorting them
    -sh or --show-hidden: Shows hidden files and directories
    -s or --size: Sorts files and directories by size
    -d or --date: Sorts files and directories by date
    -p or --perm: Shows file permissions

## Examples:

### Show no hidden files and directories:
`$ list`

### Shows all files and directories:
`$ list -sh`

### Shows all files and directories with size:
`$ list -s -sh`

### Shows all files and directories, including hidden ones, with date, in the ~/Downloads directory:
`$ list -sh -d ~/Downloads`

## Contributing:
If you want to contribute to List, feel free to submit a pull request. Any contributions are greatly appreciated!

## Warning:
This project is not my main project, so be warned that the new versions can take longer then normal to be released
