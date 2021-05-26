use std::os::raw::{c_char};
use std::ffi::{CString,CStr};

use rand::rngs::OsRng;
use bip39::{Language, Mnemonic};

#[no_mangle]
pub extern fn mnemonic(length: *const c_char) -> *mut c_char {
    // convert from CString inputs
    let input_cstr = unsafe {CStr::from_ptr(length)};
    let len:usize = match input_cstr.to_str(){
        Err(_) => 12,
        Ok(string)=> string.parse::<usize>().unwrap()
    };
    // regular rust code
    let mut rng = OsRng::new().expect("!!OsRng Error!!");
    let mnemonic = Mnemonic::generate_in_with(&mut rng, Language::English, len)
        .unwrap()
        .to_string();

    // convert to CString outputs
    CString::new(mnemonic).unwrap().into_raw()

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mnemonic() {
        let length = 12;
        let length_cstr = CString::new(length.to_string()).unwrap().into_raw();
        let mnemonic_ptr = mnemonic(length_cstr);
        println!("Output C *char: {:?}",mnemonic_ptr);
        let mnemonic_cstr = unsafe {CStr::from_ptr(mnemonic_ptr)};
        let mnemonic_native = mnemonic_cstr.to_str().unwrap();
        println!("Output Rust &str: {}",mnemonic_native);
    }


}
