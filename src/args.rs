use clap::{value_parser, Arg, Command};

pub fn cli() -> Command {
    Command::new("kamiya")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommands([
            Command::new("take").about("Create a new note.").args([
                Arg::new("content")
                    .short('c')
                    .long("content")
                    .help("The contents of the note.")
                    .num_args(1)
                    .required(true)
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
                    .value_parser(value_parser!(String))
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
            Command::new("record")
                .about("Save content of file as note.")
                .args([
                    Arg::new("name")
                        .help("Name of new note.")
                        .short('n')
                        .long("name")
                        .num_args(1)
                        .required(false)
                        .default_value("")
                        .value_parser(clap::value_parser!(String)),
                    Arg::new("filename")
                        .help("Path to file.")
                        .short('f')
                        .long("filename")
                        .num_args(1)
                        .required(true)
                        .default_value("")
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
            Command::new("editor")
                .about("Set editor for edeiting notes or check which are using now.")
                .arg(
                    Arg::new("editor")
                        .help("Editor to use")
                        .value_parser(value_parser!(String))
                        .default_value(""),
                ),
            Command::new("list").about("Get a list of the notes in the storage."),
            Command::new("db").about("Show information about database status."),
            Command::new("search")
                .about("Search for notes by name.")
                .arg(
                    Arg::new("pattern")
                        .help("Part or full name of note.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
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
            Command::new("get")
                .about("Get the contents of a note from the storage.")
                .arg(
                    Arg::new("name")
                        .help("Name of note to read.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("edit")
                .about("Edit the note using the editor.")
                .arg(
                    Arg::new("name")
                        .help("Name of the note to edit.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("rm")
                .about("Delete a note from the storage.")
                .arg(
                    Arg::new("name")
                        .help("Name of the note to be deleted.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("export").about("Export database."),
            Command::new("import")
                .about("Import notes from new database.")
                .arg(
                    Arg::new("filename")
                        .help("Path to database.")
                        .short('f')
                        .long("file")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("copy")
                .about("Copy note content into clipboard.")
                .arg(
                    Arg::new("name")
                        .help("Name of note to copy.")
                        .num_args(1)
                        .required(true)
                        .value_parser(clap::value_parser!(String)),
                ),
            Command::new("insert").about("Insert clipboard content and save it as new note."),
        ])
}
