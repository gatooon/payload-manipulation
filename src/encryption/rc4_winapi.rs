use crate::utilities::winapi::*;
use winapi::um::libloaderapi::LoadLibraryA;
use winapi::um::libloaderapi::GetProcAddress;
use std::{ptr, slice};

pub fn rc4_winapi_encryption(payload: &Vec<u8>, key: &str) -> Result<Vec<u8>, String>{
    let output_payload = payload;
    let function_name = "SystemFunction032";
    let library_name = "Advapi32";
    let p_function_name = lpcstr(function_name);
    let p_library_name = lpcstr(library_name);

    let advapi_handle = unsafe { LoadLibraryA(p_library_name) };
    if advapi_handle == ptr::null_mut() {
        return Err(format!("[Error] Can't find library {}", library_name));
    }
    let rc4_function_addr = unsafe { GetProcAddress(advapi_handle, p_function_name) };
    if rc4_function_addr == ptr::null_mut() {
        return Err(format!("[Error] Can't find function {}", function_name));
    }
    let rc4_encrypte: RC4_ENCRYPTE = unsafe { std::mem::transmute(rc4_function_addr) };
    let output_payload = initialize_ustring(&output_payload);
    let key = key.as_bytes().to_vec();
    let key = initialize_ustring(&key);
    let nt_status = rc4_encrypte(&output_payload, &key);
    if nt_status != 0 {
        return Err(format!("[Error] Something went wrong when running the api function {} from the dll {}", function_name, library_name));
    }
    let output_payload = unsafe { slice::from_raw_parts(output_payload.Buffer as *mut u8, payload.len()) };
    Ok(output_payload.to_vec())
}