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
                7 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+1], payload[i+1], payload[i+1], payload[i+1], payload[i+1], gen_random(254)]).to_string(),
                6 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+1], payload[i+1], payload[i+1], payload[i+1], gen_random(254), gen_random(254)]).to_string(),
                5 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+1], payload[i+1], payload[i+1], gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                4 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+1], payload[i+1], gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                3 => generate_random_ipv6(vec![payload[i], payload[i+1], payload[i+1], gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                2 => generate_random_ipv6(vec![payload[i], payload[i+1], gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                1 => generate_random_ipv6(vec![payload[i], gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254), gen_random(254)]).to_string(),
                _ => panic!("This can't happend !!!"),
            });
            println!("{:?}", output_payload);
            return output_payload.join(" ").as_bytes().to_vec();
        }
    }
    println!("{:?}", output_payload);
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