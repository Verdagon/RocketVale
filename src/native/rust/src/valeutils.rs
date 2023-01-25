use std::os::raw::c_char;
use std::ffi::CStr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCallback1<T : Copy, P, R> {
    this: T,
    function: unsafe extern "C" fn(T, P) -> R,
}
impl<T : Copy, P, R> CCallback1<T, P, R> {
    pub unsafe fn call(self: &CCallback1<T, P, R>, param1: P) -> R {
        return (self.function)(self.this, param1);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCallback2<T : Copy, P1, P2, R> {
    this: T,
    function: unsafe extern "C" fn(T, P1, P2) -> R,
}
impl<T : Copy, P1, P2, R> CCallback2<T, P1, P2, R> {
    pub unsafe fn call(self: &CCallback2<T, P1, P2, R>, param1: P1, param2: P2) -> R {
        return (self.function)(self.this, param1, param2);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ValeInterfaceRef {
  unused: [u8; 32],
}

#[repr(C)]
pub struct ValeStr {
    pub length: i32,
    pub contents: [c_char; 0],
}

pub unsafe fn make_vale_str(source: &str) -> *mut ValeStr {
    let size = std::mem::size_of::<ValeStr>() + source.len() + 1;
    let mallockd = memalloc::allocate(size);
    println!("allocated for string, len {:?} address {:p}", size, mallockd);
    let mut vale_str_ptr = std::mem::transmute::<*mut u8, *mut ValeStr>(mallockd);
    (*vale_str_ptr).length = source.len() as i32;
    for i in 0..source.len() {
        let needed_byte = source.chars().nth(i).unwrap() as i8;
        *(*vale_str_ptr).contents.as_mut_ptr().offset(i as isize) = needed_byte;
    }
    *(*vale_str_ptr).contents.as_mut_ptr().offset(source.len() as isize) = 0;
    vale_str_ptr
}

pub unsafe fn load_vale_str(source: *mut ValeStr) -> String {
    let response_vale_str_contents_ptr = std::mem::transmute::<&mut [c_char; 0], *mut i8>(&mut (*source).contents);
    let response_c_str: &CStr = CStr::from_ptr(response_vale_str_contents_ptr);
    let response_c_str_slice: &str = response_c_str.to_str().unwrap();
    let response_str = response_c_str_slice.to_owned();
    response_str
}
