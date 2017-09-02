
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
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

pub fn bytes_to_hex(bytes: Vec<u8>) -> String {
    let mut hex = String::with_capacity(bytes.len() * 2);
    for x in bytes {
        let high_order: u8 = x >> 4;
        let low_order: u8 = x & 0xF;
        let high_value: char = value_to_hex(high_order);
        hex.push(high_value);
        let low_value: char = value_to_hex(low_order);
        hex.push(low_value);
    }
    return hex;
}

fn value_to_hex(value: u8) -> char {
    let char_value = match value {
        0...9 => (value + 48) as char,
        10...15 => (value + 87) as char,
        _ => panic!("Invalid byte!"),
    };
    return char_value;
}

pub fn bytes_to_base64(bytes: Vec<u8>) -> String {
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
                    _ => panic!("Technicalities")
                };
                base_64.push(char_value);
                current_bit = 0;
            }
        }
    }
    return base_64;
}

pub fn xor(first_bytes: Vec<u8>, second_bytes: Vec<u8>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::with_capacity(first_bytes.len());
    for x in 0..first_bytes.len() {
        output.push(first_bytes[x] ^ second_bytes[x]);
    }
    return output;
}

struct Pair {
    output: str,
    score: i32
}

pub fn score(bytes: Vec<u8>) -> Vec<Pair> {
    let mut output: Vec<Pair> = Vec::with_capacity(bytes.len());
    let keys:str = "`1234567890-=qwertyuiop[]\\asdfghjkl;'zxcvbnm,./~!@#$%^&*()_+QWERTYUIOP{}|ASDFGHJKL:\"ZXCVBNM<>?";
    
    for key in keys {
        let mut score:i32 = 0;
        let byte_key:Vec<u8> = hex_to_bytes(value_to_hex(key as u8));
        
        let result:Vec<u8> = xor(bytes, byte_key);
        let hex_result = bytes_to_hex(result);
        
        for character in hex_result {
            let upper_character:str = character.to_uppercase().collect::<Vec<_>>();
            
            match upper_character {
                "E" | "T" | "A" | "O" | "I" | "N" | "S" | "H" | "R" | "D" | "L" | "U" => score += 1,
                _ => panic!("Technicalities")
            }
        }
        
        output.push(Pair { hex_result, score });
    }
}