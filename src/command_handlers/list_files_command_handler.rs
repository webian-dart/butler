use crate::logger::*;
use crate::shell_like_commands::pwd;
use crate::subtasks::collect_all_dart_files::CollectAllDartFiles;
use clap::ArgMatches;
use std::borrow::Borrow;
use std::process::Command;
use std::path::Path;
use crate::paths::Paths;

pub struct ListFilesCommandHandler;

const TASK_NAME: &str = "Listing files";
const DEFAULT_PATTERN: &str = "**/lib/**/*.dart";

impl ListFilesCommandHandler {
    pub fn handle(&self, matches: &ArgMatches) -> Result<(), String> {
        let path = Paths::normalize_for_os(&DEFAULT_PATTERN.to_owned());
        log_task_starting(TASK_NAME);
        log_step_with_content("Files path:", path.as_str());
        let pattern = matches
            .value_of("pattern")
            .or(Some(path.as_str()))
            .unwrap();
        let paths = CollectAllDartFiles.run(pattern);
        self.printout(&paths);
        Ok(())
    }

    fn printout(&self, paths: &Vec<String>) {
        println!("Files count: {:?}", paths.len());
        for path in paths {
            println!("File: {:?}", path);
        }
    }
}
