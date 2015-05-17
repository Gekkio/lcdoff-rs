#![no_main]
#![feature(core)]
#![feature(link_args)]

extern crate core;
extern crate user32;
extern crate winapi;

use core::option::Option::Some;
use std::ptr::{null, null_mut};
use user32::{CreateWindowExW, DefWindowProcW, RegisterClassW, SendNotifyMessageW};
use winapi::{c_int,
  CW_USEDEFAULT, HINSTANCE, HWND, LPSTR, LPCWSTR, WNDCLASSW, WM_SYSCOMMAND, WPARAM
};

// Link as "Windows application" to avoid console window flash
#[link_args = "-Wl,--subsystem,windows"]
extern {}

const SC_MONITORPOWER: WPARAM = 0xf170;

fn lcd_off(hwnd: HWND) {
  unsafe {
    // 2 (the display is being shut off)
    SendNotifyMessageW(hwnd, WM_SYSCOMMAND, SC_MONITORPOWER, 2);
  }
}

macro_rules! bytes_16( ($($e:expr),*) => ({ [$($e as u16),*] }) );

static CLASS_NAME: [u16; 10] =
  bytes_16!('l', 'c', 'd', 'o', 'f', 'f', '-', 'r', 's', '\0');

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMain(
    hInstance: HINSTANCE, _: HINSTANCE,
    _: LPSTR, _: c_int) -> c_int {

  let class = WNDCLASSW {
    style: 0,
    lpfnWndProc: Some(DefWindowProcW),
    cbClsExtra: 0,
    cbWndExtra: 0,
    hInstance: hInstance,
    hIcon: null_mut(),
    hCursor: null_mut(),
    hbrBackground: null_mut(),
    lpszMenuName: null(),
    lpszClassName: &CLASS_NAME[0]
  };
  let atom = unsafe { RegisterClassW(&class) };
  let window = unsafe { CreateWindowExW(
    0,
    atom as LPCWSTR,
    null(),
    0,
    CW_USEDEFAULT,
    CW_USEDEFAULT,
    CW_USEDEFAULT,
    CW_USEDEFAULT,
    null_mut(),
    null_mut(),
    hInstance,
    null_mut())
  };

  lcd_off(window);
  0
}
