mod encryption{
    pub mod encryption;
    pub mod xor;
    pub mod rc4;
    pub mod rc4_winapi;
}
mod obfuscation{
    pub mod obfuscation;
    pub mod ipv4;
    pub mod ipv6;
}
mod utilities{
    pub mod utilities;
    pub mod winapi;
}
use crate::utilities::utilities::{read_file, write_file};
use crate::encryption::encryption::Encryption;
use crate::obfuscation::obfuscation::Obfuscation;
use clap::Parser;

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
    let mut payload_crpt = Encryption::new(raw_payload);
    match args.encryption.as_str() {
        "xor" => payload_crpt.xor(&args.key),
        "rc4" => payload_crpt.rc4(&args.key),
        "rc4_winapi" => payload_crpt.rc4_winapi(&args.key),
        _ => panic!("[Error] Unknown encryption type: {}", args.encryption),
    };

    let mut payload_obf = Obfuscation::new(payload_crpt.encrypted_data);
    match args.obfuscation.as_str() {
        "ipv4" => payload_obf.ipv4(),
        "ipv6" => payload_obf.ipv6(),
        _ => panic!("[Error] Unknown obfuscation type: {}", args.obfuscation),
    };
    write_file(payload_obf.obfuscated_data,&args.output);
}
