# List - Implementação do comando ls em Rust

List é um projeto Rust simples criado como uma alternativa ao comando ls, capaz de ordenar arquivos e diretórios, diferenciar links simbólicos e diretórios por cor e adicionar '@' a links simbólicos e '/' a diretórios.
## Uso

`$ list [FLAGS] [OPTIONS]`
## Flags

    -h ou --help: Mostra informações de ajuda
    -a ou --all: Mostra arquivos e diretórios ocultos
    -no ou --no-order: Mostra arquivos e diretórios sem ordená-los
    -sh ou --show-hidden: Mostra arquivos e diretórios ocultos
    -s ou --size: Ordena arquivos e diretórios por tamanho
    -d ou --date: Ordena arquivos e diretórios por data
    -p ou --perm: Mostra as permissões do arquivo

## Exemplos
### Mostrar nenhum arquivo ou diretório oculto:

`$ list`
### Mostrar todos os arquivos e diretórios:

`$ list -sh`
### Mostrar todos os arquivos e diretórios com tamanho:

`$ list -s -sh`
### Mostrar todos os arquivos e diretórios, incluindo os ocultos, com data, no diretório ~/Downloads:

`$ list -sh -d ~/Downloads`
## Instalação

Para instalar o programa em seu sistema, você precisa ter Rust instalado. Se você ainda não o tem, pode baixá-lo no site oficial em https://www.rust-lang.org/tools/install.
Depois de instalar o Rust, você pode baixar o código fonte e navegar até o diretório onde ele está localizado. A partir daí, você pode executar o seguinte comando para instalar o programa:

`$ sudo cargo install --path . --root=/usr/local/`

Isso irá compilar o programa e instalá-lo no diretório /usr/local/bin, que deve estar no PATH do seu sistema. Você pode então usar o programa executando seu nome no terminal.
## Contribuição

Se você quiser contribuir para o List, sinta-se à vontade para enviar uma solicitação de pull request. Todas as contribuições são muito apreciadas!
## Aviso

Este projeto não é o meu projeto principal, então esteja ciente de que as novas versões podem levar mais tempo do que o normal para serem lançadas.
## Licença

Licença BSD 3-Clause. Veja [LICENSE](LICENSE) para mais detalhes.
