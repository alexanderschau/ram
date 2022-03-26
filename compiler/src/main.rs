mod generator;
mod parser;
mod tokenizer;
use std::ffi::{CStr, CString};
use std::io;
use std::io::prelude::*;
use std::mem;
use std::os::raw::c_char;
use std::os::raw::c_void;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let tokens = tokenizer::run(&buffer);
    let ast = parser::run(&tokens);
    let code = generator::rust::gen(&ast);
    println!("{}", code);
    Ok(())
}

// wasm :)

/*extern "C" {
    fn print(text: &str);
}*/

#[no_mangle]
pub extern "C" fn alloc() -> *mut c_void {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    mem::forget(buf);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut c_void) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}

#[no_mangle]
pub unsafe fn compile(ptr: *mut u8) -> *mut c_char {
    let data = CStr::from_ptr(ptr as *const i8).to_str().unwrap();

    let tokens = tokenizer::run(data);
    let ast = parser::run(&tokens);
    let code = generator::javascript::gen(&ast);
    let s = format!("{}", code);
    let s = CString::new(s).unwrap();

    s.into_raw()
}
