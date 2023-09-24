use std::ffi::{c_char, c_void, CString};
use crate::sftp::SftpClient;

mod sftp;
#[no_mangle]
pub extern "C" fn sftp_new_connection(addr: *const c_char) -> *mut c_void {
    let addr_str = unsafe { CString::from_raw(addr as *mut c_char) };
    let addr = addr_str.to_str().unwrap();
    let sftp_client = SftpClient::new_connection(addr);
    Box::into_raw(Box::new(sftp_client)) as *mut c_void
}

#[no_mangle]
pub extern "C" fn sftp_login(client_ptr: *mut c_void, username: *const c_char, password: *const c_char) {
    let username_str = unsafe { CString::from_raw(username as *mut c_char) };
    let password_str = unsafe { CString::from_raw(password as *mut c_char) };
    let username = username_str.to_str().unwrap();
    let password = password_str.to_str().unwrap();
    let client = unsafe { &mut *(client_ptr as *mut SftpClient) };
    client.login(username, password);
}

#[no_mangle]
pub extern "C" fn sftp_put_file(client_ptr: *mut c_void, bytes: *const u8, len: usize, filename: *const c_char) {
    let client = unsafe { &mut *(client_ptr as *mut SftpClient) };
    let buffer = unsafe { std::slice::from_raw_parts(bytes, len) };
    let filename_str = unsafe { CString::from_raw(filename as *mut c_char) };
    let filename = filename_str.to_str().unwrap();
    client.put_file(buffer, filename);
}