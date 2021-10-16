use bindings::Windows::Win32::{
    Foundation::{CloseHandle, HANDLE},
    System::{
        Diagnostics::Debug::GetLastError,
        Threading::{CreateEventW, SetEvent, WaitForSingleObject, WAIT_FAILED},
        WindowsProgramming::INFINITE,
    },
};

const ERROR_ALREADY_EXISTS: u32 = 183;

#[derive(Debug)]
pub struct Event {
    handle: HANDLE,
}

impl Event {
    pub fn new(
        manual_reset: bool,
        signaled: bool,
        name: Option<&str>,
    ) -> windows::Result<(Self, bool)> {
        let handle = unsafe {
            if let Some(name) = name {
                CreateEventW(std::ptr::null(), manual_reset, signaled, name)
            } else {
                CreateEventW(std::ptr::null(), manual_reset, signaled, None)
            }
        };
        if handle.0 == 0 {
            return Err(windows::Error::from_win32());
        }
        let created = unsafe { GetLastError() }.0 != ERROR_ALREADY_EXISTS;

        Ok((Self { handle }, created))
    }

    pub fn wait(&self) -> windows::Result<()> {
        let result = unsafe { WaitForSingleObject(self.handle, INFINITE) };
        if result == WAIT_FAILED {
            return Err(windows::Error::from_win32());
        }

        Ok(())
    }

    pub fn signal(&self) -> windows::Result<()> {
        unsafe { SetEvent(self.handle).ok() }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle).ok().unwrap();
        }
    }
}
