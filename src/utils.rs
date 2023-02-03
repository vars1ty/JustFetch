use lxinfo::info;
use std::{collections::HashMap, fs::read_to_string, process::Command};

/// Initializes the config, fetches and prints the result.
pub fn print(no_cmd: bool) -> String {
    let cfg = format!(
        "/home/{}/.config/JustFetch/config",
        info::get_by_type(info::Type::Username).unwrap()
    );
    let mut cfg = read_to_string(cfg).unwrap_or_else(|_| {
        r#"Distro: [distro]
Kernel: [kernel]
Username: [username]
Create your own config at ~/.config/JustFetch/config"#
            .to_owned()
    });

    // Fetch the final content into `cfg`.
    fetch(&mut cfg, no_cmd);
    cfg
}

/// Replaces a string in `content` if found.
fn replace_if_present(content: &mut String, replace: &str, with: &str) {
    if content.contains(replace) {
        *content = content.replace(replace, with);
    }
}

/// Fetches information and replaces strings from `cfg`.
fn fetch(cfg: &mut String, no_cmd: bool) {
    if !no_cmd {
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
    if !cfg.contains(CMD) {
        return;
    }

    let mut command_cache: HashMap<&str, String> = HashMap::new();
    let lines = cfg.to_owned();
    let lines = lines.lines();
    for line in lines {
        if !line.contains(CMD) {
            continue;
        }

        let command = parse_command(line);
        if command.is_empty() {
            panic!("[ERROR] Command on line '{line}' is empty, please specify what to execute!")
        }

        *cfg = cfg.replace(
            &format!("{CMD}{command}"),
            &get_from_cache(
                &mut command_cache,
                line.split(CMD).nth(1).unwrap(),
                &command,
            ),
        )
    }
}

/// Gets a command from the `cache` if present. Otherwise it's added and then returned.
fn get_from_cache<'a>(
    cache: &mut HashMap<&'a str, String>,
    raw_command: &'a str,
    command: &str,
) -> String {
    let output = if cache.contains_key(raw_command) {
        // Found in cache, return it.
        cache.get(raw_command).unwrap().to_owned()
    } else {
        // Not found, cache it so we can reuse the result if needed.
        let res = execute(command).expect("[ERROR] Cannot execute an empty command!");
        cache.insert(raw_command, res.to_owned());
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
pub fn execute(cmd: &str) -> Option<String> {
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
