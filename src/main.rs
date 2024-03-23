#![no_main]
use crate::utils::Utils;
use std::time::Instant;

mod parser;
mod utils;

/// The help message displayed when you do `./just-fetch --help`.
const HELP_MESSAGE: &str =
    r#"[JustFetch]: --elapsed : Displays how long it took to fetch the information."#;

/// Main startup function.
#[no_mangle]
fn main() {
    JustFetch::init().fetch();
}

/// Main JustFetch structure.
struct JustFetch {
    /// Collected arguments.
    args: Vec<String>,
}

impl JustFetch {
    /// Initializes a new instance of `JustFetch`.
    pub fn init() -> Self {
        Self {
            args: std::env::args().collect(),
        }
    }

    pub fn fetch(&mut self) {
        if self.is_arg_present("--help") {
            println!("{HELP_MESSAGE}");
            return;
        }

        let mut now = if self.is_arg_present("--elapsed") {
            Some(Instant::now())
        } else {
            None
        };

        println!("{}", Utils::print());
        if let Some(now) = now.take() {
            println!("Elapsed (from start to end): {:.2?}", now.elapsed());
        }
    }

    /// Checks if the specified argument has been passed to the process.
    fn is_arg_present(&mut self, arg: &str) -> bool {
        self.args.iter().any(|defined_arg| defined_arg == arg)
    }
}
