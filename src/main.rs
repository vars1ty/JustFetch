use crate::utils::Utils;
use std::time::Instant;

mod parser;
mod utils;

/// Main startup function.
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

    /// Fetches system information and prints it out.
    pub fn fetch(&mut self) {
        if self.is_arg_present("--help") {
            println!(
                r#"[JustFetch]: --elapsed : Displays how long it took to fetch the information."#
            );
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
