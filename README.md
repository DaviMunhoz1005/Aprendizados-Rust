# Aprendizados-Rust

Conjunto de Aprendizados e pequenos projetos realizados na linguagem Rust.

## Comandos principais

Os principais comando são usados com base no comando `cargo`.

- Criar um novo projeto:

```bash
$ cargo new nome_do_projeto
 ```

- Compila as modificações que foram feitas:

```bash
$ cargo build
 ```

- Procura por possíveis erros na aplicação, sem gerar o arquivo binário:

```bash
$ cargo check
 ```

- Atualiza as dependências do projeto:

```bash
$ cargo update
 ```

- Compila e executa o projeto:

```bash
$ cargo run
 ```

### Flags

Flags são principalmente usadas para identificar o local que será executado ou para identificar o modo, como por exemplo release ou dev.

- Executa a partir da pasta binária:

```bash
$ cargo run --bin nome_do_binário
 ```

- Compila uma versão Otimizada do projeto:

```bash
$ cargo build --release
 ```

- Executa uma versão Otimizada do projeto:

```bash
$ cargo run --release
 ```

- Cria crate de Biblioteca:

```bash
$ cargo new nome_do_projeto --lib
 ```
