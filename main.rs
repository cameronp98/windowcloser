extern crate winapi;

use winapi::um::winuser::{GetForegroundWindow, CloseWindow};
use std::time::Duration;
use std::thread;

fn main() {
    loop {
        unsafe {
            let window = GetForegroundWindow();
            println!("info: got foreground window {:?}", window);
            println!("info: close window -> {}", CloseWindow(window));
        }
        thread::sleep(Duration::from_secs(30));
    }
}
