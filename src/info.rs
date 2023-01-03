use crate::utils;
use std::fs::read_to_string;

/// Fetched system information.
#[derive(Debug)]
pub struct SystemInfo {
    pub distro_name: String,
    pub distro_id: String,
    pub distro_build_id: String,
    pub username: String,
    pub hostname: String,
    pub shell: String,
    pub kernel: String,
}

/// Parses the given key as a `String`.
pub fn parse_key(os_release: &str, key: &str) -> Option<String> {
    let mut split = os_release.split(&format!("{key}=")).nth(1)?.to_owned();
    if split.contains('\n') {
        // Only get the first line from the result.
        split = split.split('\n').next()?.to_owned()
    }

    if split.contains('"') {
        // Don't keep double-quotes.
        split = split.replace('"', "")
    }

    Some(split)
}

/// Fetches system information.
pub fn get_system_information() -> Option<SystemInfo> {
    let os_release =
        read_to_string("/etc/os-release").expect("[ERROR] Failed reading /etc/os-release!");
    let distro_name = parse_key(&os_release, "NAME")?;
    let distro_id = parse_key(&os_release, "ID")?;
    let distro_build_id = parse_key(&os_release, "BUILD_ID")?;
    let username = utils::execute("whoami");
    let hostname = utils::execute("uname -n");
    let shell = utils::execute("echo $SHELL").split('/').last()?.to_owned();
    let kernel = utils::execute("uname -r");
    Some(SystemInfo {
        distro_name,
        distro_id,
        distro_build_id,
        username,
        hostname,
        shell,
        kernel,
    })
}
