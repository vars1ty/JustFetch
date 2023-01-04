use crate::info;
use std::{collections::HashMap, fs::read_to_string, path::Path, process::Command};

/// Initializes the config, fetches and prints the result.
pub fn print() -> String {
    let cfg = format!("/home/{}/.config/JustFetch/config", execute("whoami"));
    let path = Path::new(&cfg);

    // Create/Load config.
    if !path.exists() {
        panic!("[ERROR] Missing config at '{cfg}', please create one!")
    } else {
        fetch(&read_to_string(cfg).expect("[ERROR] Failed reading config!"))
    }
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
    let mut cfg = cfg.replace("[host]", &system_info.hostname);
    cfg = cfg.replace("[kernel]", &system_info.kernel);
    cfg = cfg.replace("[username]", &system_info.username);
    cfg = cfg.replace("[distro]", &system_info.distro_name);
    cfg = cfg.replace("[distro_id]", &system_info.distro_id);
    cfg = cfg.replace("[distro_build_id]", &system_info.distro_build_id);
    cfg = cfg.replace("[shell]", &system_info.shell);
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
            panic!("[ERROR] Command on line '{line}' is empty, please specify one!")
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

/// Executes several commands at once, split by `, ` between each command.
/// For example: let result = execute_batched("whoami, uname -a");
/// Retrieving the value of `whoami`: let whoami = result[0];
pub fn execute_batched(cmds: &str) -> Vec<String> {
    let mut formatted = String::new();
    for split in cmds.split(", ") {
        formatted.push_str(&format!("$({split})|"))
    }

    if formatted.ends_with('|') {
        // Remove the last character from `formatted` (aka '|').
        formatted.pop();
    }

    let executed = execute(&format!("echo \"{formatted}\""));
    executed
        .split('|')
        .map(|entry| entry.to_owned())
        .collect::<Vec<String>>()
}
