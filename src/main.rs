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
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get content. Bad format.");
                    exit(1);
                })
                .to_string();
            let name: String = _sub
                .get_one::<String>("name")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get name. Bad format.");
                    exit(1);
                })
                .to_string();
            let desc: String = _sub
                .get_one::<String>("description")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get description. Bad format.");
                    exit(1);
                })
                .to_string();

            if content.is_empty() {
                Term::fatal("You cant take a note with empty content.");
                exit(1);
            }

            Actions::take(&content, &name, &desc);
        }
        Some(("desc", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get name. Bad format.");
                    exit(1);
                })
                .to_string();

            let desc: String = _sub
                .get_one::<String>("desc")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get description. Bad format.");
                    exit(1);
                })
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
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get note name. Bad format.");
                    exit(1);
                })
                .to_string();
            let new_name: String = _sub
                .get_one::<String>("new_name")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get new name for note. Bad format.");
                    exit(1);
                })
                .to_string();
            Actions::rename(&old_name, &new_name);
        }
        Some(("editor", _sub)) => {
            let editor: String = _sub
                .get_one::<String>("editor")
                .expect("Cannot read argument content.")
                .to_string();

            Actions::editor(&editor);
        }
        Some(("db", _sub)) => {
            Actions::db();
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
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get note name. Bad format.");
                    exit(1);
                })
                .to_string();
            let filename: String = _sub
                .get_one::<String>("filename")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get name for file. Bad format.");
                    exit(1);
                })
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
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get name of note. Bad format.");
                    exit(1);
                })
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
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get name of note to remove. Bad format.");
                    exit(1);
                })
                .to_string();

            if name.is_empty() {
                Term::fatal("You didn't pass a name to search for.");
                exit(1);
            }

            Actions::rm(&name);
        }
        Some(("export", _sub)) => {
            let path: String = _sub
                .get_one::<String>("path")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get path for export. Bad format.");
                    exit(1);
                })
                .to_string();

            Actions::export(&path);
        }
        Some(("import", _sub)) => {
            let filename: String = _sub
                .get_one::<String>("filename")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get file name for import. Bad format.");
                    exit(1);
                })
                .to_string();
            let replace: bool = _sub
                .get_flag("replace");
            let interactive: bool = _sub
                .get_flag("interactive");

            if interactive && replace {
                Term::fatal("`interactive` and `replace` cant be set at the same time.");
                exit(1);
            }

            Actions::import(filename.as_str(), replace, interactive);
        }
        Some(("copy", _sub)) => {
            let name: String = _sub
                .get_one::<String>("name")
                .unwrap_or_else(|| {
                    Term::fatal("Failed to get name of note to copy. Bad format.");
                    exit(1);
                })
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
