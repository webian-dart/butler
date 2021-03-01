use crate::logger_tags::*;
use colored::*;

lazy_static! {
    static ref TAGS: LoggerTags = LoggerTags {
        ..Default::default()
    };
}

pub(crate) fn log_error(message: &str) -> Result<(), String> {
    println!("{} {} {}", TAGS.error_prefix, TAGS.app_error, message.red());
    Ok(())
}

pub(crate) fn log_task_starting(name: &str) -> Result<(), String> {
    println!(
        "{} {} {} >> {}",
        TAGS.ok_prefix,
        TAGS.app,
        name.yellow(),
        TAGS.starting
    );
    Ok(())
}

pub(crate) fn log_task_done(name: &str) -> Result<(), String> {
    println!(
        "{} {} {} >> {}",
        TAGS.done_prefix,
        TAGS.app,
        name.yellow(),
        TAGS.done
    );
    Ok(())
}

pub(crate) fn log_step_with_content(name: &str, content: &str) -> Result<(), String> {
    println!("{}..{} {}", TAGS.step_prefix, name.yellow(), content);
    Ok(())
}

pub(crate) fn log_step(name: &str) -> Result<(), String> {
    println!("{}..{}", TAGS.step_prefix, name.yellow());
    Ok(())
}

pub(crate) fn log_subtask_step(name: &str) -> Result<(), String> {
    println!("{}..{}", TAGS.subtask_prefix, name.yellow());
    Ok(())
}

pub(crate) fn log_step_string(name: &str, content: &String) -> Result<(), String> {
    println!("{}..{} {}", TAGS.step_prefix, name.yellow(), content);
    Ok(())
}
