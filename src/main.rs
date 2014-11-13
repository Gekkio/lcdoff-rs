#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(link_args)]

extern crate libc;

use libc::{c_int};

// Link as "Windows application" to avoid console window flash
#[link_args = "-Wl,--subsystem,windows"]
extern {}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
  use libc::{
    c_int, c_uint, uintptr_t
  };
  pub use libc::types::os::arch::extra::{
    BOOL, DWORD, HANDLE, LONG_PTR, LPCWSTR, LPSTR, LPVOID, LRESULT, WORD
  };

  pub type ATOM = WORD;
  pub type HBRUSH = HANDLE;
  pub type HCURSOR = HICON;
  pub type HICON = HANDLE;
  pub type HINSTANCE = HANDLE;
  pub type HMENU = HANDLE;
  pub type HWND = HANDLE;
  pub type LPARAM = LONG_PTR;
  pub type LPCTSTR = LPCWSTR;
  pub type UINT = c_uint;
  pub type UINT_PTR = uintptr_t;
  pub type WPARAM = UINT_PTR;
  pub type WNDPROC = extern "stdcall" fn(HWND, UINT, WPARAM, LPARAM) -> LRESULT;

  pub const CW_USEDEFAULT: c_int = 0x80000000u as c_int;
  pub const WM_SYSCOMMAND: UINT = 0x0112;
  pub const SC_MONITORPOWER: WPARAM = 0xf170;

  #[repr(C)]
  pub struct WNDCLASS {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: c_int,
    pub cbWndExtra: c_int,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCTSTR,
    pub lpszClassName: LPCTSTR
  }

  #[link(name = "user32")]
  extern "stdcall" {
    pub fn DefWindowProcW(
      hwnd: HWND,
      msg: UINT,
      wParam: WPARAM,
      lParam: LPARAM) -> LRESULT;

    pub fn SendNotifyMessageW(
      hwnd: HWND,
      msg: UINT,
      wParam: WPARAM,
      lParam: LPARAM) -> BOOL;

    pub fn RegisterClassW(
      lpWndClass: *const WNDCLASS) -> ATOM;

    pub fn CreateWindowExW(
      dwExStyle: DWORD,
      lpClassName: LPCTSTR,
      lpWindowName: LPCTSTR,
      dwStyle: DWORD,
      x: c_int,
      y: c_int,
      nWidth: c_int,
      nHeight: c_int,
      hWndParent: HWND,
      hMenu: HMENU,
      hInstance: HINSTANCE,
      lpParam: LPVOID) -> HWND;
  }

  pub extern "stdcall" fn default_window_proc(
      hwnd: HWND, msg: UINT, wParam: WPARAM, lParam: LPARAM)
      -> LRESULT {
    unsafe {
      DefWindowProcW(hwnd, msg, wParam, lParam)
    }
  }
}

fn null<T>() -> *const T {
  0 as *const T
}

fn null_mut<T>() -> *mut T {
  0 as *mut T
}

fn lcd_off(hwnd: ffi::HWND) {
  unsafe {
    // 2 (the display is being shut off)
    ffi::SendNotifyMessageW(hwnd, ffi::WM_SYSCOMMAND, ffi::SC_MONITORPOWER, 2);
  }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "stdcall" fn WinMain(
    hInstance: ffi::HINSTANCE, _: ffi::HINSTANCE,
    _: ffi::LPSTR, _: c_int) -> c_int {

  let className = [
    'l' as u16, 'c' as u16, 'd' as u16,
    'o' as u16, 'f' as u16, 'f' as u16,
    '-' as u16, 'r' as u16, 's' as u16, 0 as u16
  ];
  let class = ffi::WNDCLASS {
    style: 0,
    lpfnWndProc: ffi::default_window_proc,
    cbClsExtra: 0,
    cbWndExtra: 0,
    hInstance: hInstance,
    hIcon: null_mut(),
    hCursor: null_mut(),
    hbrBackground: null_mut(),
    lpszMenuName: null(),
    lpszClassName: &className[0]
  };
  let atom = unsafe { ffi::RegisterClassW(&class) };
  let window = unsafe { ffi::CreateWindowExW(
    0,
    atom as ffi::LPCTSTR,
    null(),
    0,
    ffi::CW_USEDEFAULT,
    ffi::CW_USEDEFAULT,
    ffi::CW_USEDEFAULT,
    ffi::CW_USEDEFAULT,
    null_mut(),
    null_mut(),
    hInstance,
    null_mut())
  };

  lcd_off(window);
  0
}

// Boilerplate needed to avoid dependency on std
#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "fail_fmt"] extern fn fail_fmt() {}
