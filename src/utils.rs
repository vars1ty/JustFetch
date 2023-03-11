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

/// Replaces a string in `content` if found.
fn replace_if_present(content: &mut String, replace: &str, with: &str) {
    if content.contains(replace) {
        *content = content.replace(replace, with);
    }
}

/// Fetches information and replaces strings from `cfg`.
fn fetch(cfg: &mut String) {
    if cfg.contains("$cmd=") {
        parse_commands(cfg);
    }

    if !cfg.contains('[') && !cfg.contains(']') {
        // No alias characters found, spare some resources by not fetching system information.
        return;
    }

    let system_info =
        info::get_system_information().expect("[ERROR] Failed fetching system information!");
    replace_if_present(cfg, "[host]", &system_info.hostname);
    replace_if_present(cfg, "[kernel]", &system_info.kernel);
    replace_if_present(cfg, "[username]", &system_info.username);
    replace_if_present(cfg, "[distro]", &system_info.distro_name);
    replace_if_present(cfg, "[distro_id]", &system_info.distro_id);
    replace_if_present(cfg, "[distro_build_id]", &system_info.distro_build_id);
    replace_if_present(cfg, "[shell]", &system_info.shell);
    replace_if_present(cfg, "[uptime]", &system_info.uptime_formatted);
    replace_if_present(cfg, "[total_mem]", &system_info.total_mem);
    replace_if_present(cfg, "[cached_mem]", &system_info.cached_mem);
    replace_if_present(cfg, "[available_mem]", &system_info.available_mem);
    replace_if_present(cfg, "[used_mem]", &system_info.used_mem);
}

/// Parses the commands from the config file.
fn parse_commands(cfg: &mut String) {
    const CMD: &str = "$cmd=";
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
        if !line.contains(CMD) {
            continue;
        }

        let command = parse_command(line);
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
        if !line.contains(CMD) {
            continue;
        }

        let res_command = result.next().unwrap();
        let raw_command = parse_command(line);
        *cfg = cfg.replace(
            &format!("{CMD}{raw_command}\n"),
            &format!("{res_command}\n"),
        );
    }
}

/// Parses the command. For example: `$cmd=uname -a`
fn parse_command(line: &str) -> String {
    let split = line
        .split("$cmd=")
        .nth(1)
        .expect("[ERROR] Failed parsing command!");
    split.to_owned()
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
