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
use std::sync::Arc;

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
    static ref REQUEST_QUEUE : Mutex<(Sender<IoRequest>, Arc<Receiver<IoRequest>>)> = Mutex::new({
        let (s, r) = channel(100);
        (s, Arc::new(r))
    });
    static ref RESPONSE_QUEUE : Mutex<(Sender<IoResponse>, Option<Receiver<IoResponse>>)> = Mutex::new({
        let (s, r) = channel(100);
        (s, Some(r))
    });
}

#[repr(C)]
pub struct Buffer {
    pub data: *const c_char,
    pub len: u32,
}

#[no_mangle]
pub extern fn start() {
    thread::spawn(||{
        task::block_on(async {

            loop {

                let mut recv_queue = {
                    let mut request_queue_guard = REQUEST_QUEUE.lock().await;

                    &mut request_queue_guard.1.clone()
                };

                loop {

                    let request : IoRequest = match recv_queue.next().await {
                        Some(v) => v,
                        None => continue
                    };

                    println!("rust: starts working on task.");

                    let mut file = File::open(&request.filename).await.expect("...");

                    let mut data = vec![];
                    let _ = file.read_to_end(&mut data).await.expect("...");

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
pub extern fn poll() -> *const Buffer {

    task::block_on(async {

        let mut recv_queue = {
            let mut request_queue_guard = RESPONSE_QUEUE.lock().await;

            &mut request_queue_guard.1.take().unwrap()
        };

        loop {

            let response: IoResponse = match recv_queue.next().await {
                Some(v) => v,
                None => {

                    continue
                }
            };

            println!("rust: got msg. {:?}", &response);

            let mut buf = response.data.clone();

            let len = buf.len();
            let string_data = CString::new(buf).expect("...");

            return Box::into_raw(Box::new(Buffer { data: string_data.into_raw(), len: len as u32 }));
        }


    })
}

/*
#[no_mangle]
pub extern fn create() -> *const Buffer {
    Box::into_raw(Box::new(Buffer { a: 150, b: 220, c: 3 }))
}

#[no_mangle]
pub extern fn free_buffer(ptr : *mut Buffer) {
    unsafe { Box::from_raw(ptr) };
}
*/
