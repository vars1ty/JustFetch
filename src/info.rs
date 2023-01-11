use libc::{c_char, sysconf, _SC_HOST_NAME_MAX};

use crate::utils;
use std::{
    ffi::{CStr, OsString},
    fs::read_to_string,
    mem::MaybeUninit,
    os::unix::prelude::OsStringExt,
};

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
    pub uptime: String,
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

/// Returns the active host username.
fn get_username() -> String {
    let username;
    unsafe {
        username = CStr::from_ptr(libc::getlogin())
            .to_str()
            .expect("[ERROR] Failed retrieving username!");
    }

    username.to_owned()
}

/// Returns the active hostname.
fn get_hostname() -> String {
    // Thanks to swsnr/gethostname.rs
    let hostname_max = unsafe { sysconf(_SC_HOST_NAME_MAX) };
    let mut buffer = vec![0; (hostname_max as usize) + 1];
    unsafe { libc::gethostname(buffer.as_mut_ptr() as *mut _, buffer.len()) };
    let end = buffer
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(buffer.len());
    buffer.resize(end, 0);
    OsString::from_vec(buffer)
        .to_str()
        .expect("[ERROR] Failed getting hostname as str!")
        .to_owned()
}

/// Returns the active kernel version.
fn get_kernel_version() -> String {
    let mut info = unsafe { MaybeUninit::<libc::utsname>::zeroed().assume_init() };
    let mut result = vec![0; info.release.len()];
    unsafe { libc::uname(&mut info as *mut _) };

    // Push content into `result` as `u8`.
    for i in info.release {
        result.push(i as u8);
    }

    String::from_utf8(result).unwrap()
}

/// Fetches system information.
pub fn get_system_information() -> Option<SystemInfo> {
    let os_release =
        read_to_string("/etc/os-release").expect("[ERROR] Failed reading /etc/os-release!");
    let distro_name = parse_key(&os_release, "NAME")?;
    let distro_id = parse_key(&os_release, "ID")?;
    let distro_build_id = parse_key(&os_release, "BUILD_ID")?;

    let username = get_username();
    let hostname = get_hostname();
    let shell = env!("SHELL").split('/').last()?.to_owned();
    let kernel = get_kernel_version();
    let mut uptime = utils::execute("uptime -p");
    let uptime_start = uptime
        .to_owned()
        .split_whitespace()
        .next()
        .expect("[ERROR] Failed splitting by whitespace on uptime_start!")
        .to_owned();
    uptime = uptime.replace(&format!("{uptime_start} "), "");

    Some(SystemInfo {
        distro_name,
        distro_id,
        distro_build_id,
        username,
        hostname,
        shell,
        kernel,
        uptime,
    })
}
