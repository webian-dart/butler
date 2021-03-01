use crate::command_handlers::cover_all_command_handler::CoverAllCommandHandler;
use crate::command_handlers::coverage_command_handler::CoverageCommandHandler;
use crate::command_handlers::hello_command_handler::HelloCommandHandler;
use crate::command_handlers::install_command_handler::InstallCommandHandler;
use crate::command_handlers::list_files_command_handler::ListFilesCommandHandler;
use crate::command_handlers::map_json_command_handler::MapJsonCommandHandler;
use crate::command_handlers::run_command_handler::RunCommandHandler;
use crate::logger::*;
use clap::{App as Clapp, ArgMatches};
use std::process;
pub struct Buttler {}

impl Buttler {
    pub fn run(&self) {
        println!("Starting Buttler");
        let yaml = load_yaml!("buttler.yml");
        let matches = Clapp::from_yaml(yaml).get_matches();
        if let Err(e) = run(matches) {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}

fn run(matches: ArgMatches) -> Result<(), String> {
    match matches.subcommand() {
        ("install", Some(m)) => InstallCommandHandler {}.handle(m),
        ("coverall", Some(m)) => CoverAllCommandHandler {}.handle(m),
        ("coverage", Some(_)) => CoverageCommandHandler {}.handle(),
        ("hello", Some(_)) => HelloCommandHandler {}.handle(),
        ("run", Some(m)) => RunCommandHandler {}.handle(m),
        ("map-json", Some(m)) => MapJsonCommandHandler {}.handle(m),
        ("ls", Some(m)) => ListFilesCommandHandler {}.handle(m),
        _ => log_error("No Recognised command entered! May be run: \"./buttler --help\""),
    }
}
