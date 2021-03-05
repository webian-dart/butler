use crate::logger::log_step;
use crate::subtasks::modules_list_loader::Module;
use std::fs;
use crate::paths::Paths;

pub(crate) struct ClearLastCoverageFiles;

impl ClearLastCoverageFiles {
    pub(crate) fn run(&self) -> Result<(), String> {
        let path = Paths::normalize_for_os(&"./reports/coverage".to_string());
        log_step(format!("Clearing Last Coverage Files at: {}", path).as_str());
        fs::remove_dir_all(path);
        Ok(())
    }
}
