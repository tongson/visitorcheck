use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::io::Read;
use std::panic;

extern crate ureq;

#[no_mangle]
pub extern "C" fn score(c: *const c_char) -> *const c_char {
  panic::set_hook(Box::new(move |_| eprintln!("panic: visitorcheck.score()")));
  let cb = unsafe { CStr::from_ptr(c).to_string_lossy().into_owned() };
  let req = format!("https://tool.nodehost.cloud/visitorcheck/?action=report&key={}", cb);
  let get = ureq::get(&req).call();
  // Process response
  let mut bytes = vec![];
  if get.status().to_string() == "200" {
    let mut reader = get.into_reader();
    let _ = reader.read_to_end(&mut bytes);
  } else {
    bytes = b"0".to_vec();
  }
  // Return response
  let c_str = CString::new(bytes).unwrap();
  let ptr = c_str.as_ptr();
  std::mem::forget(c_str);
  return ptr
}

