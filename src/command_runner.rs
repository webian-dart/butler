use std::process::{Command, ExitStatus};
use std::env::current_dir;
use std::fs;

const PLATFORM_COMMAND: &str = if cfg!(unix) { "sh" } else { "cmd" };
const FIRST_ARG: &str = if cfg!(unix) { "-c" } else { "/C" };

pub(crate) struct CommandRunner {
    pub command: String,
    pub path: String
}
impl Default for CommandRunner {
    fn default() -> CommandRunner {
            CommandRunner {
                command: String::from("echo you have not provided a command"),
                path: String::from("")
            }
        }
}

impl CommandRunner {
    pub(crate) fn execute(&self) -> std::io::Result<ExitStatus> {
        return Command::new(PLATFORM_COMMAND)
            .current_dir(&self.path)
            .args(&[FIRST_ARG, &self.command])
            .status()
    }
}

