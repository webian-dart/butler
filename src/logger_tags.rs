use colored::*;

pub(crate) struct LoggerTags {
    pub app: ColoredString,
    pub app_error: ColoredString,
    pub ok_prefix: ColoredString,
    pub done_prefix: ColoredString,
    pub error_prefix: ColoredString,
    pub subtask_prefix: ColoredString,
    pub step_prefix: ColoredString,
    pub starting: ColoredString,
    pub done: ColoredString,
}

impl Default for LoggerTags {
    fn default() -> LoggerTags {
        LoggerTags {
            app: "Buttler >>".green(),
            app_error: "Buttler >>".red(),
            ok_prefix: "\n        ".on_green(),
            done_prefix: "        ".on_color("blue"),
            error_prefix: "        ".on_red(),
            subtask_prefix: "     ".on_cyan(),
            step_prefix: " ".on_yellow(),
            starting: "STARTING ".green(),
            done: "DONE\n".blue(),
        }
    }
}
