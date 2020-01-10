use async_std::fs::File;
use async_std::io;
use async_std::prelude::*;
use async_std::task;
use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use futures::lock::Mutex;
use futures::channel::mpsc::{Receiver, Sender, channel};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

#[macro_use]
extern crate lazy_static;


#[repr(C)]
pub struct Buffer {
    pub data: u64,
    pub len: u32,
}

#[no_mangle]
pub extern fn poll() -> Buffer {
    return Buffer { len: 1, data: 1 };
}