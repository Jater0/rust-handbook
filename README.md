# Rust Handbook

## Rust Setup

#### Installation

**@reference** [Install Rust - Rust Programming Language (rust-lang.org)](https://www.rust-lang.org/tools/install)

1. MacOS/Linux/Unix-like OS
`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

2. Windows
`@see [url](https://forge.rust-lang.org/infra/other-installation-methods.html)`

#### Project Init

```shell
cargo new <project_name>
cd <project_name>
```

#### Rust In VSCode

![rust-analyzer-extension-in-vscode](README.assets/rust-analyzer-vscode-extensions.png)

-----



## Cargo Handbook Module

### 1. Cargo commands

#### 1.1 Project initialization

`cargo new <project_name>`

#### 1.2 Complie & Run Project

First execute `cargo build` then execute `./target/debug/<project_name>` to run the binary crate.

#### 1.3 Run Project

`cargo run` equals to `cargo build` + `./target/debug/<project_name>`

#### 1.4 Add dependency

`cargo add <dependency_name>@<dependency_version>`



## Hello World

``` rust
fn main() {
    println!("Hello, world!");
}
```



#### hello, Ferris

``` rust
use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn hello_from_ferris() {
  let stdout = stdout();
  let message = String::from("Hello, I am ferris");
  let width = message.chars().count();

  let mut writer = BufWriter::new(stdout.lock());
  say(&message, width, &mut writer).unwrap();
}
```

