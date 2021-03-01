use crate::logger::log_step;
use crate::subtasks::modules_list_loader::Module;
use std::fs;

pub(crate) struct ClearLastCoverageFiles;

impl ClearLastCoverageFiles {
    pub(crate) fn run(&self) -> Result<(), String> {
        log_step("Clearing Last Coverage Files");
        fs::remove_dir_all("./reports/coverage");
        Ok(())
    }
}
