mod util;

fn main() {
    let first_bytes: Vec<u8> = util::hex_to_bytes("1c0111001f010100061a024b53535009181c");
    let second_bytes: Vec<u8> = util::hex_to_bytes("686974207468652062756c6c277320657965");

    println!("{}", util::bytes_to_hex(util::xor(first_bytes, second_bytes)));
}