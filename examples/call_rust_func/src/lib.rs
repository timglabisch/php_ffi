#[no_mangle]
pub extern fn return_u64() -> u64 {
    42
}

#[repr(C)]
pub struct ComplexFlatStructure {
    pub a: u8,
    pub b: i8,
    pub c: u16,
    pub d: i16,
    pub e: u32,
    pub f: i32,
    pub g: u64,
    pub h: i64,
    pub i: bool,
}

#[no_mangle]
pub extern fn return_pointer() -> *const ComplexFlatStructure {
    Box::into_raw(Box::new(ComplexFlatStructure {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
        e: 5,
        f: 6,
        g: 7,
        h: 8,
        i: true,
    }))
}
