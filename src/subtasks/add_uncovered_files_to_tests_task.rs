use crate::logger::*;
use crate::subtasks::collect_all_dart_files::CollectAllDartFiles;
use crate::subtasks::cwd::Cwd;
use crate::subtasks::dart_module_package_name::DartModulePackageName;
use crate::subtasks::modules_list_loader::Module;
use include_dir::Dir;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::ops::Add;
use std::process::Command;
use crate::paths::Paths;
use std::borrow::Borrow;

static PROJECT_DIR: Dir = include_dir!("./scripts");

const TASK_NAME: &str = "Adding All files to test for complete coverage";
const DEFAULT_PATTERN: &str = "lib/**/*.dart";

pub struct AddUncoveredFilesToTestsTask;

impl AddUncoveredFilesToTestsTask {
    pub(crate) fn run(&self, modules: &Vec<&Module>) -> Result<(), String> {
        log_subtask_step(TASK_NAME);
        for module in modules {
            let path = Paths::normalize_for_os(&format!("./{}", &module.name));
            if !&module.name.is_empty() {
                self.run_for_module(path.borrow(), &module.name);
            }
        }
        Ok(())
    }

    fn run_for_module(&self, path: &String, module_name: &String) -> Result<(), String> {
        let cwd = Cwd::get();
        log_step_with_content(
            "Registering uncovered files at:",
            &format!("{}/{}", cwd, module_name),
        );
        let package = DartModulePackageName {}.read(module_name);
        let files = CollectAllDartFiles {}.run(DEFAULT_PATTERN);
        if !files.is_empty() {
            self.create_file(package, files);
        } else {
            log_subtask_step("No files found to add to coverage!");
        }
        Ok(())
    }

    fn create_file(&self, package: String, file_paths: Vec<String>) {
        let temp_file_path =  Paths::normalize_for_os(
            &"test/register_for_coverage_tests_untouched_files_test.dart".to_string());
        File::create(temp_file_path).and_then(
            |mut f| {
                let mut content = String::new();
                content.push_str("// Helper file to make coverage work for all dart files\n");
                content.push_str("// ignore_for_file: unused_import\n");
                for raw_path in file_paths {
                    if !raw_path.ends_with(".g.dart") {
                        let clean_path = raw_path.replace("lib/", "");
                        let import_statement =
                            format!("import \"package:{}/{}\";\n", package, clean_path);
                        let normalised = Paths::normalize_for_os(&import_statement);
                        content.push_str(normalised.as_str());
                    }
                }
                content.push_str("\nvoid main(){}\n");
                f.write(content.as_bytes());
                Ok(())
            },
        );
    }
}
