use crate::utils;
use std::{ffi::CStr, fs::File, io::Read, mem::MaybeUninit};

/// Simple macro to convert all bytes to their u8 representation.
macro_rules! bytes_to_u8 {
    ($collection:expr) => {
        $collection
            .iter()
            .map(|byte| *byte as u8)
            .collect::<Vec<_>>()
    };
}

/// Fetched system information.
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

/// Type of information to obtain.
#[derive(PartialEq)]
pub enum Type {
    Username,
    HostName,
    KernelVersion,
}

/// Reads the specified file using `read_exact`.
fn fread(path: &str) -> String {
    let mut file = File::open(path).expect("[ERROR] Failed reading file!");
    let length = file
        .metadata()
        .expect("[ERROR] Failed reading metadata of file!")
        .len() as usize;
    let mut buffer = Vec::new();
    buffer.resize(length, 0);
    file.read_exact(&mut buffer)
        .expect("[ERROR] Failed reading file into buffer!");
    String::from_utf8(buffer).expect("[ERROR] Failed returning buffer as a String!")
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

/// Returns the active kernel version.
pub fn get_by_type(r#type: Type) -> String {
    // Create an uninitialized instance of `utsname`.
    let mut info = unsafe { MaybeUninit::<libc::utsname>::zeroed().assume_init() };
    // Store the output of `uname` into `info` as long as the type isn't `Username`.
    if r#type != Type::Username {
        unsafe { libc::uname(&mut info as *mut _) };
    } else {
        // Drop `info` to free its resources asap, since it won't be used.
        drop(info)
    }

    let result;
    match r#type {
        Type::Username => unsafe {
            return CStr::from_ptr(libc::getlogin())
                .to_str()
                .expect("[ERROR] Failed retrieving username!")
                .to_owned();
        },
        Type::HostName => {
            result = bytes_to_u8!(info.nodename);
        }
        Type::KernelVersion => {
            result = bytes_to_u8!(info.release);
        }
    }

    String::from_utf8(result).expect("[ERROR] Failed converting libc output to a String!")
}

/// Fetches system information.
pub fn get_system_information() -> Option<SystemInfo> {
    let os_release = fread("/etc/os-release");
    let distro_name = parse_key(&os_release, "NAME")?;
    let distro_id = parse_key(&os_release, "ID")?;
    let distro_build_id = parse_key(&os_release, "BUILD_ID")?;

    let username = get_by_type(Type::Username);
    let hostname = get_by_type(Type::HostName);
    let shell = env!("SHELL").split('/').last()?.to_owned();
    let kernel = get_by_type(Type::KernelVersion);
    let mut uptime = utils::execute("uptime -p");

    // Get the first entry when split by a whitespace, then remove it.
    // For example: "up 10 minutes" => "10 minutes".
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
