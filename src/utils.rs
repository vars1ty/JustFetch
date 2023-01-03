use std::{fs::read_to_string, path::Path};
use sysinfo::{System, SystemExt};

/// Initializes the config, fetches and prints the result.
pub fn print() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let cfg = format!("/home/{}/.config/JustFetch/config", execute("whoami"));
    let path = Path::new(&cfg);

    // Create/Load config.
    if !path.exists() {
        panic!("[ERROR] Missing config at '{cfg}', please create one!")
    } else {
        fetch(
            sys,
            &read_to_string(cfg).expect("[ERROR] Failed reading config!"),
        )
    }
}

/// Fetches information and replaces strings from `cfg`.
fn fetch(sys: System, cfg: &str) -> String {
    let mut cfg = cfg.to_owned();
    parse_commands(&mut cfg);
    let mut cfg = cfg.replace(
        "[host]",
        &sys.host_name()
            .expect("[ERROR] Failed retrieving host name!"),
    );
    cfg = cfg.replace(
        "[os_version]",
        &sys.os_version()
            .expect("[ERROR] Failed retrieving os version!"),
    );
    cfg = cfg.replace(
        "[kernel]",
        &sys.kernel_version()
            .expect("[ERROR] Failed retrieving kernel version!"),
    );
    cfg = cfg.replace(
        "[name]",
        &sys.name().expect("[ERROR] Failed retrieving system name!"),
    );
    cfg = cfg.replace("[username]", &execute("whoami"));
    cfg = cfg.replace(
        "[shell]",
        execute("echo $SHELL")
            .split('/')
            .last()
            .expect("[ERROR] Failed retrieving the current shell!"),
    );
    cfg
}

/// Parses the commands from the config file.
fn parse_commands(cfg: &mut String) {
    const CMD: &str = "$cmd=";
    if !cfg.contains(CMD) {
        return;
    }

    let mut final_cfg = cfg.clone();
    for line in cfg.lines() {
        if !line.contains(CMD) {
            continue;
        }

        let command = parse_command(line);
        if command.is_empty() {
            panic!("[ERROR] Command on line '{line}' is empty, please specify one!")
        }

        final_cfg = final_cfg.replace(&format!("{CMD}{command}"), &execute(&command))
    }

    *cfg = final_cfg;
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
fn execute(cmd: &str) -> String {
    let mut result;
    if cmd.is_empty() {
        return String::default();
    }

    result = String::from_utf8_lossy(
        &std::process::Command::new("sh")
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
