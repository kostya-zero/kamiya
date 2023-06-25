use crate::args::cli;
use crate::term::Term;
use actions::Actions;
use config::Manager;
use std::{path::Path, process::exit};

mod actions;
mod args;
mod config;
mod term;
mod utils;

fn main() {
    if !Manager::check_config() {
        Term::work("Generating new database...");
        Manager::make_default();
        Term::success("Done.");
    }
    let args = cli().get_matches();
    match args.subcommand() {
        Some(("take", _sub)) => {
            let content: String = _sub
                .get_one::<String>("content")
                .expect("Cannot read argument content.")
                .to_string();
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();

            if content.is_empty() {
                Term::fatal("You cannot take a note with empty content.");
                exit(1);
            }

            Actions::take(&content, &name);
        }
        Some(("desc", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();

            let desc: String = _sub
                .get_one::<String>("desc")
                .expect("Cannot read argument content.")
                .to_string();

            if name.is_empty() {
                Term::fatal("Cannot set description for the void.");
                exit(1);
            }

            Actions::desc(&name, &desc);
        }
        Some(("rename", _sub)) => {
            let old_name: String = _sub
                .get_one::<String>("old_name")
                .expect("Cannot read argument content.")
                .to_string();
            let new_name: String = _sub
                .get_one::<String>("new_name")
                .expect("Cannot read argument content.")
                .to_string();
            Actions::rename(&old_name, &new_name);
        }
        Some(("record", _sub)) => {
            let filename: String = _sub
                .get_one::<String>("filename")
                .expect("Cannot read argument content.")
                .to_string();
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();

            if filename.is_empty() {
                Term::fatal("You give no path to file.");
                exit(1);
            }

            Actions::record(filename.as_str(), name.as_str());
        }
        Some(("list", _sub)) => {
            Actions::list();
        }
        Some(("search", _sub)) => {
            let pattern: String = _sub
                .get_one::<String>("pattern")
                .expect("Cannot read argument content.")
                .to_string();

            Actions::search(&pattern);
        }
        Some(("edit", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::edit(&name);
        }
        Some(("save", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();
            let filename: String = _sub
                .get_one::<String>("filename")
                .expect("Cannot read argument content.")
                .to_string();

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
        }
        Some(("get", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::get(&name);
        }
        Some(("rm", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();

            if name.is_empty() {
                Term::fatal("You didn't pass a name to search for.");
                exit(1);
            }

            Actions::rm(&name);
        }
        Some(("export", _sub)) => {
            Actions::export();
        }
        Some(("import", _sub)) => {
            let filename: String = _sub
                .get_one::<String>("filename")
                .expect("Cannot read argument content.")
                .to_string();
            Actions::import(filename.as_str());
        }
        Some(("copy", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .to_string();
            if name.is_empty() {
                Term::fatal("You didn't pass a name to search for.");
                exit(1);
            }

            Actions::copy(&name);
        }
        Some(("insert", _sub)) => {
            Actions::insert();
        }
        _ => Term::fatal(
            "Unknown command! Use argument '--help' to get full list of available commands.",
        ),
    }
}
