use crate::parser::Parser;
use lxinfo::info;

/// Utilities like printing the system information, fetching and executing shell-commands.
pub struct Utils;

impl Utils {
    /// Initializes the config, fetches and prints the result.
    /// If `JF_HOME` wasn't specified, then it tries to get the home path
    /// automatically.
    pub fn print() -> String {
        let cfg = format!(
            "{}/.config/JustFetch/config",
            std::env::var("JF_HOME").unwrap_or_else(|_| std::env::home_dir()
                .expect("[ERROR] No home directory found, and JF_HOME was unspecified!")
                .to_str()
                .expect("[ERROR] Failed to get home directory as a &str!")
                .to_owned())
        );
        let mut cfg = std::fs::read_to_string(&cfg).unwrap_or_else(|_| {
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
        if !cfg.contains('[') && !cfg.contains(']') {
            // No alias characters found, spare some resources by not fetching system information.
            return;
        }

        let system_info =
            info::get_system_information().expect("[ERROR] Failed fetching system information!");
        Self::replace_if_found("[host]", &system_info.hostname, cfg);
        Self::replace_if_found("[username]", &system_info.username, cfg);
        Self::replace_if_found("[kernel]", &system_info.kernel, cfg);
        Self::replace_if_found("[distro]", &system_info.distro_name, cfg);
        Self::replace_if_found("[distro_id]", &system_info.distro_id, cfg);
        Self::replace_if_found("[distro_build_id]", &system_info.distro_build_id, cfg);
        Self::replace_if_found("[shell]", &system_info.shell, cfg);
        Self::replace_if_found("[uptime]", &system_info.uptime_formatted, cfg);
        Self::replace_if_found("[total_mem]", &system_info.total_mem, cfg);
        Self::replace_if_found("[cached_mem]", &system_info.cached_mem, cfg);
        Self::replace_if_found("[available_mem]", &system_info.available_mem, cfg);
        Self::replace_if_found("[used_mem]", &system_info.used_mem, cfg);
    }

    /// Replaces `find` with `replace` if found inside of `output`.
    /// This uses `output.contains` before calling `replace`, in order to prevent creating a new
    /// string unless needed.
    fn replace_if_found(find: &str, replace: &str, output: &mut String) {
        if !output.contains(find) {
            return;
        }

        *output = output.replace(find, replace);
    }
}
