#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate include_dir;
extern crate glob;
extern crate yaml_rust;

mod command_handlers;
mod logger;
mod logger_tags;
mod paths;
mod shell_like_commands;
mod subtasks;
mod butler;

use crate::butler::*;

fn main() {
    Butler {}.run();
}
