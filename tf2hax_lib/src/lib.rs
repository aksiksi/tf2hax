#![cfg(windows)]
use std::ffi::c_void;

use process::Process;

pub mod process;
mod util;

pub struct PlayerState {
    process: Process,
    client_base_addr: *const c_void,
}

impl PlayerState {
    const CLIENT_DLL_NAME: &'static str = "client.dll";
    const HP_OFFSET: usize = 0x00C3938C;

    pub fn new(process: Process) -> Option<Self> {
        let client_base_addr = process.find_module_info(Self::CLIENT_DLL_NAME)?;

        Some(Self {
            process,
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
            ReadProcessMemory(self.process.handle(), addr, buf_ptr, 2, 0 as *mut usize);
        }

        u16::from_le_bytes(buf) as u32
    }
}
