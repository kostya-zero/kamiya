# Kamiya

Kamiya is a note taking program that works in terminal and allows you to get quick access to your notes.

### Features

- Make notes faster.
- Manage your notes directly in  terminal.
- Lightweight and fast.
- Easy to configure.

### Installation

At this time you need to compiler Kamiya by yourself.
There a small guide to how to get started:

1. Install stable Rust Toolchain with `rustup` (if you already have `rustc` and `cargo`, you can skip this step).
2. Clone this repository.
3. Go to cloned repository directory in terminal and run `cargo build --release`. It will compile an optimized version for you.
4. Place executable in `target/release/` to the directory which exists in `PATH`.

To check which directory are in `PATH`, use this command:

```shell
echo $PATH
```

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

