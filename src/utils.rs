use crate::parser::Parser;
use lxinfo::info;
use std::{fs::read_to_string, process::Command};

/// Utilities like printing the system information, fetching and executing shell-commands.
pub struct Utils;

impl Utils {
    /// Initializes the config, fetches and prints the result.
    pub fn print() -> String {
        let cfg = format!(
            "{}/.config/JustFetch/config",
            std::env::var("HOME").unwrap_or_else(|_| std::env::var("XDG_CONFIG_HOME")
                .expect("[ERROR] No XDG_CONFIG_HOME or HOME!"))
        );
        let mut cfg = read_to_string(&cfg).unwrap_or_else(|_| {
            format!(
                r#"Distro: [distro]
Kernel: [kernel]
Username: [username]
Create your own config at {cfg}"#
            )
        });

        // Fetch the final content into `cfg`.
        Self::fetch(&mut cfg);
        Parser::parse_color(&mut cfg);
        cfg
    }

    /// Fetches information and replaces strings from `cfg`.
    fn fetch(cfg: &mut String) {
        const CMD: &str = "$cmd=";
        if cfg.contains(CMD) {
            Parser::parse_commands(cfg, CMD);
        }

        if !cfg.contains('[') && !cfg.contains(']') {
            // No alias characters found, spare some resources by not fetching system information.
            return;
        }

        let system_info =
            info::get_system_information().expect("[ERROR] Failed fetching system information!");
        *cfg = cfg.replace("[host]", &system_info.hostname);
        *cfg = cfg.replace("[kernel]", &system_info.kernel);
        *cfg = cfg.replace("[username]", &system_info.username);
        *cfg = cfg.replace("[distro]", &system_info.distro_name);
        *cfg = cfg.replace("[distro_id]", &system_info.distro_id);
        *cfg = cfg.replace("[distro_build_id]", &system_info.distro_build_id);
        *cfg = cfg.replace("[shell]", &system_info.shell);
        *cfg = cfg.replace("[uptime]", &system_info.uptime_formatted);
        *cfg = cfg.replace("[total_mem]", &system_info.total_mem);
        *cfg = cfg.replace("[cached_mem]", &system_info.cached_mem);
        *cfg = cfg.replace("[available_mem]", &system_info.available_mem);
        *cfg = cfg.replace("[used_mem]", &system_info.used_mem);
    }

    /// Executes a command and returns the output.
    pub fn execute(cmd: &str) -> Option<String> {
        if cmd.is_empty() {
            return None;
        }

        String::from_utf8(
            Command::new("sh")
                .args(["-c", cmd])
                .output()
                .unwrap()
                .stdout,
        )
        .ok()
        .map(|mut result| {
            // Remove trailing \n.
            result.pop();
            result
        })
    }
}
