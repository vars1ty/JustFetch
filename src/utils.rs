use lxinfo::info;
use std::{fs::read_to_string, process::Command};

/// Initializes the config, fetches and prints the result.
pub fn print() -> String {
    let cfg = format!(
        "{}/.config/JustFetch/config",
        std::env::var("HOME").unwrap()
    );
    let mut cfg = read_to_string(cfg).unwrap_or_else(|_| {
        r#"Distro: [distro]
Kernel: [kernel]
Username: [username]
Create your own config at ~/.config/JustFetch/config"#
            .to_owned()
    });

    // Fetch the final content into `cfg`.
    fetch(&mut cfg);
    cfg
}

/// Replaces a string in `content`.
fn replace(content: &mut String, replace: &str, with: &str) {
    *content = content.replace(replace, with);
}

/// Fetches information and replaces strings from `cfg`.
fn fetch(cfg: &mut String) {
    const CMD: &str = "$cmd=";
    if cfg.contains(CMD) {
        parse_commands(cfg, CMD);
    }

    if !cfg.contains('[') && !cfg.contains(']') {
        // No alias characters found, spare some resources by not fetching system information.
        return;
    }

    let system_info =
        info::get_system_information().expect("[ERROR] Failed fetching system information!");
    replace(cfg, "[host]", &system_info.hostname);
    replace(cfg, "[kernel]", &system_info.kernel);
    replace(cfg, "[username]", &system_info.username);
    replace(cfg, "[distro]", &system_info.distro_name);
    replace(cfg, "[distro_id]", &system_info.distro_id);
    replace(cfg, "[distro_build_id]", &system_info.distro_build_id);
    replace(cfg, "[shell]", &system_info.shell);
    replace(cfg, "[uptime]", &system_info.uptime_formatted);
    replace(cfg, "[total_mem]", &system_info.total_mem);
    replace(cfg, "[cached_mem]", &system_info.cached_mem);
    replace(cfg, "[available_mem]", &system_info.available_mem);
    replace(cfg, "[used_mem]", &system_info.used_mem);
}

/// Parses the commands from the config file.
fn parse_commands(cfg: &mut String, cmd: &str) {
    const SPLIT_BULK: &str = "%split%";

    if cfg.contains(SPLIT_BULK) {
        panic!("[ERROR] Your config contains \"%split%\". This is a reserved string, please remove it!")
    }

    let lines = cfg.to_owned();
    let lines: Vec<&str> = lines.lines().collect();

    // Packing all the commands into one and splitting it yields ~1.5x faster execution, rather
    // than calling `execute` on each command separately.
    let mut packed_command = "echo \"".to_owned();

    for i in 0..lines.len() {
        let line = lines[i];
        if !line.contains(cmd) {
            continue;
        }

        let command = parse_command(line, cmd);
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
    for line in lines {
        if !line.contains(cmd) {
            continue;
        }

        let res_command = result.next().unwrap();
        let raw_command = parse_command(line, cmd);
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

/// Executes a command and returns the output.
fn execute(cmd: &str) -> Option<String> {
    if cmd.is_empty() {
        return None;
    }

    let mut result = unsafe {
        String::from_utf8_unchecked(
            Command::new("sh")
                .args(["-c", cmd])
                .output()
                .unwrap()
                .stdout,
        )
    };

    // Remove the last character as its a new line.
    result.pop();

    Some(result)
}
