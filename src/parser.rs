use crate::utils::execute;
use colorful::{Colorful, RGB};
use lazy_regex::regex_replace_all;

pub struct Parser;

impl Parser {
    /// Parses RGB colors in the config and displays them.
    /// For example: `rgb["Hello, I'm red!", 255, 0, 0]` displays `Hello, I'm red!` as a red color.
    pub fn parse_color(cfg: &mut String) {
        // If there's no rgb pattern, skip creating the regex.
        if !cfg.contains("rgb[\"") {
            return;
        }

        *cfg = regex_replace_all!(
            r#"rgb\["(.*?)",\s*(\d+),\s*(\d+),\s*(\d+)\]"#,
            &cfg,
            |_, content: &str, r: &str, g: &str, b: &str| content
                .color(RGB::new(
                    r.parse().expect("[ERROR] Failed parsing R as u8!"),
                    g.parse().expect("[ERROR] Failed parsing G as u8!"),
                    b.parse().expect("[ERROR] Failed parsing B as u8!")
                ))
                .to_string()
        )
        .to_string();
    }
    /// Parses the commands from the config file.
    pub fn parse_commands(cfg: &mut String, cmd: &str) {
        const SPLIT_BULK: &str = "%split%";

        if cfg.contains(SPLIT_BULK) {
            panic!("[ERROR] Your config contains \"%split%\". This is a reserved string, please remove it!")
        }

        let lines = cfg.to_owned();
        let mut lines: Vec<&str> = lines.lines().filter(|line| line.contains(cmd)).collect();

        // Packing all the commands into one and splitting it yields ~1.5x faster execution, rather
        // than calling `execute` on each command separately.
        let mut packed_command = "echo \"".to_owned();

        for line in &lines {
            if !line.contains(cmd) {
                continue;
            }

            let command = Self::parse_command(line, cmd);
            if command.is_empty() {
                panic!("[ERROR] Command on line '{line}' is empty, please specify what to execute!")
            }

            packed_command += &format!("$({command})");
            packed_command += SPLIT_BULK;
        }

        packed_command.push('"');

        let result = execute(&packed_command).expect("[ERROR] Failed executing custom command!");
        let mut result = result.split(SPLIT_BULK);

        // Only keep the lines that have a command defined, as we'll be executing it and getting
        // its output, then replacing the command in `cfg`.
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
