use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK};

fn main() {
    let hello_text = "Hello, World!";
    let hello_caption = "Hello";

    unsafe {
        MessageBoxW(
            HWND(0),
            PCWSTR::from_raw(hello_text.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr()),
            PCWSTR::from_raw(hello_caption.encode_utf16().chain(Some(0)).collect::<Vec<u16>>().as_ptr()),
            MB_OK,
        );
    }
}
