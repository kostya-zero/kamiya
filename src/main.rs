use std::{process::exit, path::Path};
use actions::Actions;
use clap::{Command, Arg};
use config::Manager;
use crate::term::Term;

mod config;
mod actions;
mod utils;
mod term;

fn cli() -> Command {
    Command::new("kamiya")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommands([
            Command::new("take")
            .about("Create a new note.")
            .args([
                Arg::new("content")
                .short('c')
                .long("content")
                .help("The contents of the note.")
                .required(true)
                .value_parser(clap::value_parser!(String)),

                Arg::new("name")
                .short('n')
                .long("name")
                .help("Name of the note.")
                .required(false)
                .default_value("")
                .value_parser(clap::value_parser!(String))
            ]),

            Command::new("record")
                .about("Save content of file as note.")
                .args([
                    Arg::new("name")
                    .help("Name of new note.")
                    .short('n')
                    .long("name")
                    .required(false)
                    .default_value("")
                    .value_parser(clap::value_parser!(String)),
                    
                    Arg::new("filename")
                    .help("Path to file.")
                    .short('f')
                    .long("filename")
                    .required(true)
                    .default_value("")
                    .value_parser(clap::value_parser!(String))
                ]),

            Command::new("list")
            .about("Get a list of the notes in the storage."),

            Command::new("save")
                .about("Save note from storage as file.")
                .args([
                    Arg::new("name")
                      .short('n')
                      .long("name")
                      .help("Name of note.")
                      .value_parser(clap::value_parser!(String)),

                    Arg::new("filename")
                        .short('f')
                        .long("filename")
                        .help("Name of file.")
                        .value_parser(clap::value_parser!(String))
                ]),

            Command::new("get")
                .about("Get the contents of a note from the storage.")
                .arg(
                    Arg::new("name")
                    .help("Name of note to read.")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                ),

            Command::new("edit")
                .about("Edit the note using the editor.")
                .arg(
                    Arg::new("name")
                    .help("Name of the note to edit.")
                    .required(true)
                    .value_parser(clap::value_parser!(String))
                ),

            Command::new("rm")
            .about("Delete a note from the storage.")
            .arg(
                Arg::new("name")
                .help("Name of the note to be deleted.")
                .required(true)
                .value_parser(clap::value_parser!(String))
            ),
        ])
}

fn main() {
    if !Manager::check_config() {
        Term::work("Generating new database...");
        Manager::make_default();
        Term::success("Done.");
    }
    let args = cli().get_matches();
    match args.subcommand() {
        Some(("take", _sub)) => {
            let content: String = _sub.get_one::<String>("content").expect("Cannot read argument content.").to_string();
            let name: String = _sub.get_one::<String>("name").expect("Cannot read argument content.").to_string();

            if content.is_empty() {
                Term::fatal("You cannot take a note with empty content.");
                exit(1);
            }

            Actions::take(&content, &name);
        },
        Some(("record", _sub)) => {
            let filename: String = _sub.get_one::<String>("filename").expect("Cannot read argument content.").to_string();
            let name: String = _sub.get_one::<String>("name").expect("Cannot read argument content.").to_string();

            if filename.is_empty() {
                Term::fatal("You give no path to file.");
                exit(1);
            }

            Actions::record(filename.as_str(), name.as_str());
       },
        Some(("list", _sub)) => {
            Actions::list();
        },
        Some(("edit", _sub)) => {
            let name: String = _sub.get_one::<String>("name").expect("Cannot read argument content.").to_string();

            if name.is_empty(){
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::edit(&name);
        },
        Some(("save", _sub)) => {
            let name: String = _sub.get_one::<String>("name").expect("Cannot read argument content.").to_string();
            let filename: String = _sub.get_one::<String>("filename").expect("Cannot read argument content.").to_string();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            if filename.is_empty() {
                Term::fatal("Bad name for file!");
                exit(1);
            }

            if Path::new(&filename).exists() {
                Term::fatal("Same file already exists in file system.");
                exit(1);
            }

            Actions::save(&name, &filename);
        },
        Some(("get", _sub)) => {
            let name: String = _sub.get_one::<String>("name").expect("Cannot read argument content.").to_string();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::get(&name);
        },
        Some(("rm", _sub)) => {
            let name: String = _sub.get_one::<String>("name").expect("Cannot read argument content.").to_string();
            
            if name.is_empty() {
                Term::fatal("You didn't pass a name to search for.");
                exit(1);
            }

            Actions::rm(&name);
        }
        _ => Term::fatal("Unknown command! Use argument '--help' to get full list of available commands.")
    }
}

