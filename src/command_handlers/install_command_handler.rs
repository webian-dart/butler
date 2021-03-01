use crate::logger::*;
use crate::subtasks::modules_list_loader::{Module, ModulesListLoader};
use clap::ArgMatches;
use std::process::Command;

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
        println!("{}", module.name);
        let result = Command::new("flutter")
            .current_dir(&format!("./{}", &module.name))
            .args(&["pub", "get"])
            .status()
            .expect(
                &format!("{}{}", "Could not install dependencies for ", module.name).to_owned(),
            );
        return if result.success() {
            Ok(())
        } else {
            Err(result.code().unwrap().to_string())
        };
    }
}
