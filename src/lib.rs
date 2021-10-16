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
            CallNextHookEx, MessageBoxW, SendMessageW, CWPSTRUCT, MB_ICONERROR, MB_OK,
            WH_CALLWNDPROC,
        },
    },
};
use widestring::U16CStr;

use crate::{
    util::{get_class_name, image_base},
    windows_hook::WindowsHook,
};

extern "system" fn call_wndproc_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let _result = catch_unwind(|| {
        if code < 0 {
            return;
        }

        let message_info = unsafe { (lparam.0 as *const CWPSTRUCT).as_ref() };

        if let Some(message_info) = message_info {
            if let Err(error) = process_call_wndproc_hook(message_info) {
                unsafe {
                    OutputDebugStringW(format!("bong | {:?}", error));
                }
            }
        }
    });

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

fn process_call_wndproc_hook(message_info: &CWPSTRUCT) -> anyhow::Result<()> {
    if message_info.message != WM_CONTEXTMENU {
        return Ok(());
    }

    let owner_class_name = unsafe { get_class_name(message_info.hwnd)? };
    if owner_class_name.to_string_lossy().to_lowercase() != "edit" {
        return Ok(());
    }

    unsafe {
        SendMessageW(
            message_info.hwnd,
            EM_ENABLESEARCHWEB,
            WPARAM(true.into()),
            LPARAM(0),
        );
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

    let _edit_menu_hook = unsafe {
        WindowsHook::set(
            WH_CALLWNDPROC,
            Some(call_wndproc_hook),
            image_base(),
            tid,
        )?
    };

    Ok(())
}
