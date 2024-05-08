use crate::obfuscation::ipv4::ipv4_obfuscation;
use crate::obfuscation::ipv6::ipv6_obfuscation;

use super::ipv4::ipv4_unobfuscation;
pub struct Obfuscation{
    pub raw_data: Vec<u8>,
    pub obfuscated_data: Vec<u8>,
}

impl Obfuscation{
    pub fn new(payload: Vec<u8>) -> Obfuscation{
        Obfuscation{
            raw_data: payload,
            obfuscated_data: vec![],
        }
    }

    pub fn ipv4(&mut self){
        self.obfuscated_data = ipv4_obfuscation(&self.raw_data);
    }

    pub fn ipv4_undo(&mut self){
        self.obfuscated_data = ipv4_unobfuscation(&self.raw_data);
    }

    pub fn ipv6(&mut self){
        self.obfuscated_data = ipv6_obfuscation(&self.raw_data);
    }
}