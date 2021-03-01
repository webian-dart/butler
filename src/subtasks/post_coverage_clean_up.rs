use crate::logger::log_step;
use crate::paths::Paths;
use crate::subtasks::modules_list_loader::Module;
use std::fs;

pub(crate) struct PostCoverageCleanUp;

impl PostCoverageCleanUp {
    pub fn run(&self, modules: &Vec<&Module>) -> Result<(), String> {
        log_step("Cleaning up Leftovers");
        for module in modules {
            PostCoverageCleanUp::clean_coverage_directory_in(&module.name);
            PostCoverageCleanUp::clean_tests_untouched_files_file(&module.name);
        }
        Ok(())
    }

    fn clean_coverage_directory_in(module: &String) {
        let path = &format!("{}/coverage", Paths::absolute_path_to_module(&module));
        log_step(&format!("Deleting: {}", path));
        fs::remove_dir_all(path);
    }

    fn clean_tests_untouched_files_file(module: &String) {
        let path = &format!(
            "{}/test/register_for_coverage_tests_untouched_files_test.dart",
            Paths::absolute_path_to_module(module)
        );
        log_step(&format!("Deleting: {}", path));
        fs::remove_file(path);
    }
}
