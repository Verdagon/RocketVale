#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket::data::ByteUnit;
use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;
mod valeutils;
use crate::valeutils::CCallback2;
use crate::valeutils::ValeInterfaceRef;
use crate::rocket::tokio::io::AsyncReadExt;
//use tokio::io::AsyncReadExt;
use tokio::runtime::Builder;
use crate::valeutils::ValeStr;
use crate::valeutils::make_vale_str;
use crate::valeutils::load_vale_str;
use std::path::PathBuf;
use rocket::Data;


lazy_static! {
    static ref VALE_MUTEX: Mutex<Option<CCallback2<ValeInterfaceRef, *mut ValeStr, *mut ValeStr, *mut ValeStr>>> = Mutex::new(None);
}

#[get("/<path..>")]
fn handleGet(path: PathBuf) -> String {
    let locked = VALE_MUTEX.lock().unwrap();

    let request_str = path.into_os_string().into_string().unwrap();
    
    let callback = locked.as_ref().unwrap();

    let response_str = 
        unsafe {
            load_vale_str(callback.call(make_vale_str(&request_str), make_vale_str(&"".to_owned())))
        };
    response_str
}

#[post("/<path..>", format = "application/json", data = "<data>")]
async fn handlePost(path: PathBuf, data: Data<'_>) -> String {
    let mut stream = data.open(10 * ByteUnit::MB);
    let mut body = "".to_owned();
    stream.read_to_string(&mut body).await.expect("Couldn't read body");

    let locked = VALE_MUTEX.lock().unwrap();

    let request_str = path.into_os_string().into_string().unwrap();
    
    let callback = locked.as_ref().unwrap();
    let response_str = 
        unsafe {
            load_vale_str(callback.call(make_vale_str(&request_str), make_vale_str(&body)))
        };
    response_str
}

#[no_mangle]
fn rocketvale_rust_run_server(callback: &CCallback2<ValeInterfaceRef, *mut ValeStr, *mut ValeStr, *mut ValeStr>) {
    {
        let mut locked = VALE_MUTEX.lock().unwrap();
        *locked = Some(callback.clone());
    }

    let mut rt = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_io()
        .thread_name("my-custom-name")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();
    let rocket =
        rt.block_on(
            rocket::build()
                .mount("/", routes![handlePost, handleGet])
                .launch()
        ).expect("wat");
}
