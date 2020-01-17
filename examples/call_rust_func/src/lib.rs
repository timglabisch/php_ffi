use std::os::raw::c_char;
use std::ffi::{CString, CStr};

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
pub extern fn free_pointer(ptr: *mut ComplexFlatStructure) {
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern fn return_string() -> *const c_char {
    CString::into_raw(CString::new("some string ...😀").expect("..."))
}

#[no_mangle]
pub extern fn free_string(cstr: *mut c_char) {
    unsafe { CString::from_raw(cstr); }
}


#[repr(C)]
pub struct StructWithPointerInnerC {
    _1: *mut c_char,
    _2: *mut c_char,
}

#[repr(C)]
pub struct StructWithPointerC {
    _1: *mut c_char,
    _2: *mut StructWithPointerInnerC,
    _3: *mut c_char,
}

#[no_mangle]
pub extern fn return_struct_with_pointer() -> *const StructWithPointerC {
    let y = StructWithPointerC {
        _1: CString::new("foo1").unwrap().into_raw(),
        _2: Box::into_raw(Box::new(StructWithPointerInnerC {
            _1: CString::new("ifoo1").unwrap().into_raw(),
            _2: CString::new("ifoo2").unwrap().into_raw(),
        })),
        _3: CString::new("foo3").unwrap().into_raw(),
    };

    Box::into_raw(Box::new(y))
}

#[no_mangle]
pub extern fn free_struct_with_pointer(d: *mut StructWithPointerC) {
    // cou can also do this in the drop function :)
    unsafe { CString::from_raw((*d)._1); }
    unsafe { CString::from_raw((*(*d)._2)._1); }
    unsafe { CString::from_raw((*(*d)._2)._2); }
    unsafe { Box::from_raw((*d)._2); }
    unsafe { CString::from_raw((*d)._3); }
    unsafe { Box::from_raw(d); }
}

#[repr(C)]
pub struct StructWithPointerAndDrop {
    _1: *mut c_char,
    _2: *mut c_char,
}

impl Drop for StructWithPointerAndDrop {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self._1);
            CString::from_raw(self._2);
        }
    }
}

#[no_mangle]
pub extern fn return_struct_with_pointer_drop() -> *const StructWithPointerAndDrop {
    Box::into_raw(Box::new(StructWithPointerAndDrop {
        _1: CString::new("foo1").unwrap().into_raw(),
        _2: CString::new("foo2").unwrap().into_raw(),
    }))
}

#[no_mangle]
pub extern fn free_struct_with_pointer_drop(d: *mut StructWithPointerC) {
    unsafe { Box::from_raw(d); }
}