#![no_std]
#![feature(lang_items)]
#![feature(link_args)]

extern crate libc;

// Link as "Windows application" to avoid console window flash
#[link_args = "-Wl,--subsystem,windows"]
extern {}

#[allow(non_camel_case_types)]
mod ffi {
  use libc::{c_uint, uintptr_t};
  use libc::types::os::arch::extra::{BOOL, HANDLE, LONG_PTR};

  type UINT = c_uint;
  type UINT_PTR = uintptr_t;
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
#[lang = "fail_fmt"] extern fn fail_fmt() {}
#[start]
fn main(_: int, _: *const *const u8) -> int {
  lcd_off();
  0
}
