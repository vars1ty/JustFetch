use crate::utils::Utils;
use colorful::{Colorful, RGB};
use regex_lite::Regex;

/// Keyword for
const SPLIT_BULK_PLACEHOLDER: &str = " %split%";

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

    /// Parses the commands from the config file.
    pub fn parse_commands(cfg: &mut String, cmd: &str) {
        if cfg.contains(SPLIT_BULK_PLACEHOLDER) {
            panic!("[ERROR] Your config contains \"{SPLIT_BULK_PLACEHOLDER}\". This is a reserved string, please remove it!")
        }

        let lines = cfg.to_owned();
        let lines: Vec<&str> = lines.lines().filter(|line| line.contains(cmd)).collect();

        // Packing all the commands into one and splitting it yields ~1.5x faster execution, rather
        // than calling `execute` on each command separately.
        let mut packed_command = "echo \"".to_owned();

        for line in &lines {
            let command = Self::parse_command(line, cmd);
            if command.is_empty() {
                panic!("[ERROR] Command on line '{line}' is empty, please specify what to execute!")
            }

            packed_command.push_str(&format!("$({command})"));
            packed_command.push_str(SPLIT_BULK_PLACEHOLDER);
        }

        packed_command.push('"');

        let result = Utils::execute(&packed_command)
            .expect("[ERROR] Failed executing commands from the config!");
        let mut result = result.split(SPLIT_BULK_PLACEHOLDER);

        for line in lines {
            let current_command = result.next().unwrap();

            if current_command.ends_with('\n') {
                // No need to clone current_command, just replace.
                *cfg = cfg.replace(line, current_command);
                continue;
            }

            // No trailing new-line character, clone and add it before replacing.
            let mut current_command = current_command.to_owned();
            current_command.push('\n');
            *cfg = cfg.replace(line, &current_command);
        }
    }

    /// Parses the command. For example: `$cmd=uname -a`
    fn parse_command<'a>(line: &'a str, cmd: &'a str) -> &'a str {
        line.split(cmd)
            .nth(1)
            .expect("[ERROR] Failed parsing command!")
    }
}
