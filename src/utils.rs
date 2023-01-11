use crate::info;
use std::{collections::HashMap, fs::read_to_string, process::Command};

/// Initializes the config, fetches and prints the result.
pub fn print() -> String {
    let cfg = format!("/home/{}/.config/JustFetch/config", execute("whoami"));
    fetch(
        &read_to_string(cfg).unwrap_or(
            r#"Distro: [distro]
Kernel: [kernel]
Username: [username]
Create your own config at ~/.config/JustFetch/config"#
                .to_owned(),
        ),
    )
}

/// Replaces a string in `content` if found.
fn replace_if_present(content: String, replace: &str, with: &str) -> String {
    if !content.contains(replace) {
        return content;
    }

    let content = content.replace(replace, with);
    content
}

/// Fetches information and replaces strings from `cfg`.
fn fetch(cfg: &str) -> String {
    let mut cfg = cfg.to_owned();
    parse_commands(&mut cfg);
    if !cfg.contains('[') && !cfg.contains(']') {
        // No alias characters found, spare some resources by not fetching system information.
        return cfg;
    }

    let system_info =
        info::get_system_information().expect("[ERROR] Failed fetching system information!");
    let mut cfg = replace_if_present(cfg, "[host]", &system_info.hostname);
    cfg = replace_if_present(cfg, "[kernel]", &system_info.kernel);
    cfg = replace_if_present(cfg, "[username]", &system_info.username);
    cfg = replace_if_present(cfg, "[distro]", &system_info.distro_name);
    cfg = replace_if_present(cfg, "[distro_id]", &system_info.distro_id);
    cfg = replace_if_present(cfg, "[distro_build_id]", &system_info.distro_build_id);
    cfg = replace_if_present(cfg, "[shell]", &system_info.shell);
    cfg = replace_if_present(cfg, "[uptime]", &system_info.uptime);
    cfg
}

/// Parses the commands from the config file.
fn parse_commands(cfg: &mut String) {
    const CMD: &str = "$cmd=";
    if !cfg.contains(CMD) {
        return;
    }

    let mut final_cfg = cfg.clone();
    let mut command_cache: HashMap<String, String> = HashMap::new();
    for line in cfg.lines() {
        if !line.contains(CMD) {
            continue;
        }

        let command = parse_command(line);
        if command.is_empty() {
            panic!("[ERROR] Command on line '{line}' is empty, please specify what to execute!")
        }

        let raw_command = &format!("{CMD}{command}");
        final_cfg = final_cfg.replace(
            raw_command,
            &get_from_cache(&mut command_cache, raw_command, &command),
        )
    }

    *cfg = final_cfg;
}

/// Gets a command from the `cache` if present. Otherwise it's added and then returned.
fn get_from_cache(
    cache: &mut HashMap<String, String>,
    raw_command: &String,
    command: &str,
) -> String {
    let output = if cache.contains_key(raw_command) {
        // Found in cache, return it.
        cache.get(raw_command).unwrap().to_owned()
    } else {
        // Not found, cache it so we can reuse the result if needed.
        let res = execute(&command);
        cache.insert(raw_command.to_owned(), res.to_owned());
        res
    };

    output
}

/// Parses the command. For example: `$cmd=uname -a`
pub fn parse_command(line: &str) -> String {
    let split = line
        .split("$cmd=")
        .nth(1)
        .expect("[ERROR] Failed parsing command!");
    split.to_owned()
}

/// Executes a command and returns the output.
pub fn execute(cmd: &str) -> String {
    let mut result;
    if cmd.is_empty() {
        return String::default();
    }

    result = String::from_utf8_lossy(
        &Command::new("sh")
            .args(["-c", cmd])
            .output()
            .unwrap()
            .stdout,
    )
    .to_string();

    // Remove the last character as its a new line.
    result.pop();

    result
}
