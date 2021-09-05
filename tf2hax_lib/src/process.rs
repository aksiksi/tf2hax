use std::ffi::c_void;
use std::path::PathBuf;

use bindings::Windows::Win32::Foundation::{CloseHandle, HANDLE, PSTR};
use bindings::Windows::Win32::System::Diagnostics::Debug::GetLastError;
use bindings::Windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE,
    TH32CS_SNAPMODULE32,
};
use bindings::Windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameA, PROCESS_ALL_ACCESS,
};
use bindings::Windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetWindowThreadProcessId};

/// Represents the info needed to interact with a Windows process.
#[derive(Clone, Debug)]
pub struct Process {
    /// Absolute path to the process executable
    path: PathBuf,
    process_id: u32,
    handle: HANDLE,
}

impl Process {
    /// Queries the process that owns the window with the given name
    pub fn from_window(name: &str) -> Option<Self> {
        let (handle, process_id, path) = unsafe { Self::find_process_info(name)? };
        let path = PathBuf::from(&path);

        Some(Self {
            handle,
            process_id,
            path,
        })
    }

    pub fn handle(&self) -> HANDLE {
        self.handle
    }

    /// Returns the name of the executable that spawned this process.
    ///
    /// The return type is `Option<&str>` due to `OSString` conversion. You should
    /// be able to unwrap the returned `Option` in the vast majority of cases.
    pub fn name(&self) -> Option<&str> {
        self.path.file_name().and_then(|n| n.to_str())
    }

    pub fn path(&self) -> Option<&str> {
        self.path.to_str()
    }

    unsafe fn find_process_info(window_name: &str) -> Option<(HANDLE, u32, String)> {
        // Get the handle for the window with the given name
        let win_handle = FindWindowW(None, window_name);
        if win_handle.is_null() {
            eprintln!("failed to get window handle");
            return None;
        }

        // Find the process ID that owns the window
        let mut process_id = 0u32;
        GetWindowThreadProcessId(win_handle, &mut process_id);

        // Get a handle to the process
        let handle = OpenProcess(PROCESS_ALL_ACCESS, false, process_id);
        if handle.is_null() || handle.is_invalid() {
            eprintln!("failed to get process handle: {:?}", GetLastError());
            return None;
        }

        // Use the handle to read the process executable's path
        let process_path = Self::get_process_executable_path(handle)?;

        return Some((handle, process_id, process_path));
    }

    unsafe fn get_process_executable_path(handle: HANDLE) -> Option<String> {
        let mut buf = [0u8; 256];
        let pstr = PSTR(buf.as_mut_ptr() as *mut u8);
        let mut pstr_size = buf.len() as u32;

        let status = QueryFullProcessImageNameA(handle, 0.into(), pstr, &mut pstr_size);
        if !status.as_bool() {
            eprintln!("failed to get process name: {:?}", GetLastError());
            return None;
        }

        let process_path = crate::util::string_from_bytes(&buf)?;

        Some(process_path)
    }

    /// Given the name of a module (DLL), returns the base address of the module in this process.
    ///
    /// # Notes
    ///
    /// * Approach used here: https://docs.microsoft.com/en-us/windows/win32/toolhelp/traversing-the-module-list
    /// * Another approach: https://stackoverflow.com/a/26397667/845275
    pub fn find_module_info(&self, name: &str) -> Option<*const c_void> {
        // SAFETY: No buffers are impacted by input to this function. The module_entry is the only "concerning" usage,
        // but we use size_of to ensure that the API gets the correct size.
        unsafe {
            let snapshot_handle =
                CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, self.process_id);
            if snapshot_handle.is_null() || snapshot_handle.is_invalid() {
                eprintln!(
                    "failed to get process snapshot handle: {:?}",
                    GetLastError()
                );
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
                let module_name =
                    crate::util::string_from_ascii_raw(&module_entry.szModule).unwrap();
                if &module_name == &name {
                    break;
                }

                if !Module32Next(snapshot_handle, &mut module_entry).as_bool() {
                    eprintln!("module \"{}\" not found", module_name);
                    return None;
                }
            }

            CloseHandle(snapshot_handle);

            return Some(module_entry.modBaseAddr as *const c_void);
        }
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle);
        }
    }
}
