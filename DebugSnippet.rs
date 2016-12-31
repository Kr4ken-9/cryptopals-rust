use std::str;

fn main() {
    println!("{}", bytes_to_base64(hex_to_bytes("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")));
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex_chars = hex.chars();
    let mut byte_array_vector: Vec<u8> = Vec::with_capacity(48 as usize);
    let mut is_high = true;
    let mut current_value:u8 = 0;
    for x in hex_chars {
        let byte_val = match x {
            '0'...'9' => (x as i16) - 48,
            'a'...'f' => (x as i16) - 87,
            _ => panic!("Invalid character!"),
        };
        if is_high {
            current_value = (byte_val as u8) << 4;
        } else {
            byte_array_vector.push(current_value | (byte_val as u8));
        }
        
        is_high = !is_high;
    }

    return byte_array_vector;
}

fn bytes_to_base64(bytes: Vec<u8>) -> String {
    let mut current_bit = 0;
    let mut current_base64:u8 = 0;
    let mut base_64 = String::with_capacity(64);

    for x in bytes {
        for i in 0..8 {
            let y = (x >> (7 - i)) & 0x01;

            if current_bit == 0 {
                current_base64 = y; 
            } else {
                current_base64 = (current_base64 << 1) | y;
            }
            current_bit = current_bit + 1;
            if current_bit > 5 {
                let char_value = match current_base64 {
                    0...25 => (current_base64 + 65) as char,
                    26...51 => (current_base64 + 71) as char,
                    52...61 => (current_base64 - 4) as char,
                    62 => '+',
                    63 => '/',
                    _ => panic!("Technicalities"),
                };
                base_64.push(char_value);
                current_bit = 0;
            }
        }
    }
    return base_64;
}