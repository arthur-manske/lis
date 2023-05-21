#!/bin/bash

if ! command -v cargo &> /dev/null; then
  echo "Error: 'cargo' command not found. Before compiling the project, you need to install rust and cargo."
  exit 1
fi

user_prompt_is_yes() {
  while true; do
    read -p "$1 [y/n]: " yn
    case $yn in
      [Yy]* ) return 0;;
      [Nn]* ) return 1;;
      * ) echo "Please answer yes or no.";;
    esac
  done
}

ask_for_completion() {
  if user_prompt_is_yes "Do you want to add the ZSH completion?"; then
  	echo "Adding the completion..."
  	sudo cp -r "Scripts/.AutoCompletions/zsh/_lis" "/usr/share/zsh/vendor-completions/"
  fi
}

if ! user_prompt_is_yes "Do you want to install the program?"; then
  ask_for_completion
  exit 0
fi

if user_prompt_is_yes "Do you want to install the program on system-wide?"; then
  sudo cargo install --path . --root="/usr/local" --force
  ask_for_completion
else
  cargo install --path . --root="~/.local/bin" --force
fi
