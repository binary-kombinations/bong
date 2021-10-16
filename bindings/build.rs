fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::SetWindowsHookExW,
        Windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx,
        Windows::Win32::UI::WindowsAndMessaging::CallNextHookEx,
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxW,
        Windows::Win32::UI::WindowsAndMessaging::GetClassNameW,
        Windows::Win32::UI::WindowsAndMessaging::AppendMenuW,
        Windows::Win32::UI::WindowsAndMessaging::GetMenuItemCount,
        Windows::Win32::UI::WindowsAndMessaging::GetMenuItemID,

        Windows::Win32::System::Diagnostics::Debug::OutputDebugStringW,

        Windows::Win32::System::LibraryLoader::GetModuleHandleW,
        Windows::Win32::System::LibraryLoader::GetProcAddress,

        Windows::Win32::System::SystemServices::IMAGE_DOS_HEADER,

        Windows::Win32::UI::WindowsAndMessaging::CWPRETSTRUCT,

        Windows::Win32::UI::WindowsAndMessaging::HC_ACTION,
        Windows::Win32::UI::WindowsAndMessaging::WM_INITMENUPOPUP,
        Windows::Win32::UI::WindowsAndMessaging::WM_INITMENU,
    };
}
