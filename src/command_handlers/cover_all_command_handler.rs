use crate::logger::*;
use crate::subtasks::add_uncovered_files_to_tests_task::AddUncoveredFilesToTestsTask;
use crate::subtasks::build_coverage_report::BuildCoverageReport;
use crate::subtasks::clear_last_coverage_files::ClearLastCoverageFiles;
use crate::subtasks::modules_list_loader::*;
use crate::subtasks::post_coverage_clean_up::PostCoverageCleanUp;
use clap::ArgMatches;
use std::fs;
use std::process::Command;

pub struct CoverAllCommandHandler;

impl CoverAllCommandHandler {
    pub fn handle(&self, matches: &ArgMatches) -> Result<(), String> {
        let name = "Test Coverage for all modules";
        if cfg!(windows) {
            return Result::Err("Test coverall is not available on Windows at the moment.".to_string());
        }
        log_task_starting(name);
        let loading_result = ModulesListLoader {}.load(matches);
        let result = match loading_result {
            Ok(vector) => self.process_modules(&vector),
            Err(errorMessage) => self.process_module(),
        };
        log_task_done(name);
        Command::new("kill");
        result
    }

    fn process_modules(&self, modules: &Vec<Module>) -> Result<(), String> {
        let to_cover_modules = modules
            .iter()
            .filter(|module| {
                return match module.include_in_coverage {
                    Some(true) => true,
                    None => true,
                    _ => false,
                };
            })
            .collect::<Vec<_>>();
        ClearLastCoverageFiles {}.run();
        AddUncoveredFilesToTestsTask {}.run(&to_cover_modules);
        fs::create_dir_all("./reports/coverage");
        BuildCoverageReport {}.run(&to_cover_modules);
        PostCoverageCleanUp {}.run(&to_cover_modules);
        Ok(())
    }

    fn process_module(&self) -> Result<(), String> {
        let root_module = Module {
            name: String::new(),
            include_in_coverage: Some(true),
        };
        self.process_modules(&vec![root_module])
    }
}
