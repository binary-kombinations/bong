use bindings::Windows::Win32::{
    Foundation::{HINSTANCE, HWND, PWSTR},
    System::SystemServices::IMAGE_DOS_HEADER,
    UI::WindowsAndMessaging::GetClassNameW,
};
use widestring::U16CString;

extern "C" {
    static __ImageBase: IMAGE_DOS_HEADER;
}

pub fn image_base() -> HINSTANCE {
    unsafe { HINSTANCE(&__ImageBase as *const IMAGE_DOS_HEADER as _) }
}

pub unsafe fn get_class_name(hwnd: HWND) -> windows::Result<U16CString> {
    // The maximum length for lpszClassName is 256
    // https://docs.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-wndclassexa
    let mut name = vec![0u16; 256 + 1];

    let returned_chars = GetClassNameW(hwnd, PWSTR(name.as_mut_ptr()), name.len() as _);
    if returned_chars == 0 {
        return Err(windows::Error::from_win32());
    }

    Ok(U16CString::from_vec_truncate(name))
}
