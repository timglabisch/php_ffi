use std::os::raw::c_char;
use std::ffi::CString;

#[no_mangle]
pub extern fn return_u64() -> u64 {
    42
}

#[repr(C)]
pub struct ComplexFlatStructure {
    pub _1: i8,
    pub _2: i16,
    pub _3: i32,
    pub _4: i64,
    pub _5: u8,
    pub _6: u16,
    pub _7: u32,
    pub _8: u64,
}

impl ComplexFlatStructure {
    pub fn new() -> Self {
        ComplexFlatStructure {
            _1: 1,
            _2: 2,
            _3: 3,
            _4: 4,
            _5: 5,
            _6: 6,
            _7: 7,
            _8: 8,
        }
    }
}

#[no_mangle]
pub extern fn return_pointer() -> *const ComplexFlatStructure {
    Box::into_raw(Box::new(ComplexFlatStructure::new()))
}

#[no_mangle]
pub extern fn free_pointer(ptr : *mut ComplexFlatStructure) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern fn return_string() -> *const c_char {
    CString::into_raw(CString::new("some string ...😀").expect("..."))
}

#[no_mangle]
pub extern fn free_string(cstr : *mut c_char) {
    unsafe { CString::from_raw(cstr); }
}
