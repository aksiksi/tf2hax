use bindings::Windows::Win32::System::SystemServices::CHAR;

/// Converts a Windows NULL-terminated CHAR buffer into a String.
pub(crate) fn string_from_ascii_raw(s: &[CHAR]) -> Option<String> {
    let bytes: Vec<u8> = s.iter().map(|c| c.0).take_while(|c| *c != 0).collect();
    String::from_utf8(bytes).ok()
}

/// Converts a NULL-terminated u8 buffer into a String.
pub(crate) fn string_from_bytes(bytes: &[u8]) -> Option<String> {
    let bytes: Vec<u8> = bytes.iter().map(|c| *c).take_while(|c| *c != 0).collect();
    String::from_utf8(bytes).ok()
}
