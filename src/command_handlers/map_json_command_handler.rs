use crate::logger::*;
use crate::logger::*;
use crate::shell_like_commands::pwd;
use clap::ArgMatches;
use std::borrow::Borrow;
use std::convert::TryInto;
use std::process::Command;

pub struct MapJsonCommandHandler;

const DEFAULT_FILTER: &str = "**/json_mappers/**";
const LIB_FILTER: &str = "./lib/**";
const TASK_NAME: &str = "Generate Json Mappings";

impl MapJsonCommandHandler {
    pub fn handle(&self, matches: &ArgMatches) -> Result<(), String> {
        let mut command_args = vec!["pub", "run", "build_runner", "build", "--build-filter"];
        let filter = self.grab_filter(matches);
        log_step_string(TASK_NAME, &filter);
        command_args.push(&filter);
        pwd();
        let script_run_result = Command::new("flutter").args(command_args).status();
        Command::new("kill");
        let result = match script_run_result {
            Ok(_) => Ok(()),
            Err(err) => log_error(err.to_string().borrow()),
        };
        log_task_done(TASK_NAME);
        result
    }

    fn grab_filter(&self, matches: &ArgMatches) -> String {
        let run_for_all_in_lib_dir = matches.is_present("lib");
        let mut filter = matches.value_of("filter").or(Some("")).unwrap().to_owned();
        filter = if !filter.is_empty() {
            filter
        } else if run_for_all_in_lib_dir {
            LIB_FILTER.to_string()
        } else {
            DEFAULT_FILTER.to_string()
        };
        return filter;
    }
}
