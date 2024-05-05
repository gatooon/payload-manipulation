use rc4::{Key, Rc4, KeyInit, StreamCipher};
use rc4::consts::U6;
use rc4::cipher::generic_array::GenericArray;

pub fn rc4_encryption(payload: &Vec<u8>, key: &str) -> Vec<u8>{
    let formated_key: &GenericArray<u8, U6> = Key::<U6>::from_slice(key.as_bytes());
    let mut rc4 = Rc4::<_>::new(formated_key);
    let mut data = payload.to_owned();
    rc4.apply_keystream(&mut data);
    return data;
}