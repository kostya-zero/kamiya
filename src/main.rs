use crate::args::args;
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
    let args = args().get_matches();
    match args.subcommand() {
        Some(("take", _sub)) => {
            let content: &str = _sub
                .get_one::<String>("content")
                .unwrap()
                .as_str();
            let mut name: String = _sub
                .get_one::<String>("name")
                .unwrap()
                .to_string();
            let desc: &str = _sub
                .get_one::<String>("description")
                .unwrap()
                .as_str();

            if content.is_empty() {
                Term::fatal("You cant take a note with empty content.");
                exit(1);
            }

            Actions::take(content, &mut name, desc);
        }
        Some(("desc", _sub)) => {
            let name: &str = _sub
                .get_one::<String>("name")
                .unwrap()
                .as_str();

            let desc: &str = _sub
                .get_one::<String>("desc")
                .unwrap()
                .as_str();

            if name.is_empty() {
                Term::fatal("Cannot set description for the void.");
                exit(1);
            }

            Actions::desc(name, desc);
        }
        Some(("rename", _sub)) => {
            let old_name: &str = _sub
                .get_one::<String>("old_name")
                .unwrap()
                .as_str();
            let new_name: &str = _sub
                .get_one::<String>("new_name")
                .unwrap()
                .as_str();
            Actions::rename(old_name, new_name);
        }
        Some(("editor", _sub)) => {
            let editor: &str = _sub
                .get_one::<String>("editor")
                .expect("Cannot read argument content.")
                .as_str();

            Actions::editor(editor);
        }
        Some(("db", _sub)) => {
            Actions::db();
        }
        Some(("record", _sub)) => {
            let filename: &str = _sub
                .get_one::<String>("filename")
                .expect("Cannot read argument content.")
                .as_str();
            let name: &str = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .as_str();

            if filename.is_empty() {
                Term::fatal("You give no path to file.");
                exit(1);
            }

            Actions::record(filename, name);
        }
        Some(("list", _sub)) => {
            Actions::list();
        }
        Some(("search", _sub)) => {
            let pattern: &str = _sub
                .get_one::<String>("pattern")
                .expect("Cannot read argument content.")
                .as_str();

            Actions::search(pattern);
        }
        Some(("edit", _sub)) => {
            let name: &str = _sub
                .get_one::<String>("name")
                .expect("Cannot read argument content.")
                .as_str();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::edit(name);
        }
        Some(("save", _sub)) => {
            let name: &str = _sub
                .get_one::<String>("name")
                .unwrap()
                .as_str();
            let filename: &str = _sub
                .get_one::<String>("filename")
                .unwrap()
                .as_str();

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

            Actions::save(name, filename);
        }
        Some(("get", _sub)) => {
            let name: &str = _sub
                .get_one::<String>("name")
                .unwrap()
                .as_str();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::get(name);
        }
        Some(("rm", _sub)) => {
            let name: &str = _sub
                .get_one::<String>("name")
                .unwrap()
                .as_str();

            if name.is_empty() {
                Term::fatal("You didn't pass a name to search for.");
                exit(1);
            }

            Actions::rm(name);
        }
        Some(("export", _sub)) => {
            let path: &str = _sub
                .get_one::<String>("path")
                .unwrap()
                .as_str();

            Actions::export(path);
        }
        Some(("import", _sub)) => {
            let filename: &str = _sub
                .get_one::<String>("filename")
                .unwrap()
                .as_str();
            let replace: bool = _sub.get_flag("replace");
            let interactive: bool = _sub.get_flag("interactive");

            if interactive && replace {
                Term::fatal("`interactive` and `replace` cant be set at the same time.");
                exit(1);
            }

            Actions::import(filename, replace, interactive);
        }
        _ => Term::fatal(
            "Unknown command! Use argument '--help' to get full list of available commands.",
        ),
    }
}
