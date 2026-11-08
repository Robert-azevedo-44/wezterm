//! Windows registry read helpers for Wezterm.

pub const APP_KEY: &str = "Software\\Wezterm\\Wezterm";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Wezterm";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Wezterm") },
        RegValue { name: "Version",     value: String::from("v1.5.7") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Wezterm") },
        RegValue { name: "Publisher",       value: String::from("Wezterm") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Wezterm\\uninstall.exe\"") },
    ]
}
