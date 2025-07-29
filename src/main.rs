use std::env;
use std::fmt;
use std::fmt::Display;
use std::io;
use std::path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitStatus;

struct MetaData {
    size: u64,
    files: usize,
}

impl MetaData {
    pub fn examine(&mut self, path: &path::Path) {
        let target_dir = path.join("target");
        self.recursive_examine(target_dir);
    }

    fn recursive_examine(&mut self, target_dir: PathBuf) {
        let Ok(entries) = target_dir.read_dir()
        else {
            return;
        };

        for entry in entries {
            let Ok(entry) = entry
            else {
                continue;
            };

            let inner_path = entry.path();
            if inner_path.is_dir() {
                self.recursive_examine(entry.path());
            }
            if inner_path.is_file() {
                if let Ok(metadata) = inner_path.metadata() {
                    self.size += metadata.len();
                    self.files += 1;
                }
            }
        }
    }
}

impl Display for MetaData {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "files: {}\nsize mb: {:.2}\nsize gb: {:.4}",
            self.files,
            self.size as f64 / 2f64.powi(20),
            self.size as f64 / 2f64.powi(30),
        )
    }
}

fn main() {
    let envs: Vec<String> = env::args().collect();

    let mut root = path::PathBuf::new();
    root.push(envs.get(1).expect("failed to open root directory"));

    let cargo_projects = find_cargo_projects(&root);

    let mut metadata = MetaData { size: 0, files: 0 };
    for project in &cargo_projects {
        metadata.examine(project);
    }
    println!("total projects metadata:\n{metadata}");

    let mut after_data = MetaData { size: 0, files: 0 };
    for project in &cargo_projects {
        let display_path = project.strip_prefix(root.parent().unwrap()).unwrap().display();
        println!("\nclean `{display_path}`??\nenter [y/n]");

        let mut user_input = String::new();
        loop {
            while user_input.is_empty() {
                io::stdin().read_line(&mut user_input).unwrap();
            }

            let selection = user_input.as_str().trim();
            match selection {
                | "y" => {
                    clean_project(project).expect("some sort of issue cleaning the project");
                    break;
                }
                | "n" => {
                    break;
                }
                | _ => {
                    println!("enter only y or n");
                    user_input.clear();
                }
            }
        }
        after_data.examine(project);
    }
    println!("total projects metadata after extra-cleaning:\n{after_data}");
}

fn find_cargo_projects(root: &path::Path) -> Vec<PathBuf> {
    let mut project_paths = Vec::new();
    recursive_find_projects(root, &mut project_paths);
    return project_paths;

    fn recursive_find_projects(path: &path::Path, project_paths: &mut Vec<PathBuf>) {
        let Ok(entries) = path.read_dir()
        else {
            return;
        };

        for entry in entries {
            let Ok(entry) = entry
            else {
                continue;
            };

            let path = entry.path();
            if is_cargo_object(&path) {
                project_paths.push(path);
            }
            else {
                recursive_find_projects(&path, project_paths);
            }
        }
    }
}

fn is_cargo_object(path: &path::Path) -> bool {
    (path.join("Cargo.toml").is_file() || path.join("cargo.toml").is_file()) && path.join("target").is_dir()
}

fn clean_project(path: &path::Path) -> io::Result<ExitStatus> {
    Command::new("cargo").arg("clean").current_dir(path).status()
}
