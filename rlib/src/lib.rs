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

#[derive(Debug, Clone)]
struct IoRequest {
    filename: String,
    token: u64
}

#[derive(Debug, Clone)]
struct IoResponse {
    token: u64,
    data: Vec<u8>,
    request: IoRequest
}

lazy_static! {
    static ref TOKEN : AtomicUsize = AtomicUsize::new(0);
    static ref REQUEST_QUEUE : Mutex<(Sender<IoRequest>, Option<Receiver<IoRequest>>)> = Mutex::new({
        let (s, r) = channel(100);
        (s, Some(r))
    });
    static ref RESPONSE_QUEUE : Mutex<(Sender<IoResponse>, Option<Receiver<IoResponse>>)> = Mutex::new({
        let (s, r) = channel(100);
        (s, Some(r))
    });
}

#[repr(C)]
pub struct Buffer {
    pub data: *mut u8,
    pub len: u32,
}

#[no_mangle]
pub extern fn start() {
    thread::spawn(||{
        task::block_on(async {

            loop {

                let mut recv_queue = {
                    let mut request_queue_guard = REQUEST_QUEUE.lock().await;

                    &mut request_queue_guard.1.take().unwrap()
                };

                loop {

                    let request : IoRequest = match recv_queue.next().await {
                        Some(v) => v,
                        None => continue
                    };

                    println!("rust: starts working on task.");

                    let mut file = File::open(&request.filename).await.expect("...");

                    let mut data = vec![];
                    let _ = file.read(&mut data).await.expect("...");

                    RESPONSE_QUEUE.lock().await.0.try_send(IoResponse {
                        token: request.token,
                        data,
                        request
                    }).expect("...");

                }
            }
        })
    });
}

#[no_mangle]
pub extern fn queue_read(filename: *const c_char) -> u64 {
    let token = TOKEN.fetch_add(1, Ordering::Relaxed) as u64;

    let filename = unsafe { CStr::from_ptr(filename) }.to_string_lossy();

    task::block_on(async move {
        let mut request_queue_guard = REQUEST_QUEUE.lock().await;

        &mut request_queue_guard.0.try_send(IoRequest {
            token,
            filename: filename.to_string(),
        }).expect("...");
    });


    token
}

#[no_mangle]
pub extern fn poll() -> Buffer {

    task::block_on(async {

        let mut recv_queue = {
            let mut request_queue_guard = RESPONSE_QUEUE.lock().await;

            &mut request_queue_guard.1.take().unwrap()
        };

        loop {

            let response: IoResponse = match recv_queue.next().await {
                Some(v) => v,
                None => continue
            };

            println!("rust: got msg.");

            let mut buf = response.data.clone();
            buf.push(b'\0');
            let data = buf.as_mut_ptr();
            let len = buf.len();
            std::mem::forget(buf);
            return Buffer { data, len: len as u32 }
        }

    })
}

/*
#[no_mangle]
pub extern fn free_buf(buf: Buffer) {
    let s = unsafe { std::slice::from_raw_parts_mut(buf.data, buf.len as usize) };
    let s = s.as_mut_ptr();
    unsafe {
        Box::from_raw(s);
    }
}
*/


#[no_mangle]
pub extern fn hello_rust(_buffer: *const c_char) -> i32 {

    /*

    if ptr.is_null() {
        return;
    }
    */

    /*
    let s = unsafe { CStr::from_ptr(buffer) };
    let ptr = s.as_ptr();

    unsafe {
        // `ptr` is dangling
        *ptr[0] = 1;
    }

    println!("{:?}", &s);
    */

    10


}