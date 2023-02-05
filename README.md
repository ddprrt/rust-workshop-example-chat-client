# Workshop Repo: Tokio

_NOTE_: This is Work-In-Progress! Please check for updates a day before the
workshop.

This Github repository contains all the source code and examples for the first
homework assessment

## Install Rust

[Rustup](https://rustup.rs) provides you with all the software to compile and
run Rust applications, e.g.

1. Cargo - build tool and package manager
2. `rustfmt` - Auto-formatting tool for Rust code
3. `clippy` - Linting for common mistakes

[and many more](https://rust-lang.github.io/rustup-components-history/).
_Rustup_ also allows you to install different compile targets and multiple
toolchains, as well as keeping your toolchains up to date.

After installing, you should have a set of new command line tools available.

### Verify your Rust installation:

1. Open a Terminal/Shell/Command Line of your choice
2. Check out this repo
3. Navigate to this repository
4. Enter

```bash
$ cargo build
```

5. Open your browser at https://localhost:3000

## Recommended Editor

During the workshop, we will use
[Visual Studio Code](https://code.visualstudio.com/) as editor. It's free, fast
and very extensible. Making yourself familiar with VS Code is highly
recommended.

However, working with VS Code is not required. If you have a preferred editor
with Rust support you're more productive with, please feel free to use whatever
you like. What we highyly recommend though, is checking if your editor has
support for [Rust analyzer](https://rust-analyzer.github.io/).

## Recommended VS Code Extensions

To work effeciently, please install a couple of extensions that help you
developing Rust. _Note_: Please don't install the recommendend Rust extension.
It's outdated and the community decided to move to other tools. You can search
and install VS Code extensions through the menu on the side

We recommend the following extensions:

- [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer).
  This is the main extension for Rust development, with the best language
  support available. _Note_: This extension is also available for other IDEs and
  editors, check out [their website](https://rust-analyzer.github.io/)

- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates).
  This extension helps installing dependencies from crates.io

- [Better TOML](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml).
  TOML is the format that the dependency manager Cargo uses to manage
  dependencies. This extension helps formatting and editing TOML files

- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).
  All Rust code is compiled against LLVM. This extension helps debugging LLVM
  code inside VS Code

- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens).
  Inline errors

## Teaching goals

- Tokio framework: TCP Listeners, TCP Sockets
- Makros, Reader and Writer Extensions

## Tasks

Build a TCP chat client that works with the TCP chat server we developed
together during the course.

1. It should connect to a TCP server (e.g. `localhost:8001`).
2. It reads from `stdin` and writes to the socket
3. It reads from the socket and writes to `stdout`
4. It stops on `exit`

To solve this example, you need to look at the following structs and traits:

- `tokio::io::stdin`
- `tokio::io::AsyncBufReadExt`
- `tokio::io::AsyncWriteExt`
- `tokio::io::BufReader`
- `tokio::io::BufWriter`
- `tokio::net::TcpStream`

NOTE: You need to `flush` to send a line to the server.
