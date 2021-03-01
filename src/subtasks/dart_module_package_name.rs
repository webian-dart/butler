use crate::subtasks::read_pubspec::ReadPubSpec;
use std::borrow::Borrow;
use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};

pub struct DartModulePackageName;

impl DartModulePackageName {
    pub fn read(&self, module_dir: &String) -> String {
        let empty = &String::new();
        let path: &String = if module_dir.is_empty() {
            empty
        } else {
            module_dir
        };
        let result = ReadPubSpec {}.read(&path);
        return if result.is_ok() {
            let doc = result.unwrap();
            format!("{}", doc["name"].as_str().unwrap())
        } else {
            "".to_string()
        };
    }
}
