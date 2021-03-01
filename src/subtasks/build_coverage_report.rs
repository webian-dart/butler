use crate::logger::*;
use crate::paths::Paths;
use crate::subtasks::modules_list_loader::Module;
use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub struct BuildCoverageReport;

impl BuildCoverageReport {
    pub(crate) fn run(&self, modules: &Vec<&Module>) -> Result<(), String> {
        log_step("Running Coverage Report");
        Paths::cwd();
        let mut modules_lcovs: Vec<String> = Vec::new();
        for module in modules {
            let path = &format!("./{}", &module.name);
            self.generate_lcov(path, &module.name);
            self.remove_exclude_from_lcov(path, &module.name);
            let result = self.read_lcov_report(&module.name);
            match result {
                Ok(lcov_report) => modules_lcovs.push(lcov_report),
                Err(_) => {}
            }
        }
        self.aggregated_reports(modules_lcovs);
        self.generate_html_report();
        Ok(())
    }

    fn aggregated_reports(&self, lcov_files_content: Vec<String>) {
        log_step("Aggregating reports");
        let aggregated: String = lcov_files_content.join("");
        let mut file = File::create("./reports/coverage/lcov.info").unwrap();
        file.write_all(aggregated.as_bytes()).unwrap();
    }

    fn generate_html_report(&self) {
        log_step("Generating Html");
        log_subtask_step("this task depends on lcov genhtml, make sure it is installed");
        log_subtask_step("in the terminal check with command: which genhtml");
        Command::new("genhtml")
            .args(&[
                "reports/coverage/lcov.info",
                "--legend",
                "--sort",
                "--function-coverage",
                "--branch",
                "--show-details",
                "--output",
                "./reports/coverage",
            ])
            .status()
            .expect("Could not generate HTML coverage report");
    }

    fn generate_lcov(&self, path: &String, module_name: &String) -> Result<(), String> {
        log_step_with_content(
            "Generation coverage report for :",
            &format!("{}/{}", Paths::cwd(), module_name),
        );
        Command::new("flutter")
            .current_dir(path)
            .args(&["test", "--coverage"])
            .status()
            .expect(&format!(
                "Could not generate coverage report for '{}'",
                module_name
            ));
        Ok(())
    }

    fn remove_exclude_from_lcov(&self, path: &String, module_name: &String) -> Result<(), String> {
        Command::new("lcov")
            .current_dir(path)
            .args(&[
                "--remove",
                "coverage/lcov.info",
                "-o",
                "coverage/lcov_after_excluded.info",
                "**/*.g.dart",
            ])
            .status()
            .expect(&format!("Failed while excluding files'{}'", module_name));
        Ok(())
    }

    fn read_lcov_report(&self, module_name: &String) -> Result<String, String> {
        log_step("Reading lcov file");
        let path = Paths::absolute_path_to_module(module_name);
        let contents =
            Paths::read_file(&format!("{}/{}", path, "coverage/lcov_after_excluded.info"));
        let re = Regex::new(r"SF:lib/").unwrap();
        let new_token = if module_name.is_empty() {
            "SF:lib/".to_string()
        } else {
            format!("SF:{}/lib/", module_name).to_string()
        };
        let result = re.replace_all(&contents, new_token.as_str()).to_string();
        Ok(result)
    }
}
