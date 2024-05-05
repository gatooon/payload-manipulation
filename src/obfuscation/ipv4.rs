use std::net::Ipv4Addr;
use rand::Rng;

pub fn ipv4_obfuscation(payload: &Vec<u8>) -> Vec<u8>{
    let mut output_payload = vec![];
    let mut i = 0;
    let mut random = rand::thread_rng();
    while i + 4 < payload.len(){
        output_payload.push(Ipv4Addr::new(payload[i], payload[i+1], payload[i+2], payload[i+3]).to_string());
        i += 4;
        if i + 4 > payload.len(){
            let num_padding = payload.len() - i;
            output_payload.push(match num_padding{
                3 => Ipv4Addr::new(payload[i], payload[i+1], payload[i+2], gen_random(254)).to_string(),
                2 => Ipv4Addr::new(payload[i], payload[i+1], gen_random(254), gen_random(254)).to_string(),
                1 => Ipv4Addr::new(payload[i], gen_random(254), gen_random(254), gen_random(254)).to_string(),
                _ => panic!("This can't happend !!!"),
            });
            output_payload.insert(0, Ipv4Addr::new(gen_random(254), gen_random(254), obf_padding(num_padding), gen_random(254)).to_string());
            return output_payload.join(" ").as_bytes().to_vec();
        }
    }
    output_payload.insert(0, Ipv4Addr::new(gen_random(254), gen_random(254), obf_padding(0), gen_random(254)).to_string());
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
            let padding_bytes: Vec<char> = ip_splitted[2].chars().collect();
            padding = padding_bytes[padding_bytes.len() - 1].to_string().parse::<usize>().unwrap();
            println!("{:?}", padding);
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

fn gen_random(range: u8) -> u8{
    let mut random = rand::thread_rng();
    return random.gen_range(0..=range);
}

fn obf_padding(padding: usize) -> u8{
    return format!("{}{}", gen_random(23), padding).parse::<u8>().unwrap();
}