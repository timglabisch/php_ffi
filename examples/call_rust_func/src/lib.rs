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

pub struct StructWithPointerInner {
    pub _1: CString,
    pub _2: CString,
}

pub struct StructWithPointer {
    pub _1: CString,
    pub _2: StructWithPointerInner,
    pub _3: CString,
}

impl StructWithPointer {
    pub fn new() -> StructWithPointer {
        StructWithPointer {
            _1: CString::new("foo1").unwrap(),
            _2: StructWithPointerInner {
                _1: CString::new("ifoo1").unwrap(),
                _2: CString::new("ifoo2").unwrap(),
            },
            _3: CString::new("foo2").unwrap(),
        }
    }
}

#[repr(C)]
pub struct StructWithPointerInnerC {
    _1: *const c_char,
    _2: *const c_char,
}

#[repr(C)]
pub struct StructWithPointerC {
    _1: *const c_char,
    _2: *const StructWithPointerInnerC,
    _3: *const c_char,
}

#[no_mangle]
pub extern fn return_struct_with_pointer() -> *const StructWithPointerC {
    let StructWithPointer { _1: _1, _2: _2, _3: _3 } = StructWithPointer::new();
    let StructWithPointerInner { _1: i1, _2: i2 } = _2;

    let y = StructWithPointerC {
        _1: _1.into_raw(),
        _2: Box::into_raw(Box::new(StructWithPointerInnerC {
            _1: i1.into_raw(),
            _2: i2.into_raw(),
        })),
        _3: _3.into_raw(),
    };

    Box::into_raw(Box::new(y))
}

#[no_mangle]
pub extern fn free_struct_with_pointer(d: *mut StructWithPointer) {
    unsafe { Box::from_raw(d); }
}

