# Minigrep
Minigrep project from [The Rust Programming Language - Chapter 12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

## Usage

#### Case Sensitive
```sh
cargo run -- to poem.txt
```
#### Case Insensitive
```sh
IGNORE_CASE=1 cargo run -- to poem.txt
```

#### Save Output to a File
```sh
IGNORE_CASE=1 cargo run -- to poem.txt
```