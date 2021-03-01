use crate::logger::*;
use clap::ArgMatches;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct ModulesConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<Module>>,
}

#[derive(Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub include_in_coverage: Option<bool>,
}

pub struct ModulesListLoader;

impl ModulesListLoader {
    pub fn load(&self, matches: &ArgMatches) -> Result<Vec<Module>, &'static str> {
        let modules: ModulesConfig = self.extract_module_config(matches);
        return match modules.modules {
            Some(list) => Ok(list),
            _e => Err("Problem opening the file: {:?}"),
        };
    }

    fn extract_module_config(&self, matches: &ArgMatches) -> ModulesConfig {
        log_subtask_step("Loading modules list");
        let input = matches.value_of("file").unwrap();
        log_step_with_content("File path =>", input);
        let contents = fs::read_to_string(input).expect("Something went wrong parsin the file");
        log_step_string("Contents =>\n", &contents);
        return serde_json::from_str(&*contents).unwrap();
    }
}
