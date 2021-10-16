mod event;
mod util;
mod windows_hook;

use std::panic::catch_unwind;

use anyhow::Context;
use bindings::Windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    System::Diagnostics::Debug::OutputDebugStringW,
    UI::{
        Controls::{EM_ENABLESEARCHWEB, WM_CONTEXTMENU},
        WindowsAndMessaging::{
            CallNextHookEx, MessageBoxW, SendMessageW, CWPSTRUCT, MB_ICONERROR, MB_OK, MSG,
            WH_CALLWNDPROC, WH_GETMESSAGE,
        },
    },
};
use widestring::U16CStr;

use crate::{
    event::Event,
    util::{get_class_name, image_base},
    windows_hook::WindowsHook,
};

const SHUTDOWN_EVENT: &'static str = "BONG_SHUTDOWN_EVENT_033e0095-da6d-4e9e-a6c7-a9fb45871522";

extern "system" fn call_wndproc_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let _result = catch_unwind(|| {
        if code < 0 {
            return;
        }

        let message_info = unsafe { (lparam.0 as *const CWPSTRUCT).as_ref() };

        if let Some(message_info) = message_info {
            if let Err(error) = enable_search(message_info.hwnd, message_info.message) {
                unsafe {
                    OutputDebugStringW(format!("bong | {:?}", error));
                }
            }
        }
    });

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

extern "system" fn get_message_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let _result = catch_unwind(|| {
        if code < 0 {
            return;
        }

        let message = unsafe { (lparam.0 as *const MSG).as_ref() };

        if let Some(message) = message {
            if let Err(error) = enable_search(message.hwnd, message.message) {
                unsafe {
                    OutputDebugStringW(format!("bong | {:?}", error));
                }
            }
        }
    });

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

fn enable_search(hwnd: HWND, message: u32) -> anyhow::Result<()> {
    if message != WM_CONTEXTMENU {
        return Ok(());
    }

    let class_name = unsafe { get_class_name(hwnd)? };
    if class_name.to_string_lossy().to_lowercase() != "edit" {
        return Ok(());
    }

    unsafe {
        SendMessageW(hwnd, EM_ENABLESEARCHWEB, WPARAM(true.into()), LPARAM(0));
    }

    Ok(())
}

#[no_mangle]
pub extern "system" fn hookW(
    hwnd: HWND,
    _hinstance: HINSTANCE,
    command_line: *const u16,
    _cmd_show: i32,
) {
    let _result = catch_unwind(|| {
        if let Err(error) = do_hook(command_line) {
            unsafe {
                MessageBoxW(hwnd, format!("{:?}", error), "bong", MB_OK | MB_ICONERROR);
            }
        }
    });
}

fn do_hook(command_line: *const u16) -> anyhow::Result<()> {
    let command_line = if command_line.is_null() {
        String::new()
    } else {
        unsafe { U16CStr::from_ptr_str(command_line) }.to_string_lossy()
    };
    let command_line = command_line.trim();
    let command_line = if command_line.is_empty() {
        "0"
    } else {
        command_line
    };

    let tid: u32 = command_line
        .parse()
        .with_context(|| format!("Invalid TID: {}", command_line))?;

    let (shutdown_event, event_created) = Event::new(true, false, Some(SHUTDOWN_EVENT))?;

    let mut _call_wndproc_hook_handle = None;
    let mut _get_message_hook_handle = None;

    if event_created {
        // We're the first. Install the hooks.

        _call_wndproc_hook_handle = Some(unsafe {
            WindowsHook::set(WH_CALLWNDPROC, Some(call_wndproc_hook), image_base(), tid)?
        });

        _get_message_hook_handle = Some(unsafe {
            WindowsHook::set(WH_GETMESSAGE, Some(get_message_hook), image_base(), tid)?
        });
    }

    shutdown_event.wait()?;

    Ok(())
}

#[no_mangle]
pub extern "system" fn unhookW(
    hwnd: HWND,
    _hinstance: HINSTANCE,
    _command_line: *const u16,
    _cmd_show: i32,
) {
    let _result = catch_unwind(|| {
        if let Err(error) = do_unhook() {
            unsafe {
                MessageBoxW(hwnd, format!("{:?}", error), "bong", MB_OK | MB_ICONERROR);
            }
        }
    });
}

fn do_unhook() -> anyhow::Result<()> {
    let (shutdown_event, _) = Event::new(true, false, Some(SHUTDOWN_EVENT))?;

    shutdown_event.signal()?;

    Ok(())
}
