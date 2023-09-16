use crate::args::args;
use crate::term::Term;
use actions::Actions;
use manager::Manager;
use std::{fs, path::Path, process::exit};

mod actions;
mod args;
mod manager;
mod term;
mod utils;

fn main() {
    if !Manager::check_db() || !Manager::check_config() {
        Manager::make_default();
    }

    if Path::new(&Manager::get_old_config_path()).exists() {
        Term::warn("In Kamiya 0.6.0 database structure has been changed. You old database has been saved as `kamiya.yaml.bak`.");
        Term::warn("You should import all your old notes manually. Sorry for this!");
        let old_database = fs::read_to_string(Manager::get_old_config_path()).unwrap();
        fs::write(Manager::get_old_config_path() + ".bak", old_database).unwrap();
        fs::remove_file(Manager::get_old_config_path()).unwrap();
    }

    let args = args().get_matches();
    match args.subcommand() {
        Some(("take", _sub)) => {
            let content: &str = _sub.get_one::<String>("content").unwrap();
            let mut name: String = _sub.get_one::<String>("name").unwrap().to_string();
            let desc: &str = _sub.get_one::<String>("description").unwrap();

            if content.is_empty() {
                Term::fatal("You cant take a note with empty content.");
                exit(1);
            }

            Actions::take(content, &mut name, desc);
        }
        Some(("add", _sub)) => {
            let filename: &str = _sub.get_one::<String>("filename").unwrap();
            let mut name: String = _sub.get_one::<String>("name").unwrap().to_string();

            if filename.is_empty() {
                Term::fatal("You give no path to file.");
                exit(1);
            }

            Actions::add(filename, &mut name);
        }
        Some(("desc", _sub)) => {
            let name: &str = _sub.get_one::<String>("name").unwrap();

            let desc: &str = _sub.get_one::<String>("desc").unwrap();

            if name.is_empty() {
                Term::fatal("Cannot set description for the void.");
                exit(1);
            }

            Actions::desc(name, desc);
        }
        Some(("rename", _sub)) => {
            let old_name: &str = _sub.get_one::<String>("old_name").unwrap().as_str();
            let new_name: &str = _sub.get_one::<String>("new_name").unwrap().as_str();
            Actions::rename(old_name, new_name);
        }
        Some(("get", _sub)) => {
            let name: &str = _sub.get_one::<String>("name").unwrap();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::get(name);
        }
        Some(("open", _sub)) => {
            let name: &str = _sub.get_one::<String>("name").unwrap();

            if name.is_empty() {
                Term::fatal("You didn't give a name for the note.");
                exit(1);
            }

            Actions::open(name);
        }
        Some(("editor", _sub)) => {
            let editor: &str = _sub
                .get_one::<String>("editor")
                .expect("Cannot read argument content.")
                .as_str();

            Actions::editor(editor);
        }
        Some(("template", _sub)) => {
            let template: &str = _sub.get_one::<String>("template").unwrap();

            Actions::template(template);
        }
        Some(("rm", _sub)) => {
            let name: &str = _sub.get_one::<String>("name").unwrap();

            if name.is_empty() {
                Term::fatal("You didn't pass a name to search for.");
                exit(1);
            }

            Actions::rm(name);
        }
        Some(("search", _sub)) => {
            let pattern: &str = _sub.get_one::<String>("pattern").unwrap();

            Actions::search(pattern);
        }
        Some(("list", _sub)) => {
            Actions::list();
        }
        Some(("save", _sub)) => {
            let name: &str = _sub.get_one::<String>("name").unwrap();
            let filename: &str = _sub.get_one::<String>("filename").unwrap();

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
        Some(("export", _sub)) => {
            let path: &str = _sub.get_one::<String>("path").unwrap();

            Actions::export(path);
        }
        Some(("import", _sub)) => {
            let filename: &str = _sub.get_one::<String>("filename").unwrap();
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
