use crate::encryption::rc4::rc4_encryption;
use crate::encryption::xor::xor_encryption;
use super::rc4_winapi::rc4_winapi_encryption;

pub struct Encryption{
    pub raw_data: Vec<u8>,
    pub altered_data: Vec<u8>,
}

impl Encryption{

    pub fn new(payload: Vec<u8>) -> Encryption{
        Encryption{
            raw_data: payload,
            altered_data: vec![],
        }
    }

    pub fn xor(&mut self, key: &str){
        self.altered_data = xor_encryption(&self.raw_data, key);
    }

    pub fn rc4(&mut self, key: &str){
        self.altered_data = rc4_encryption(&self.raw_data, key);
    }
    pub fn rc4_winapi(&mut self, key: &str){
        self.altered_data = match rc4_winapi_encryption(&self.raw_data, key) {
            Ok(data) => data,
            Err(err) => panic!("{}", err),
        };
    }
}