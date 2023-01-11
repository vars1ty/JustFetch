use ansi_rgb::Foreground;
use arguments::Arguments;
use rgb::RGB8;

mod info;
mod utils;

/// Setup CLI Arguments.
fn setup_args() -> Arguments {
    let args = std::env::args();
    let args = arguments::parse(args).expect("[ERROR] Failed parsing CLI Arguments!");
    // Help argument.
    if args.get::<String>("help").is_some() {
        println!(
            r#"[JustFetch]: --red   : 0 to 255
[JustFetch]: --green : 0 to 255
[JustFetch]: --blue  : 0 to 255"#
        );
        std::process::exit(0)
    }

    args
}

/// Gets the color specified by the user via the arguments.
fn get_color(args: Arguments) -> RGB8 {
    const DEFAULT: u8 = 255;
    let r = args.get::<u8>("red").unwrap_or(DEFAULT);
    let g = args.get::<u8>("green").unwrap_or(DEFAULT);
    let b = args.get::<u8>("blue").unwrap_or(DEFAULT);
    RGB8::new(r, g, b)
}

/// Main startup function.
fn main() {
    println!("{}", utils::print().fg(get_color(setup_args())));
}
