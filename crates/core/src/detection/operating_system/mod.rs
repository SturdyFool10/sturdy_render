pub enum OS {
    Windows,
    MacOS,
    Linux,
    FreeBSD,
    Unknown,
}

//gets the OS as a enum, most values are apparent, but if its none of the obvious ones it returns OS::Unknown, still static enough to not have runtime cost
pub fn get_os() -> OS {
    #[cfg(target_os = "windows")]
    let os = OS::Windows;
    #[cfg(target_os = "macos")]
    let os = OS::MacOS;
    #[cfg(target_os = "linux")]
    let os = OS::Linux;
    #[cfg(target_os = "freebsd")]
    let os = OS::FreeBSD;
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "freebsd"
    )))]
    let os = OS::Unknown;

    os
}
