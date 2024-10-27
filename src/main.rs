use crate::utils::Utils;
use std::{ffi::CString, time::Instant};

mod parser;
mod utils;

extern "C" {
    /// https://en.cppreference.com/w/cpp/utility/program/system
    fn system(cmd: *const i8) -> i32;
}

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

        let result = Utils::print();
        if self.is_arg_present("--raw") {
            println!("{}", result);
        } else {
            unsafe {
                let cstr = CString::new(format!("echo \"{result}\"")).unwrap();
                system(cstr.as_ptr());
            }
        }

        if let Some(now) = now.take() {
            println!("Elapsed (from start to end): {:.2?}", now.elapsed());
        }
    }

    /// Checks if the specified argument has been passed to the process.
    fn is_arg_present(&mut self, arg: &str) -> bool {
        self.args.iter().any(|defined_arg| defined_arg == arg)
    }
}
