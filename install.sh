#!/bin/bash

if ! command -v cargo &> /dev/null; then
  echo "Error: 'cargo' command not found. Please install Rust and Cargo first."
  exit 1
fi

INSTALL_ROOT="local"
INSTALL_SYSTEM=false

# Processa os argumentos passados para o script
while [[ $# -gt 0 ]]; do
  case $1 in
    -h|--help) # Inserir ajuda
      echo "Usage: ./install.sh [-h] [-a] [-r|--root INSTALL_ROOT] [-s|--system]"
      echo ""
      echo "Options:"
      echo "  -h|--help              Show help message"
      echo "  -az|--auto-complete     Add auto-complete to usr/zsh/vendor-completions/_list"
      echo "  -r|--root              Define installation root directory (default is 'local')"
      echo "  -s|--system            Install the program on the system (requires root permissions)"
      exit 0
      ;;
    -az|--auto-complete-zsh) # Adicionar o auto-complete
      echo "Adding auto-complete to usr/zsh/vendor-completions/_list"
      sudo cp "auto-complete/_lis" "/usr/share/vendor-completion"
      exit 0
      ;;
    -r|--root) # Define o diretório de instalação
      INSTALL_ROOT="$2"
      shift
      shift
      ;;
    -s|--system) # Define que é para instalar no sistema
      INSTALL_SYSTEM=true
      shift
      ;;
    *)
      echo "Error: Invalid argument '$1'"
      exit 1
      ;;
  esac
done

# Define as variáveis de instalação
INSTALL_DIR=""
PATH_FILE=""
shell_rc=""
INSTALL_MSG=""

# Define o diretório de instalação e o arquivo de configuração do PATH
if [ "$INSTALL_ROOT" = "local" ]; then
  INSTALL_DIR="$HOME/.local"
  shell_rc=".bashrc"
  PATH_FILE="$HOME/$shell_rc"
else
  INSTALL_DIR="/usr/local"
  PATH_FILE="/etc/environment"
  shell_rc=""
fi

if [ "$INSTALL_SYSTEM" = false ]; then 
  cargo install --path . --root="$INSTALL_DIR"
  if [[ ! "$PATH" =~ "$INSTALL_DIR/bin" ]]; then
    if [ "$SHELL" = "/usr/bin/zsh" ]; then
      shell_rc=".zshrc"
      echo "export PATH=\"$INSTALL_DIR/bin:$PATH\"" >> "$HOME/$shell_rc"
      echo "Please run: 'source ~/.$shell_rc'"
    else
      echo "export PATH=\"$INSTALL_DIR/bin:$PATH\"" >> "$PATH_FILE"
      echo "Please run: 'source ~/.$shell_rc'"
    fi
  fi
  echo "The program is installed on your user."
else
  sudo cargo install --path . --root="/usr/local"
  if ! grep -q "$INSTALL_DIR/bin" "$PATH_FILE"; then
    echo "PATH=\"$INSTALL_DIR/bin:\$PATH\"" >> "$PATH_FILE"
  fi
  echo "The program is installed on the system."
fi
