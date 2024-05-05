use std::net::Ipv4Addr;

pub fn ipv4_obfuscation(payload: &Vec<u8>) -> Vec<u8>{
    let mut output_payload = vec![];
    let mut i = 0;
    while i + 4 < payload.len(){
        output_payload.push(Ipv4Addr::new(payload[i], payload[i+1], payload[i+2], payload[i+3]).to_string());
        i += 4;
        if i + 4 > payload.len(){
            let num_padding = payload.len() - i;
            output_payload.push(match num_padding{
                3 => Ipv4Addr::new(payload[i], payload[i+1], payload[i+2], 1).to_string(),
                2 => Ipv4Addr::new(payload[i], payload[i+1], 1, 1).to_string(),
                1 => Ipv4Addr::new(payload[i], 1, 1, 1).to_string(),
                _ => panic!("This can't happend !!!"),
            });
            output_payload.insert(0, Ipv4Addr::new(219, 112, num_padding as u8, 32).to_string());
            return output_payload.join(" ").as_bytes().to_vec();
        }
    }
    output_payload.insert(0, Ipv4Addr::new(219, 112, 0, 32).to_string());
    return output_payload.join(" ").as_bytes().to_vec();
}


pub fn ipv4_unobfuscation(obfuscated: &Vec<u8>) -> Vec<u8>{
    let string_obfuscated = std::str::from_utf8(obfuscated.as_slice()).unwrap();
    let string_splitted: Vec<&str> = string_obfuscated.split(" ").collect();
    let mut output_payload: Vec<u8> = vec![];
    let mut i = 0;
    let mut padding: usize = 0;

    for ip in string_splitted.iter() {
        let ip_splitted: Vec<&str> = ip.split(".").collect();
        if i == 0 {
            padding = ip_splitted[2].parse::<usize>().unwrap();
            i += 1;
            continue;
        }
        for bytes in ip_splitted.iter() {
            output_payload.push(bytes.parse::<u8>().unwrap());
        }
        i += 1;
    }
    output_payload.truncate(output_payload.len() - padding);
    return output_payload;
}