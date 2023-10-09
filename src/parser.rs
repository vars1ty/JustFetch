use crate::utils::execute;
use colorful::{Colorful, RGB};
use lazy_regex::regex;

pub struct Parser;

impl Parser {
    /// Parses RGB colors in the config and displays them.
    /// For example: `rgb["Hello, I'm red!", 255, 0, 0]` displays `Hello, I'm red!` as a red color.
    pub fn parse_color(cfg: &mut String) {
        // If there's no rgb pattern, skip creating the regex.
        if !cfg.contains("rgb[\"") {
            return;
        }

        let cfg_clone = cfg.to_owned();
        let regex =
            regex!(r#"rgb\["(.*?)",\s*(\d+),\s*(\d+),\s*(\d+)\]"#).captures_iter(&cfg_clone);
        //println!("{content}, {r}, {g}, {b}");
        for capture in regex {
            let whole = capture
                .get(0)
                .expect("[ERROR] Failed getting whole content at index 0!")
                .as_str();
            let content = capture
                .get(1)
                .expect("[ERROR] Failed getting regex content at index 1!")
                .as_str();
            let r: u8 = capture
                .get(2)
                .expect("[ERROR] Failed getting regex r at index 2!")
                .as_str()
                .parse()
                .expect("[ERROR] Failed parsing R!");
            let g: u8 = capture
                .get(3)
                .expect("[ERROR] Failed getting regex g at index 3!")
                .as_str()
                .parse()
                .expect("[ERROR] Failed parsing G!");
            let b: u8 = capture
                .get(4)
                .expect("[ERROR] Failed getting regex b at index 4!")
                .as_str()
                .parse()
                .expect("[ERROR] Failed parsing B!");
            *cfg = cfg.replace(whole, &content.color(RGB::new(r, g, b)).to_string());
        }
    }
    /// Parses the commands from the config file.
    pub fn parse_commands(cfg: &mut String, cmd: &str) {
        const SPLIT_BULK: &str = "%split%";

        if cfg.contains(SPLIT_BULK) {
            panic!("[ERROR] Your config contains \"%split%\". This is a reserved string, please remove it!")
        }

        let lines = cfg.to_owned();
        let mut lines: Vec<&str> = lines.lines().collect();

        // Packing all the commands into one and splitting it yields ~1.5x faster execution, rather
        // than calling `execute` on each command separately.
        let mut packed_command = "echo \"".to_owned();

        for i in 0..lines.len() {
            let line = lines[i];
            if !line.contains(cmd) {
                continue;
            }

            let command = Self::parse_command(line, cmd);
            if command.is_empty() {
                panic!("[ERROR] Command on line '{line}' is empty, please specify what to execute!")
            }

            packed_command.push_str(&format!("$({command})"));
            if i != lines.len() - 1 {
                packed_command.push_str(SPLIT_BULK);
            } else {
                packed_command.push('"');
            }
        }

        let result = execute(&packed_command).unwrap();
        let mut result = result.split(SPLIT_BULK);
        lines.retain(|line| line.contains(cmd));
        for line in lines {
            let res_command = result.next().unwrap();
            let raw_command = Self::parse_command(line, cmd);
            *cfg = cfg.replace(
                &format!("{cmd}{raw_command}\n"),
                &format!("{res_command}\n"),
            );
        }
    }

    /// Parses the command. For example: `$cmd=uname -a`
    fn parse_command<'a>(line: &'a str, cmd: &'a str) -> &'a str {
        line.split(cmd)
            .nth(1)
            .expect("[ERROR] Failed parsing command!")
    }
}
