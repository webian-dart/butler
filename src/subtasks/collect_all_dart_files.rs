use crate::glob::glob;
use crate::logger::{log_step, log_subtask_step};
use crate::shell_like_commands::pwd;
use crate::subtasks::modules_list_loader::Module;
use glob::Paths;
use std::borrow::Borrow;
use std::fs;
use std::path::Path;

pub(crate) struct CollectAllDartFiles;

impl CollectAllDartFiles {
    pub(crate) fn run(&self, globPattern: &str) -> Vec<String> {
        log_step(format!("Collection all files {}", globPattern).borrow());
        pwd();
        glob(globPattern)
            .expect("Failed to read glob pattern")
            .into_iter()
            .filter(|it| it.is_ok())
            .map(|it| {
                it.unwrap()
                    .as_path()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned()
            })
            .collect::<Vec<_>>()
    }
}
