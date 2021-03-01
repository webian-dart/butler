use crate::logger::*;
use crate::shell_like_commands::pwd;
use clap::ArgMatches;
use std::borrow::Borrow;
use std::process::Command;

pub struct RunCommandHandler;

impl RunCommandHandler {
    pub fn handle(&self, matches: &ArgMatches) -> Result<(), String> {
        let script = matches.value_of("script").or(Some("")).unwrap();
        let options = matches.value_of("options").or(Some("")).unwrap();
        return if !script.is_empty() {
            self.process_script(script, options);
            Ok(())
        } else {
            log_error("Oops! Nothing to run. Try: ./butler run the_name_of_file_the_bin_folder_without_extension")
        };
    }

    fn process_script(&self, script_name: &str, options: &str) -> Result<(), String> {
        println!("Trying to run: ./bin/{}.dart", script_name.borrow());
        pwd();
        let script_run_result = Command::new("dart")
            .current_dir("./bin/")
            .args(&["run", &format!("{}.dart", script_name), options])
            .status();
        Command::new("kill");
        let result = match script_run_result {
            Ok(_) => Ok(()),
            Err(err) => log_error(err.to_string().borrow()),
        };
        result
    }
}
