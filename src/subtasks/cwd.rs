use std::env;

pub struct Cwd {}

impl Cwd {
    pub(crate) fn get() -> String {
        env::current_dir().unwrap().to_str().unwrap().to_owned()
    }

    pub(crate) fn print() {
        print!("{}", Cwd::get());
    }
}
