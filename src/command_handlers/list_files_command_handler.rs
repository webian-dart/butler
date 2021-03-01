use crate::logger::*;
use crate::shell_like_commands::pwd;
use crate::subtasks::collect_all_dart_files::CollectAllDartFiles;
use clap::ArgMatches;
use std::borrow::Borrow;
use std::process::Command;

pub struct ListFilesCommandHandler;

const TASK_NAME: &str = "Listing files";
const DEFAULT_PATTERN: &str = "lib/**/*.dart";

impl ListFilesCommandHandler {
    pub fn handle(&self, matches: &ArgMatches) -> Result<(), String> {
        log_task_starting(TASK_NAME);
        let pattern = matches
            .value_of("pattern")
            .or(Some(DEFAULT_PATTERN))
            .unwrap();
        let paths = CollectAllDartFiles.run(pattern);
        self.printout(&paths);
        Ok(())
    }

    fn printout(&self, paths: &Vec<String>) {
        for path in paths {
            println!("{:?}", path);
        }
    }
}
