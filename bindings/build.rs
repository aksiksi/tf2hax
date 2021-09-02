fn main() {
    windows::build! {
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::System::Diagnostics::Debug::{GetLastError, ReadProcessMemory, WriteProcessMemory},
        Windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32},
        Windows::Win32::System::SystemServices::CHAR,
        Windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameA},
        Windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId},
    };
}
