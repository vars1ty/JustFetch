#![no_main]
use std::time::Instant;

mod parser;
mod utils;

/// The help message displayed when you do `./just-fetch --help`.
const HELP_MESSAGE: &str = r#"[JustFetch]: --elapsed : Displays how long it took to fetch the information."#;

/// Checks if the specified argument has been passed to the process.
fn is_arg_present(arg: &str) -> Option<String> {
    std::env::args().find(|defined_arg| defined_arg == arg)
}

/// Main startup function.
#[no_mangle]
fn main() {
    if is_arg_present("--help").is_some() {
        println!("{HELP_MESSAGE}");
        return;
    }

    let mut now = if is_arg_present("--elapsed").is_some() {
        Some(Instant::now())
    } else {
        None
    };

    println!("{}", utils::print());
    if let Some(now) = now.take() {
        println!("Elapsed (Start » End): {:.2?}", now.elapsed());
    }
}
