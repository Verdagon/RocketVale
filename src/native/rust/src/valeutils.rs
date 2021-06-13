use std::os::raw::c_char;

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
pub struct ValeInterfaceRef {
  unused: [u8; 32],
}

#[repr(C)]
pub struct ValeStr {
    pub length: i32,
    pub contents: [c_char; 0],
}
