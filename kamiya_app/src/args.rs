use clap::{value_parser, Arg, ArgAction, Command};

pub fn args() -> Command {
    Command::new("kamiya")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("take").about("Create a new note.").args([
                Arg::new("content")
                    .help("The contents of the note.")
                    .num_args(1)
                    .required(false)
                    .value_parser(clap::value_parser!(String)),
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .help("Name of the note.")
                    .num_args(1)
                    .required(false)
                    .default_value("")
                    .value_parser(clap::value_parser!(String)),
                Arg::new("description")
                    .short('d')
                    .long("desc")
                    .help("Description for new note.")
                    .num_args(1)
                    .required(false)
                    .default_value("")
                    .value_parser(value_parser!(String)),
            ]),
            Command::new("add")
                .about("Save content of file as note.")
                .args([
                    Arg::new("filename")
                        .help("Path to file.")
                        .num_args(1)
                        .required(false)
                        .default_value("")
                        .value_parser(clap::value_parser!(String)),
                    Arg::new("name")
                        .help("Name of new note.")
                        .short('n')
                        .long("name")
                        .num_args(1)
                        .required(false)
                        .default_value("")
                        .value_parser(clap::value_parser!(String)),
                ]),
            Command::new("desc")
                .about("Add description to note.")
                .args([
                    Arg::new("name")
                        .help("Name of note.")
                        .short('n')
                        .long("name")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    Arg::new("desc")
                        .help("Description for note.")
                        .short('d')
                        .long("desc")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ]),
            Command::new("rename").about("Change name of note.").args([
                Arg::new("old_name")
                    .short('o')
                    .long("old")
                    .required(true)
                    .help("Old note name.")
                    .value_parser(value_parser!(String)),
                Arg::new("new_name")
                    .short('n')
                    .long("new")
                    .required(true)
                    .help("New note name.")
                    .value_parser(value_parser!(String)),
            ]),
            Command::new("get")
                .about("Get the contents of a note from the storage.")
                .arg(
                    Arg::new("name")
                        .help("Name of note to read.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("open").about("Open note in your editor.").arg(
                Arg::new("name")
                    .help("Name of note to open.")
                    .num_args(1)
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            ),
            Command::new("editor")
                .about("Set editor for editing notes or check which are using now.")
                .arg(
                    Arg::new("editor")
                        .help("Editor to use")
                        .value_parser(value_parser!(String))
                        .default_value("")
                        .num_args(1),
                ),
            Command::new("template")
                .about("Template for new notes.")
                .arg(
                    Arg::new("template")
                        .help("New template.")
                        .value_parser(value_parser!(String))
                        .default_value("")
                        .num_args(1),
                ),
            Command::new("delete")
                .about("Delete a note from the storage.")
                .arg(
                    Arg::new("name")
                        .help("Name of the note to be deleted.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("search")
                .about("Search for notes by name.")
                .arg(
                    Arg::new("pattern")
                        .help("Part or full name of note.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("list").about("Get a list of the notes in the storage."),
            Command::new("save")
                .about("Save note from storage as file.")
                .args([
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .help("Name of note.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                    Arg::new("filename")
                        .short('f')
                        .long("filename")
                        .help("Name of file.")
                        .num_args(1)
                        .value_parser(clap::value_parser!(String)),
                ]),
            Command::new("export").about("Export database.").arg(
                Arg::new("path")
                    .help("Path where database will be saved.")
                    .short('p')
                    .long("path")
                    .value_parser(value_parser!(String))
                    .default_value("kamiya_exported.json"),
            ),
            Command::new("import")
                .about("Import notes from new database.")
                .args([
                    Arg::new("filename")
                        .help("Path to database.")
                        .short('f')
                        .long("file")
                        .num_args(1)
                        .default_value("kamiya_exported.yml")
                        .value_parser(clap::value_parser!(String)),
                    Arg::new("replace")
                        .help("Replace if note with the same name exists or not.")
                        .short('r')
                        .long("replace")
                        .required(false)
                        .action(ArgAction::SetTrue),
                    Arg::new("interactive")
                        .help("Ask what to do if note with same name exists.")
                        .short('i')
                        .long("interactive")
                        .required(false)
                        .action(ArgAction::SetTrue),
                ]),
        ])
}
