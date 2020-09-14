use directories::BaseDirs;
use std::path::{Component, PathBuf};

pub fn norm_path(path: &PathBuf) -> PathBuf {
    let mut p = PathBuf::new();

    for component in path.components() {
        match component {
            Component::RootDir => p.push("/"),
            Component::Normal(name) => match name.to_str().unwrap() {
                "~" => match BaseDirs::new() {
                    Some(dirs) => p.push(dirs.home_dir()),
                    None => {}
                },
                _ => p.push(name),
            },
            _ => {}
        }
    }

    p
}
