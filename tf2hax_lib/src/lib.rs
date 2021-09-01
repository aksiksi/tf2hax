#![cfg(windows)]
use std::ffi::c_void;

use bindings::Windows::Win32::Foundation::HANDLE;

mod util;

pub struct PlayerState {
    handle: HANDLE,
    client_base_addr: *const c_void,
}

impl PlayerState {
    const CLIENT_DLL_NAME: &'static str = "client.dll";
    const HP_OFFSET: usize = 0x00C3938C;

    pub fn new() -> Option<Self> {
        let (handle, process_id) = unsafe {
            util::find_process_info("Team Fortress 2").unwrap()
        };

        let client_base_addr = unsafe {
            util::find_module_info(process_id, Self::CLIENT_DLL_NAME).unwrap()
        };

        Some(Self {
            handle,
            client_base_addr,
        })
    }

    /// Returns player's HP
    pub fn get_hp(&self) -> u32 {
        use bindings::Windows::Win32::System::Diagnostics::Debug::ReadProcessMemory;

        let buf = [0; 2];
        let buf_ptr = buf.as_ptr() as *mut c_void;
        let addr = (self.client_base_addr as usize + Self::HP_OFFSET) as *const c_void;

        unsafe {
            ReadProcessMemory(self.handle, addr, buf_ptr, 2, 0 as *mut usize);
        }

        u16::from_le_bytes(buf) as u32
    }
}
