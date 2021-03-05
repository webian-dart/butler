use std::{env, fs};
use std::path::{PathBuf, Path};
use path_slash::{PathExt, PathBufExt};
use crate::logger::log_step_with_content;

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

    pub(crate) fn normalize_for_os(path: &String) -> String {
        return if cfg!(unix) {
            let result = Path::new(path).to_slash().expect("Not a valid path");
            log_step_with_content("Unix path:", result.as_str());
            result
        } else {
            let result = PathBuf::from_slash(path)
                .into_os_string().into_string().expect("Not a valid path");
            log_step_with_content("Windows path:", result.as_str());
            result
        }
    }
}
