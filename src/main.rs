#![no_main]

use ansi_rgb::Foreground;
use arguments::Arguments;
use rgb::RGB8;
use std::time::Instant;

mod utils;

/// Setup CLI Arguments.
fn setup_args() -> Arguments {
    let args = arguments::parse(std::env::args()).expect("[ERROR] Failed parsing CLI Arguments!");
    if args.get::<String>("help").is_some() {
        println!(
            r#"[JustFetch]: --red      : 0 to 255
[JustFetch]: --green    : 0 to 255
[JustFetch]: --blue     : 0 to 255
[JustFetch]: --elapsed  : Displays how long it took to fetch the information."#
        );
        std::process::exit(0)
    }

    args
}

/// Checks if the color should be modified or not.
fn has_specified_color(args: &Arguments) -> bool {
    args.get::<u8>("red").is_some()
        || args.get::<u8>("green").is_some()
        || args.get::<u8>("blue").is_some()
}

/// Gets the color specified by the user via the arguments.
fn get_color(args: Arguments) -> RGB8 {
    const FALLBACK: u8 = 255;
    let r = args.get::<u8>("red").unwrap_or(FALLBACK);
    let g = args.get::<u8>("green").unwrap_or(FALLBACK);
    let b = args.get::<u8>("blue").unwrap_or(FALLBACK);
    RGB8::new(r, g, b)
}

/// Main startup function.
#[no_mangle]
fn main() {
    let args = setup_args();
    let show_elapsed = args.get::<bool>("elapsed").unwrap_or_default();
    let now = if show_elapsed {
        Some(Instant::now())
    } else {
        None
    };
    if has_specified_color(&args) {
        println!("{}", utils::print().fg(get_color(args)));
    } else {
        // No color to override, skip creating a new instance of RGB8
        println!("{}", utils::print())
    }
    if show_elapsed {
        let elapsed = now.unwrap().elapsed();
        println!("Elapsed (Start » End): {elapsed:.2?}");
    }
}
