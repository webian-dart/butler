use crate::logger::{log_step, log_step_with_content, log_subtask_step};
use std::fs::File;
use std::io::Read;
use yaml_rust::{Yaml, YamlLoader};

pub struct ReadPubSpec;

impl ReadPubSpec {
    pub fn read(&self, path: &String) -> Result<Yaml, &'static str> {
        let filePath = format!("{}pubspec.yaml", path);
        log_step_with_content("Reading PubSpec", filePath.as_str());
        log_subtask_step(filePath.as_str());
        let mut file = File::open(filePath).expect("Unable to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Unable to read file");
        print!("{}", contents);
        let docs = YamlLoader::load_from_str(&contents).unwrap();
        let doc = docs[0].to_owned();
        println!(
            "Module name for pubspec it: {:?}",
            doc["name"].as_str().unwrap()
        );
        Ok(doc)
    }
}
