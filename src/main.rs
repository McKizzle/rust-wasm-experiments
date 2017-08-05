use std::os::raw::c_char;
use std::ffi::CString;

fn main() { }

#[no_mangle]
pub fn ping() -> *mut c_char {
    CString::new("pong").unwrap().into_raw()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
