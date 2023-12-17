
/// required to removed windows console when launching GUI.
/// Tauri by default does not support this feature.
#[cfg(target_os = "windows")]
pub fn hide_windows_console(switch: bool) {
    unsafe {
        if switch {
            windows_sys::Win32::System::Console::FreeConsole();
        } else {
            windows_sys::Win32::System::Console::AllocConsole();
        }
    }
}
