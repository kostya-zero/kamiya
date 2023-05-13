# Kamiya

Kamiya is a note taking program that works in terminal and allows you to get quick access to your notes.

[![asciicast](https://asciinema.org/a/584606.svg)](https://asciinema.org/a/584606?speed=1.5&autoplay=1)

### Features

- Make notes faster.
- Manage your notes directly in  terminal.
- Lightweight and fast.
- Easy to configure.

### Quick start

You can download package from releases and unpack it to the directory that exists in `PATH` variable.

> **Warning:** At this moment we cant provide packages for Windows and macOS because of cross compiling issues.

### Build from source

1. Install `rustup` and install latest stable Rust toolchain.
2. Clone this repository and enter their directory.
3. Run `cargo build` to compile debug executable, or `cargo build --release` to compile optimized executable.
4. Executable file will appear in `/target/[debug, release]` directory. You can move them to directory which exists in `PATH` variable.

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
- `name_template` - An example of a title for a new post. Note that the contents of this parameter must contain `&i`, otherwise the program will give an error.
- `editor` - Which editor will be opened to edit note content.
- `entries` - Array of notes. **Do not edit it.**

### Reporting problems

If you encoter a problem feel free to report about it on GitHub or GitLab issues of this repository.

### Contribution

Want to suggest fixes? Add new feature? Typo fixes? 
Feel free to this repository and send merge request for fixes/feature implementation/code refactoring.

### Links

- GitHub Repository - https://github.com/kostya-zero/kamiya
- GitLab Repository - https://gitlab.com/kostya-zero/kamiya

