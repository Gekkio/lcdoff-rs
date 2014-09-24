#![no_std]
#![feature(lang_items)]

extern crate libc;

#[allow(non_camel_case_types)]
mod ffi {
  use libc::{c_int, c_uint, c_long, c_void};

  type BOOL = c_int;
  type UINT = c_uint;
  type PVOID = *mut c_void;

  #[cfg(target_arch = "x86")]
  type LONG_PTR = c_long;
  #[cfg(target_arch = "x86_64")]
  type LONG_PTR = i64;

  #[cfg(target_arch = "x86")]
  type UINT_PTR = c_uint;
  #[cfg(target_arch = "x86_64")]
  type UINT_PTR = u64;

  type HANDLE = PVOID;
  type HWND = HANDLE;
  type WPARAM = UINT_PTR;
  type LPARAM = LONG_PTR;

  pub static HWND_BROADCAST: HWND = 0xffff as HWND;
  pub static WM_SYSCOMMAND: UINT = 0x0112;
  pub static SC_MONITORPOWER: WPARAM = 0xf170;

  #[link(name = "user32")]
  extern "system" {
    pub fn SendNotifyMessageW(hwnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM) -> BOOL;
  }
}

fn lcd_off() {
  unsafe {
    // 2 (the display is being shut off)
    ffi::SendNotifyMessageW(ffi::HWND_BROADCAST, ffi::WM_SYSCOMMAND, ffi::SC_MONITORPOWER, 2);
  }
}

// Boilerplate needed to avoid dependency on std
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[start]
fn main(_: int, _: *const *const u8) -> int {
  lcd_off();
  0
}
