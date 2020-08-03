use std::ffi::CString;
use libc::c_char;
use ipc_channel::ipc::IpcSender;

pub struct APIClient{
    api_key: String,
    sender: IpcSender<Vec<u8>>,
}
/*impl APIClient{
    pub fn new(key: String) -> Self{
        Self {api_key: key}
    }
}*/
/*#[no_mangle]
pub extern "C" fn register_app(ptr: *mut c_char) -> API{
    let cstr = unsafe {
        CString::from_raw(ptr)
    };
    API::new(cstr.into_string().unwrap())
}*/