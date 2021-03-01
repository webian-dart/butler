use std::{env, fs};

pub(crate) struct Paths;

impl Paths {
    pub(crate) fn read_file(path: &String) -> String {
        return fs::read_to_string(path).expect("Something went wrong reading the file");
    }

    pub(crate) fn absolute_path_to_module(module_name: &String) -> String {
        return format!("{}/{}", Paths::cwd(), module_name);
    }

    pub(crate) fn cwd() -> String {
        return env::current_dir().unwrap().to_str().unwrap().to_owned();
    }
}
