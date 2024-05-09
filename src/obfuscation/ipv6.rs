use std::net::Ipv6Addr;
use crate::utilities::utilities::gen_random;

pub fn ipv6_obfuscation(payload: &Vec<u8>) -> Vec<u8>{
    let mut output_payload = vec![];
    let mut i = 0;
    while i + 8 < payload.len(){
        output_payload.push(generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+2], payload[i+3], payload[i+4], payload[i+5], payload[i+6], payload[i+7]]).to_string());
        i += 8;
        if i + 8 > payload.len(){
            let num_padding = payload.len() - i;
            output_payload.push(match num_padding{
                7 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+2], payload[i+3], payload[i+4], payload[i+5], payload[i+6], gen_random(254)]).to_string(),
                6 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+2], payload[i+3], payload[i+4], payload[i+5], gen_random(254), gen_random(254)]).to_string(),
                5 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+2], payload[i+3], payload[i+4], gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                4 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+2], payload[i+3], gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                3 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+2], gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                2 => generate_random_ipv6(vec![payload[i], payload[i+1], gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                1 => generate_random_ipv6(vec![payload[i], gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                _ => panic!("This can't happend !!!"),
            });
            output_payload.insert(0, generate_random_ipv6(vec![gen_random(254), gen_random(254), num_padding as u8, gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string());
            return output_payload.join(" ").as_bytes().to_vec();
        }
    }
    output_payload.insert(0, generate_random_ipv6(vec![gen_random(254), gen_random(254), 0, gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string());
    return output_payload.join(" ").as_bytes().to_vec();
}

fn generate_random_ipv6(bytes_array: Vec<u8>) -> Ipv6Addr {
    let mut output_array = vec![];
    for bytes in bytes_array {
        let mut random = gen_random(254) as u16;
        random = random << 8;
        let bytes_u16 = bytes as u16;
        output_array.push(random | bytes_u16);   
    }
    return Ipv6Addr::new(output_array[0],output_array[1],output_array[2],output_array[3],output_array[4],output_array[5],output_array[6], output_array[7]);
}

pub fn ipv6_unobfuscation(obfuscated: &Vec<u8>) -> Vec<u8>{
    let string_obfuscated = std::str::from_utf8(obfuscated.as_slice()).unwrap();
    let string_splitted: Vec<&str> = string_obfuscated.split(" ").collect();
    let mut output_payload: Vec<u8> = vec![];
    let mut i = 0;
    let mut padding: usize = 0;

    for ip in string_splitted.iter() {
        let ipv6 = ip.parse::<Ipv6Addr>().unwrap().octets();
        println!("{:?}", ipv6);
        if i == 0 {
            padding = ipv6[5] as usize;
            println!("{:?}", padding);
            i += 1;
            continue;
        }
        for n in 0..16 {
            if n % 2 != 0 {
                output_payload.push(ipv6[n]);
            }
        }
        i += 1;
    }
    output_payload.truncate(output_payload.len() - padding);
    return output_payload;
}