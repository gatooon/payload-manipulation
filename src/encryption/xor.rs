pub fn xor_encryption(payload: &Vec<u8>, key: &str) -> Vec<u8>{
    let mut output_payload = vec![];
    let mut i = 0;
    for p_bytes in payload.iter() {
        output_payload.push(p_bytes ^ key.as_bytes().to_vec()[i]);
        if i >= key.len() - 1 {
            i = 0;
        }else{
            i += 1;
        }
    }
    return output_payload;
}