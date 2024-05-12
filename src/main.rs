#[allow(dead_code)]
mod encryption{
    pub mod encryption;
    pub mod xor;
    pub mod rc4;
    pub mod rc4_winapi;
}
#[allow(dead_code)]
mod obfuscation{
    pub mod obfuscation;
    pub mod ipv4;
    pub mod ipv6;
}
#[allow(dead_code)]
mod utilities{
    pub mod utilities;
    pub mod winapi;
}
use crate::utilities::utilities::{read_file, write_file};
use crate::encryption::encryption::Encryption;
use crate::obfuscation::obfuscation::Obfuscation;
use clap::Parser;
use std::str::from_utf8;

#[derive(Parser, Debug)]
struct Args {
    /// Path to raw payload
    #[arg(short, long)]
    input: String,

    /// Path to output payload
    #[arg(short, long)]
    output: String,
    
    /// Encryption Type
    #[arg(long)]
    encryption: String,
    
    /// Encryption Key
    #[arg(long)]
    key: String,

    /// Obfuscation Type
    #[arg(long)]
    obfuscation: String,
}

fn main() {
    let args = Args::parse();

    let raw_payload = read_file(&args.input);
    println!("------------------Raw Payload------------------\n{:?}\n------------------Raw Payload------------------", raw_payload);
    let mut payload_crpt = Encryption::new(raw_payload);
    match args.encryption.as_str() {
        "xor" => payload_crpt.xor(&args.key),
        "rc4" => payload_crpt.rc4(&args.key),
        "rc4_winapi" => payload_crpt.rc4_winapi(&args.key),
        _ => panic!("[Error] Unknown encryption type: {}", args.encryption),
    };
    println!("------------------Encrypted Payload------------------\n{:?}\n------------------Encrypted Payload------------------", payload_crpt.altered_data);
    let mut payload_obf = Obfuscation::new(payload_crpt.altered_data);
    match args.obfuscation.as_str() {
        "ipv4" => payload_obf.ipv4(),
        "ipv6" => payload_obf.ipv6(),
        _ => panic!("[Error] Unknown obfuscation type: {}", args.obfuscation),
    };
    println!("------------------Obfuscated Payload------------------\n{:?}\n------------------Obfuscated Payload------------------", from_utf8(payload_obf.altered_data.as_ref()).unwrap());
    write_file(payload_obf.altered_data,&args.output);
}
