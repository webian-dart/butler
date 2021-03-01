use crate::logger::*;
use crate::subtasks::add_uncovered_files_to_tests_task::AddUncoveredFilesToTestsTask;
use crate::subtasks::build_coverage_report::BuildCoverageReport;
use crate::subtasks::clear_last_coverage_files::ClearLastCoverageFiles;
use crate::subtasks::modules_list_loader::*;
use crate::subtasks::post_coverage_clean_up::PostCoverageCleanUp;
use clap::ArgMatches;
use std::fs;
use std::process::Command;

pub struct CoverageCommandHandler;

impl CoverageCommandHandler {
    pub fn handle(&self) -> Result<(), String> {
        let name = "Test Coverage";
        let root_module = Module {
            name: String::new(),
            include_in_coverage: Some(true),
        };
        let result = self.process_modules(&root_module);
        log_task_done(name);
        Command::new("kill");
        result
    }

    fn process_modules(&self, module: &Module) -> Result<(), String> {
        let list = &vec![module];
        ClearLastCoverageFiles {}.run();
        AddUncoveredFilesToTestsTask {}.run(&list);
        fs::create_dir_all("./reports/coverage");
        BuildCoverageReport {}.run(&list);
        PostCoverageCleanUp {}.run(&list);
        Ok(())
    }
}
