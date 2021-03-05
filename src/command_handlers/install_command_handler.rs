use crate::logger::*;
use crate::subtasks::modules_list_loader::{Module, ModulesListLoader};
use clap::ArgMatches;
use std::process::{Command, ExitStatus};
use crate::paths::Paths;
use crate::command_runner::{CommandRunner};

const FLUTTER_INSTALL_COMMAND: &str = "flutter pub get";

pub struct InstallCommandHandler;

impl InstallCommandHandler {
    pub fn handle(&self, matches: &ArgMatches) -> Result<(), String> {
        let loading_result = ModulesListLoader {}.load(matches);
        let result = match loading_result {
            Ok(vector) => self.process_modules(&vector),
            Err(errorMessage) => log_error(errorMessage),
        };
        result
    }

    fn process_modules(&self, modules: &Vec<Module>) -> Result<(), String> {
        log_task_starting("Installing");
        for entry in modules {
            self.install_dependencies_for(entry);
        }
        Ok(())
    }

    fn install_dependencies_for(&self, module: &Module) -> Result<(), String> {
        let path = Paths::normalize_for_os(&format!(".\\{}", &module.name));
        println!("Module name: {}", path);
        let result = CommandRunner {
            command: FLUTTER_INSTALL_COMMAND.to_owned(),
            path
        }.execute();
        return if result.is_ok() {
            Ok(())
        } else {
            Err(format!("{}{}", "Could not install dependencies for ", module.name).to_owned())
        };
    }
}
