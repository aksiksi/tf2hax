use std::ffi::c_void;

use bindings::Windows::Win32::Foundation::{CloseHandle, HANDLE};
use bindings::Windows::Win32::System::Diagnostics::Debug::GetLastError;
use bindings::Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32};
use bindings::Windows::Win32::System::SystemServices::CHAR;
use bindings::Windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId};

pub(crate) fn string_from_ascii_raw(s: &[CHAR]) -> String {
    let bytes: Vec<u8> = s.iter().map(|c| c.0).take_while(|c| *c != 0).collect();
    String::from_utf8(bytes).unwrap()
}

/// Finds the process handle and ID for the window with the given name
pub(crate) unsafe fn find_process_info(window_name: &str) -> Option<(HANDLE, u32)> {
    let win_handle = FindWindowW(None, window_name);
    if win_handle.is_null() {
        eprintln!("failed to get window handle");
        return None;
    }

    let mut process_id = 0u32;
    GetWindowThreadProcessId(win_handle, &mut process_id);

    let handle =
        OpenProcess(PROCESS_ALL_ACCESS, false, process_id);
    if handle.is_null() || handle.is_invalid() {
        eprintln!("failed to get process handle: {:?}", GetLastError());
        return None;
    }

    return Some((handle, process_id));
}

/// Returns the base address of a module in an external process
///
/// * Approach used here: https://docs.microsoft.com/en-us/windows/win32/toolhelp/traversing-the-module-list
/// * Alternate approach: https://stackoverflow.com/a/26397667/845275
pub(crate) unsafe fn find_module_info(process_id: u32, name: &str) -> Option<*const c_void> {
    let snapshot_handle = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id);
    if snapshot_handle.is_null() || snapshot_handle.is_invalid() {
        eprintln!("failed to get process snapshot handle: {:?}", GetLastError());
        return None;
    }

    let mut module_entry = MODULEENTRY32::default();
    module_entry.dwSize = std::mem::size_of::<MODULEENTRY32>() as u32;

    if !Module32First(snapshot_handle, &mut module_entry).as_bool() {
        eprintln!("failed to get first module: {:?}", GetLastError());
        CloseHandle(snapshot_handle);
        return None;
    }

    loop {
        let module_name = string_from_ascii_raw(&module_entry.szModule);
        if &module_name == &name {
            break;
        }

        if !Module32Next(snapshot_handle, &mut module_entry).as_bool() {
            eprintln!("module \"{}\" not found", name);
            return None;
        }
    }

    CloseHandle(snapshot_handle);

    return Some(module_entry.modBaseAddr as *const c_void);
}
