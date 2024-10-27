use colorful::{Colorful, RGB};
use regex_lite::Regex;

/// Parses the rgb color regex and commands inside of the config.
pub struct Parser;

impl Parser {
    /// Parses RGB colors in the config and displays them.
    /// For example: `rgb["Hello, I'm red!", 255, 0, 0]` displays `Hello, I'm red!` as a red color.
    pub fn parse_color(cfg: &mut String) {
        // If there's no rgb pattern, skip creating the regex.
        if !cfg.contains("rgb[\"") {
            return;
        }

        let regex = Regex::new(r#"rgb\["(.*?)",\s*(\d+),\s*(\d+),\s*(\d+)\]"#)
            .expect("[ERROR] Failed creating Regex!");
        let mut cfg_clone = cfg.to_owned();
        for capture in regex.captures_iter(cfg) {
            let content = capture.get(0).unwrap().as_str();
            let text = capture.get(1).unwrap().as_str();
            let r = capture
                .get(2)
                .expect("[ERROR] Failed getting red channel!")
                .as_str()
                .parse::<u8>()
                .expect("[ERROR] Failed parsing red as u8!");
            let g = capture
                .get(3)
                .expect("[ERROR] Failed getting green channel!")
                .as_str()
                .parse::<u8>()
                .expect("[ERROR] Failed parsing green as u8!");
            let b = capture
                .get(4)
                .expect("[ERROR] Failed getting blue channel!")
                .as_str()
                .parse::<u8>()
                .expect("[ERROR] Failed parsing blue as u8!");
            cfg_clone = cfg_clone.replace(content, &text.color(RGB::new(r, g, b)).to_string());
        }

        *cfg = cfg_clone;
    }
}
