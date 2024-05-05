use winapi::shared::minwindef::DWORD;
use winapi::shared::ntdef::PVOID;
use winapi::shared::ntdef::NTSTATUS;
use std::ffi::CString;

pub type RC4_ENCRYPTE = extern "system" fn(&USTRING, &USTRING) -> NTSTATUS;

#[repr(C)]
pub struct USTRING{
    pub Length: DWORD,
    pub MaximumLength: DWORD,
    pub Buffer: PVOID,
}

pub fn initialize_ustring(data: &Vec<u8>) -> USTRING{
    return USTRING{
        Length: data.len() as u32,
        MaximumLength: data.len() as u32,
        Buffer: data.as_ptr() as *mut winapi::ctypes::c_void,
    };
}

pub fn lpwstr(data: &str) -> *const u16{
    let mut output_string = vec![];
    for bytes in data.as_bytes().to_vec().iter(){
        output_string.push(bytes.to_owned() as u16);
    }
    return output_string.as_ptr();
}

pub fn lpcstr(data: &str) -> *const i8{
    return CString::new(data).unwrap().into_raw();
}