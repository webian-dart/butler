use crate::logger::{log_error, log_step, log_step_string};
use crate::subtasks::collect_all_dart_files::CollectAllDartFiles;
use crate::yaml_rust::YamlLoader;
use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::process::Command;
use yaml_rust::YamlEmitter;

pub struct HelloCommandHandler;

impl HelloCommandHandler {
    pub fn handle(&self) -> Result<(), String> {
        self.say_hello()
    }

    fn say_hello(&self) -> Result<(), String> {
        let result = Command::new("echo")
            .args(&["Hello World, I am Buttler! Ready at your service!"])
            .status();
        return match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        };
    }
}
