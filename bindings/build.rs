fn main() {
    windows::build! {
        Windows::Win32::UI::WindowsAndMessaging::SetWindowsHookExW,
        Windows::Win32::UI::WindowsAndMessaging::UnhookWindowsHookEx,
        Windows::Win32::UI::WindowsAndMessaging::CallNextHookEx,
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxW,
        Windows::Win32::UI::WindowsAndMessaging::GetClassNameW,
        Windows::Win32::UI::WindowsAndMessaging::SendMessageW,

        Windows::Win32::System::Diagnostics::Debug::OutputDebugStringW,

        Windows::Win32::System::SystemServices::IMAGE_DOS_HEADER,

        Windows::Win32::UI::WindowsAndMessaging::CWPSTRUCT,

        Windows::Win32::UI::Controls::WM_CONTEXTMENU,
        Windows::Win32::UI::Controls::EM_ENABLESEARCHWEB,
    };
}
