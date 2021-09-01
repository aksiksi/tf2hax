fn main() {
    windows::build! {
        Windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory},
    };
}
