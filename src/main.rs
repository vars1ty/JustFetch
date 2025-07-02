#![feature(duration_millis_float)]
#![feature(optimize_attribute)]

use crate::utils::Utils;
use std::{ffi::CString, time::Instant};

mod parser;
mod utils;

unsafe extern "C" {
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
                r#"[JustFetch]: Commands
    --elapsed : Displays how long it took to fetch the information.
    --raw     : Skips bash command injection support within the config, like $(whoami)."#
            );
            return;
        }

        let mut now = if self.is_arg_present("--elapsed") {
            Some(Instant::now())
        } else {
            None
        };

        let result = Utils::print();
        if self.is_arg_present("--raw") || !result.contains("$(") {
            println!("{result}");
        } else {
            let cstr = CString::new(format!("echo \"{result}\""))
                .expect("[ERROR] Failed creating CString out of result content!");
            let cstr_ptr = cstr.into_raw();

            unsafe {
                system(cstr_ptr);
                drop(CString::from_raw(cstr_ptr));
            }
        }

        let Some(now) = now.take() else {
            return;
        };

        let elapsed = now.elapsed();
        println!(
            "[JustFetch]: Took {}ms, {elapsed:.2?} in total",
            elapsed.as_millis_f32()
        );
    }

    /// Checks if the specified argument has been passed to the process.
    fn is_arg_present(&mut self, arg: &str) -> bool {
        self.args.iter().any(|defined_arg| defined_arg == arg)
    }
}
