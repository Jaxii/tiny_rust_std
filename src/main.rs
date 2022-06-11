#![windows_subsystem = "console"]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::io;
use std::io::Write;
// Blow up if we try to compile without msvc, x64 arch, or windows.
// Includes syscall constant.

pub fn main() {

  let mut stdout = io::stdout().lock();

  stdout.write_all(b"hello world").unwrap();

}

