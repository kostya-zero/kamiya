<div align="center" style="text-align:center">
    <img src="imgs/banner.png"/>
    <div align="center">
        <img alt="Latest version" src="https://img.shields.io/gitlab/v/release/kostya-zero/kamiya?style=flat-square&labelColor=303030&color=26A269">
        <img alt="Commit activity" src="https://img.shields.io/github/commit-activity/w/kostya-zero/kamiya?style=flat-square&labelColor=303030&color=3584E4">
        <img alt="Project License" src="https://img.shields.io/github/license/kostya-zero%2Fkamiya?style=flat-square&labelColor=303030&color=FFA348">
        <img alt="Crates.io" src="https://img.shields.io/crates/v/kamiya?style=flat-square&labelColor=303030&color=F8E45C">
    </div>
    <h3>An easy, fast and lightweight tool to create notes in terminal.</h3>
    <a href="https://asciinema.org/a/584606" target="_blank"><img src="https://asciinema.org/a/584606.svg"></a>
</div>

### 🗺️ Navigation

- [Features](#✨-features)
- [Installation](#💾-installation)
- [Usage](#⌨%EF%B8%8F-usage)
- [Configuration](#⚙%EF%B8%8F-configuration)
- [Reporting Problems](#📨-reporting-problems)
- [Contribution](#📦-contribution)
- [Links](#🪐-links)


### ✨ Features

- **Works faster.** Kamiya is designed to be as fast as possible. To do this, we use modern technology.
- **Lightweight database.** Kamiya's database have a small size so it can fit for users who care about file sizes.
- **Easy to use.** Kamiya has nothing hard to do for regular user. Only you need is have any terminal.
- **Easy to backup.** You can backup your notes database by yourself or with integrated tools.
- **Best choice for terminal.** If you are a regular terminal user or using editors like (neo)vim, and you need to make notes - it's a good choice for you.

### 💾 Installation

You have some ways to install Kamiya. Here is some of possible:

##### Install with cargo

1. Install `rustup` and install latest stable Rust toolchain.
2. Install compiler that required for your system.
3. Run `cargo install kamiya` and wait until cargo build Kamiya for you.
4. After build you can run Kamiya from your terminal!

##### Install from releases

1. Go to releases page.
2. Download a version that matches your OS and architecture.
3. Unpack executable file from archive and place it in directory which is exists in `PATH` variable.
4. (**Additional**) Make file executable if it's not.

##### Build from source

1. Install `rustup` and install latest stable Rust toolchain.
2. Clone this repository and enter their directory.
3. Run `cargo build` to compile debug executable, or `cargo build --release` to compile optimized executable. If you have encountered some problems when compiling Kamiya on Windows, try to compile it with WSL.
4. Executable file will appear in `/target/[debug, release]` directory. You can move them to directory which exists in `PATH` variable.

### ⌨️ Usage

##### Structure

Kamiya have an easy usage structure.

```shell
kamiya <command> [argument, ..]
```

Some commands dont require arguments.

##### Take your first note.

To make note, use `take` command and pass `--content` or `-c` argument to give content to save.

```shell
kamiya take -c "This content will be saved."
```

Also, you can provide a name for your note with `--name` or `-n` argument. 
If not, Kamiya will generate the name for you note depending on configuration.

```shell
kamiya take -c "This content will be saved." -n "My Awesome Note"
```

##### View saved notes.

You can check for notes which are saved in your database with `list` command.

```shell
kamiya list
```

##### Search for notes.

You can search for note that you need. 
Just use `search` command and pass the name of note that you need to find.

```shell
kamiya search "My Awesome Note"
```
It's not necessary to specify the full name of the note. 
It's enough to write part of his name, and Kamiya will find notes with similar characters in the name.

```shell
kamiya search Awesome
```

##### Get content of notes.

You can use `get` command to get the content of note by passing the name of note.

```shell
kamiya get "My Awesome Note"
```

##### Get help.

Just use `help` command to get full list of available commands or `--help` to get help about specific command.

```shell
kamiya help        # Get list of available commands.
kamiya take --help # Get help for `take` command.
```

### ⚙️ Configuration

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

### 📨 Reporting problems

If you encoter a problem feel free to report about it on GitHub or GitLab issues of Kamiya.

### 📦 Contribution

We are welcome new contributors to Kamiya!
Feel free to fork this reposotiry, make changes that you want to suggest and create merge request.

### 🪐 Links

- [GitHub Repository](https://github.com/kostya-zero/kamiya)
- [GitLab Repository](https://gitlab.com/kostya-zero/kamiya)

