#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use lazy_static::lazy_static; // 1.4.0
use std::sync::Mutex;
use std::mem::size_of;
mod valeutils;
use crate::valeutils::CCallback1;
use crate::valeutils::ValeInterfaceRef;
use crate::valeutils::ValeStr;



lazy_static! {
    static ref VALE_MUTEX: Mutex<Option<CCallback1<ValeInterfaceRef, *mut ValeStr, *mut ValeStr>>> = Mutex::new(None);
}

#[get("/")]
fn index() -> String {
    let locked = VALE_MUTEX.lock().unwrap();

    let request_str = "hello".to_owned();
    let size = size_of::<ValeStr>() + request_str.len() + 1;
    // let mut vec = Vec::with_capacity(size);
    // for _i in 0..size {
    //     vec.push(0);
    // }

    println!("hello from index!");

    unsafe {
        println!("bork a");
        let mallockd = memalloc::allocate(size);
        let mut request_vale_str_ptr = std::mem::transmute::<*mut u8, *mut ValeStr>(mallockd);
        (*request_vale_str_ptr).length = request_str.len() as i32;
        println!("bork b");
        for i in 0..request_str.len() {
            let needed_byte = request_str.chars().nth(i).unwrap() as i8;
            *(*request_vale_str_ptr).contents.as_mut_ptr().offset(i as isize) = needed_byte;
        }
        println!("bork c");
        *(*request_vale_str_ptr).contents.as_mut_ptr().offset(request_str.len() as isize) = 0;

        println!("bork d");
        let callback = locked.as_ref().unwrap();
        let response_vale_str_ptr = callback.call(request_vale_str_ptr);
        // let response_vale_str_ptr =
        //     (locked.as_ref().unwrap().function)(
        //         locked.as_ref().unwrap().tag,
        //         request_vale_str_ptr);
        println!("bork e");
        println!("bork f1. ptr: {:p}", response_vale_str_ptr);
        let response_vale_str_contents_ptr = &mut (*response_vale_str_ptr).contents;
        println!("bork f2. contents ptr: {:p}", response_vale_str_contents_ptr);
        println!("bork f3. size: {:?}", (*response_vale_str_ptr).length as usize);

        let mut response_str = String::with_capacity((*response_vale_str_ptr).length as usize);
        for i in 0..(*response_vale_str_ptr).length {
            response_str.push(*response_vale_str_contents_ptr.as_ptr().offset(i as isize) as u8 as char);
        }

        println!("bork g");

        response_str
    }
}

#[no_mangle]
fn rocketvale_rust_run_server(callback: &CCallback1<ValeInterfaceRef, *mut ValeStr, *mut ValeStr>) {
    println!("zork a");
    {
        let mut locked = VALE_MUTEX.lock().unwrap();
        println!("zork a2");
        *locked = Some((*callback).clone());
    }
    println!("zork b");

    rocket::ignite()
        .mount("/request", routes![index])
        .launch();
}
