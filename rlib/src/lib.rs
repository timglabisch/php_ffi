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
use std::time::Duration;
use rand::Rng;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Clone)]
struct IoRequest {
    filename: String,
    token: u64,
}

#[derive(Debug, Clone)]
struct IoResponse {
    token: u64,
    data: Vec<u8>,
    request: IoRequest,
}

lazy_static! {
    static ref TOKEN : AtomicUsize = AtomicUsize::new(0);
    static ref REQUEST_QUEUE : (Mutex<Sender<IoRequest>>, ::std::sync::Mutex<Receiver<IoRequest>>) = {
        let (s, r) = channel(100);
        (Mutex::new(s), ::std::sync::Mutex::new(r))
    };
    static ref RESPONSE_QUEUE : (Mutex<Sender<IoResponse>>, ::std::sync::Mutex<Receiver<IoResponse>>) = {
        let (s, r) = channel(100);
        (Mutex::new(s), ::std::sync::Mutex::new(r))
    };
}

#[repr(C)]
pub struct Buffer {
    pub data: *const c_char,
    pub len: u32,
}

#[no_mangle]
pub extern fn start() {
    thread::spawn(|| {
        task::block_on(async {
            loop {
                let mut request_queue_guard = REQUEST_QUEUE.1.lock().expect("...");

                loop {
                    let request: IoRequest = match request_queue_guard.next().await {
                        Some(v) => v,
                        None => continue
                    };

                    // println!("rust: starts working on task.");

                    let mut file = File::open(&request.filename).await.expect("...");

                    let mut data = vec![];
                    let _ = file.read_to_end(&mut data).await.expect("...");

                    RESPONSE_QUEUE.0.lock().await.try_send(IoResponse {
                        token: request.token,
                        data,
                        request,
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

    ::std::thread::spawn(move || {
        task::block_on(async move {
            let mut rng = rand::thread_rng();
            let dur = Duration::from_millis(rng.gen_range(0, 1000));
            let never = ::async_std::future::pending::<()>();
            ::async_std::future::timeout(dur, never).await.is_err();


            let mut request_queue_guard = REQUEST_QUEUE.0.lock().await;

            &mut request_queue_guard.try_send(IoRequest {
                token,
                filename: filename.to_string(),
            }).expect("...");
        });
    });


    token
}

#[no_mangle]
pub extern fn poll() -> *const Buffer {
    task::block_on(async {
        let mut recv_queue = RESPONSE_QUEUE.1.lock().expect("...");

        loop {
            let response: IoResponse = match recv_queue.next().await {
                Some(v) => v,
                None => {
                    continue;
                }
            };

            // println!("rust: got msg. {:?}", &response);

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
