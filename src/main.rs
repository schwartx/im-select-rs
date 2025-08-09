use std::env;
use winapi::shared::minwindef::{BOOL, HKL, LPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::WM_INPUTLANGCHANGEREQUEST;
use winapi::um::winuser::{GetForegroundWindow, GetKeyboardLayout, PostMessageW};

fn get_input_method() -> Option<u16> {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.is_null() {
            #[cfg(debug_assertions)]
            eprintln!("GetForegroundWindow returned null.");
            return None;
        }

        let layout: HKL = GetKeyboardLayout(0);
        if layout.is_null() {
            #[cfg(debug_assertions)]
            eprintln!("GetKeyboardLayout returned null.");
            return None;
        }

        let lang_id: u16 = (layout as usize & 0xFFFF) as u16;
        Some(lang_id)
    }
}

fn switch_input_method(locale: u16) -> bool {
    unsafe {
        let hwnd: HWND = GetForegroundWindow();
        if hwnd.is_null() {
            #[cfg(debug_assertions)]
            eprintln!("GetForegroundWindow returned null.");
            return false;
        }

        let result: BOOL = PostMessageW(hwnd, WM_INPUTLANGCHANGEREQUEST, 0, locale as LPARAM);
        result != 0
    }
}

fn print_loaded_hkls() {
    // 局部引入
    use winapi::um::winuser::GetKeyboardLayoutList;
    unsafe {
        let count = GetKeyboardLayoutList(0, std::ptr::null_mut());

        if count == 0 {
            // Release 不打印，Debug 打印
            #[cfg(debug_assertions)]
            eprintln!("Failed to get keyboard layout count.");
            return;
        }

        let mut hkls = vec![std::ptr::null_mut(); count as usize];
        let ret = GetKeyboardLayoutList(count, hkls.as_mut_ptr());

        if ret == 0 {
            #[cfg(debug_assertions)]
            eprintln!("Failed to get keyboard layout list.");
            return;
        }

        println!("Loaded HKLs (count={}):", ret);
        for hkl in &hkls {
            let lang_id = (*hkl as usize & 0xFFFF) as u16;
            println!("HKL=0x{:x}, lang_id=0x{:04x}", *hkl as usize, lang_id);
        }
    }
}

fn main() {
    if let Some(arg) = env::args().nth(1) {
        if let Ok(locale) = arg.parse::<u16>() {
            // 成功解析为数字，尝试切换输入法
            if switch_input_method(locale) {
                #[cfg(debug_assertions)]
                println!("Switched input method to locale 0x{:04x}", locale);
            } else {
                #[cfg(debug_assertions)]
                eprintln!("Failed to switch input method");
            }
        } else {
            // 不是数字，判断字符串命令
            match arg.as_str() {
                "version" => {
                    println!("{}", env!("CARGO_PKG_VERSION"));
                }
                "hkls" => {
                    print_loaded_hkls();
                }
                _ => {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "Invalid argument. Provide a decimal locale or use 'version'/'hkls'."
                    );
                }
            }
        }
    } else {
        match get_input_method() {
            Some(lang_id) => println!("{}", lang_id),
            None => {
                #[cfg(debug_assertions)]
                eprintln!("Failed to retrieve the current input method.");
            }
        }
    }
}
