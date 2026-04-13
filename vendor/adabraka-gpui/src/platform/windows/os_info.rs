use windows::Win32::System::WindowsProgramming::GetComputerNameW;
use windows::core::PWSTR;

use crate::OsInfo;

pub(crate) fn get_os_info() -> OsInfo {
    let version = get_windows_version();
    let hostname = get_hostname();
    let locale = get_locale();

    OsInfo {
        name: "windows".into(),
        version: version.into(),
        arch: std::env::consts::ARCH.into(),
        locale: locale.into(),
        hostname: hostname.into(),
    }
}

fn get_windows_version() -> String {
    let mut info = unsafe { std::mem::zeroed() };
    let status = unsafe { windows::Wdk::System::SystemServices::RtlGetVersion(&mut info) };
    if status.is_ok() {
        format!(
            "{}.{}.{}",
            info.dwMajorVersion, info.dwMinorVersion, info.dwBuildNumber
        )
    } else {
        String::new()
    }
}

fn get_hostname() -> String {
    let mut size: u32 = 256;
    let mut buffer = vec![0u16; size as usize];
    let result = unsafe { GetComputerNameW(Some(PWSTR(buffer.as_mut_ptr())), &mut size) };
    if result.is_ok() {
        String::from_utf16_lossy(&buffer[..size as usize])
    } else {
        String::new()
    }
}

fn get_locale() -> String {
    let mut buffer = [0u16; 85];
    let len = unsafe { windows::Win32::Globalization::GetUserDefaultLocaleName(&mut buffer) };
    if len > 0 {
        String::from_utf16_lossy(&buffer[..(len as usize - 1)])
    } else {
        String::new()
    }
}
