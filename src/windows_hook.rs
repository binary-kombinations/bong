use bindings::Windows::Win32::{
    Foundation::HINSTANCE,
    UI::WindowsAndMessaging::{
        SetWindowsHookExW, UnhookWindowsHookEx, HHOOK, HOOKPROC, WINDOWS_HOOK_ID,
    },
};

#[derive(Debug)]
pub struct WindowsHook {
    hook: HHOOK,
}

impl WindowsHook {
    pub unsafe fn set(
        id: WINDOWS_HOOK_ID,
        function: Option<HOOKPROC>,
        module: HINSTANCE,
        thread_id: u32,
    ) -> windows::Result<Self> {
        let hook = SetWindowsHookExW(id, function, module, thread_id);
        if hook.0 == 0 {
            return Err(windows::Error::from_win32());
        }
        Ok(Self { hook })
    }
}

impl Drop for WindowsHook {
    fn drop(&mut self) {
        unsafe {
            UnhookWindowsHookEx(self.hook).ok().unwrap();
        }
    }
}
