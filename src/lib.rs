mod util;
mod windows_hook;

use std::panic::catch_unwind;

use anyhow::{ensure, Context};
use bindings::Windows::Win32::{
    Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
    System::Diagnostics::Debug::OutputDebugStringW,
    UI::WindowsAndMessaging::{
        AppendMenuW, CallNextHookEx, GetMenuItemCount, GetMenuItemID, MessageBoxW, CWPRETSTRUCT,
        HMENU, MB_ICONERROR, MB_OK, MF_DISABLED, MF_SEPARATOR, MF_STRING, WH_CALLWNDPROCRET,
        WM_INITMENU, WM_INITMENUPOPUP,
    },
};
use widestring::U16CStr;

use crate::{
    util::{get_class_name, image_base},
    windows_hook::WindowsHook,
};

extern "system" fn bong_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let _result = catch_unwind(|| {
        if code < 0 {
            return;
        }

        let message_info = unsafe { (lparam.0 as *const CWPRETSTRUCT).as_ref() };

        if let Some(message_info) = message_info {
            if let Err(error) = process_window_message(message_info) {
                unsafe {
                    OutputDebugStringW(format!("bong | {:?}", error));
                }
            }
        }
    });

    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

fn process_window_message(message_info: &CWPRETSTRUCT) -> anyhow::Result<()> {
    // https://stackoverflow.com/questions/1346473/add-item-to-the-default-textbox-context-menu
    // https://stackoverflow.com/questions/46437708/inherit-or-subclass-textbox-to-disable-paste-on-textbox

    const WM_UAHINITMENU: u32 = 0x93;

    let hmenu = match message_info.message {
        WM_UAHINITMENU => {
            ensure!(message_info.lParam.0 != 0, "LPARAM is NULL");
            HMENU(unsafe { *(message_info.lParam.0 as *const isize) })
        }
        WM_INITMENU | WM_INITMENUPOPUP => HMENU(message_info.wParam.0 as _),
        _ => return Ok(()),
    };
    ensure!(hmenu.0 != 0, "HMENU is NULL");

    let owner_class_name = unsafe { get_class_name(message_info.hwnd)? };
    if owner_class_name.to_string_lossy().to_lowercase() != "edit" {
        return Ok(());
    }

    let max_id = unsafe { find_max_menu_id(hmenu)? };

    unsafe {
        AppendMenuW(hmenu, MF_SEPARATOR, 0, None)
            .ok()
            .with_context(|| "Failed adding separator")?;
    }

    unsafe {
        AppendMenuW(
            hmenu,
            MF_DISABLED | MF_STRING,
            (max_id + 1) as _,
            "Search with Bing...",
        )
        .ok()
        .with_context(|| "Failed adding new menu item")?;
    }

    Ok(())
}

unsafe fn find_max_menu_id(hmenu: HMENU) -> anyhow::Result<u32> {
    let menu_items = GetMenuItemCount(hmenu);
    ensure!(menu_items >= 0, "GetMenuItemCount failed");

    let mut max_id = 0u32;
    for index in 0..menu_items {
        let id = GetMenuItemID(hmenu, index);
        if id == u32::MAX {
            // It's a submenu
            continue;
        }
        if id > max_id {
            max_id = id;
        }
    }

    Ok(max_id)
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

    let _hook = unsafe { WindowsHook::set(WH_CALLWNDPROCRET, Some(bong_hook), image_base(), tid)? };

    Ok(())
}
