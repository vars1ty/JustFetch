use colorful::{Colorful, RGB};
use regex_lite::Regex;

/// Parses the rgb color regex and commands inside of the config.
pub struct Parser;

impl Parser {
    /// Parses RGB colors in the config and displays them.
    /// For example: `rgb["Hello, I'm red!", 255, 0, 0]` displays `Hello, I'm red!` as a red color.
    #[optimize(speed)]
    pub fn parse_color(cfg: &mut String) {
        // If there's no rgb pattern, skip creating the regex.
        if !cfg.contains("rgb[\"") {
            return;
        }

        let regex = Regex::new(r#"rgb\["(.*?)",\s*(\d{1,3}),\s*(\d{1,3}),\s*(\d{1,3})\]"#)
            .expect("[ERROR] Failed creating Regex!");
        let mut cfg_clone = cfg.to_owned();
        for capture in regex.captures_iter(cfg) {
            let (raw_content, text_rgba) = capture.extract::<4>();
            let inner_content = text_rgba[0];

            // The regex is limited to RGB decimals of 1-3 in length, so
            // the unwrap_unchecked should be safe.
            unsafe {
                let r = lexical::parse(text_rgba[1]).unwrap_unchecked();
                let g = lexical::parse(text_rgba[2]).unwrap_unchecked();
                let b = lexical::parse(text_rgba[3]).unwrap_unchecked();
                cfg_clone = cfg_clone.replace(
                    raw_content,
                    &inner_content.color(RGB::new(r, g, b)).to_string(),
                );
            }
        }

        *cfg = cfg_clone;
    }
}
