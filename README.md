<div align="center" style="text-align:center">
    <picture>
        <source media="(prefers-color-scheme: dark)" srcset="imgs/banner-dark.png">
        <source media="(prefers-color-scheme: light)" srcset="imgs/banner-light.png">
        <img alt="Kamiya Banner" height="128" src="imgs/banner-dark.png">
    </picture>
    <div align="center">
        <img alt="GitLab Release (latest by date)" src="https://img.shields.io/gitlab/v/release/kostya-zero/kamiya?style=flat">
        <img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/w/kostya-zero/kamiya?style=flat">
    </div>
    <h3>An easy and lightweight tool to create notes in terminal.</h3>
    <a href="https://asciinema.org/a/584606" target="_blank"><img src="https://asciinema.org/a/584606.svg"></a>
</div>

### Features

- **Works faster.** Kamiya is designed to be as fast as possible. To do this, we use modern technology.
- **Easy to use.** Kamiya has nothing hard to do for regular user. Only you need is have any terminal.
- **Easy to backup.** You can backup your notes database by yourself or with integrated tools.
- **Best choice for terminal.** If you are a regular terminal user or using editors like (neo)vim, and you need to make notes - it's a good choice for you.

### Quick start

#### Install with `cargo` (recommended)

You can use `cargo` to install Kamiya:

```shell
cargo install kamiya
```

#### Install from releases

You can download package from releases and unpack it to the directory that exists in `PATH` variable.

> Warning: At this moment we cant provide packages for Windows and macOS because of cross compiling issues.

> Note: Windows and macOS support are currently in experimental.

### Build from source

1. Install `rustup` and install latest stable Rust toolchain.
2. Clone this repository and enter their directory.
3. Run `cargo build` to compile debug executable, or `cargo build --release` to compile optimized executable.
4. Executable file will appear in `/target/[debug, release]` directory. You can move them to directory which exists in `PATH` variable.

> Note: If you have troubles when compiling Kamiya on Windows, try to compile it with WSL.

### Usage

To get full list of available commands, use `help` or pass no argument.

### Configuration

The first time Kamiya is started, it will create a configuration file in the user's `.config` directory called `kamiya.yml`.
It's a YAML file and has the following structure.

```yml
options:
    name_template: NewNote&i
    editor: nano
entries: []
```
- `name_template` - An example of a title for a new note. Note that the contents of this option must contain `&i`, otherwise the program will give an error.
- `editor` - Which editor will be opened to edit note content.
- `entries` - Array of notes that you have saved. **Do not edit.**

### Reporting problems

If you encoter a problem feel free to report about it on GitHub or GitLab issues of this repository.

### Contribution

Want to suggest fixes? Add new feature? Typo fixes?
Feel free to this repository and send merge request for fixes/feature implementation/code refactoring.

### Links

- GitHub Repository - https://github.com/kostya-zero/kamiya
- GitLab Repository - https://gitlab.com/kostya-zero/kamiya

