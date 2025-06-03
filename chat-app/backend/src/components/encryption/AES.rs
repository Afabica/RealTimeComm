use std::fs::File;
use std::io::{self, Read, BufReader};

const BLOCK_SIZE: usize = 32;

pub fn string_to_hex(input: &str) -> String {
    input
        .chars()
        .map(|c| format!("{:02X}", c as u8))
        .collect::<Vec<String>>() // Collect into a vector of hex strings
        .join(" ") // Join with spaces for readability
}

// XOR each byte in the state matrix with the corresponding round key byte
pub fn add_round_key(state: &mut [[u8; 4]; 4], round_key: &[[u8; 4]; 4]) {
    for row in 0..4 {
        for col in 0..4 {
            state[row][col] ^= round_key[row][col];
        }
    }
}



//pub fn convert_to_u8_array(hex_matrix: &[Vec<String>]) -> Option<[u8; 256]> {
//    let hex_values: Vec<String> = hex_matrix.iter().flatten().cloned().collect();
//
//    if hex_values.len() != 256 {
//        return None;
//    }
//
//    let mut result_array = [0u8; 256];
//
//    for (i, hex_str) in hex_values.iter().enumerate() {
//        if let Ok(value) = u8::from_str_radix(hex_str, 16) {
//            result_array[i] = value;
//        } else {
//            return None;
//        }
//    }
//
//    Some(result_array)
//}


//pub fn convert_to_u8_array(hex_strings: Vec<Vec<String>>) -> Option<Vec<u8>> {
//
//    let mut result: Vec<u8> = Vec::new();
//
//
//
//    for hex in hex_strings {
//        // Ensure each string has exactly 2 hex characters
//        if hex.len() != 2 {
//            eprintln!("❌ Invalid hex string length: {}", hex);
//            return None;
//        }
//
//        // Try parsing hex string into u8
//        match u8::from_str_radix(&hex, 16) {
//            Ok(byte) => result.push(byte),
//            Err(_) => {
//                eprintln!("❌ Failed to convert hex to u8: {}", hex);
//                return None;
//            }
//        }
//    }
//
//    Some(result)
//}
//
/// Converts a nested vector of hexadecimal strings (`Vec<Vec<String>>`)
/// into a flat vector of bytes (`Vec<u8>`).
/// Returns `None` if any conversion fails.
/// Converts a nested vector of hexadecimal strings (`Vec<Vec<String>>`)
/// into a flat vector of bytes (`Vec<u8>`).
/// Returns `None` if any conversion fails.
pub fn convert_to_u8_array(hex_values: Vec<Vec<String>>) -> Option<Vec<u8>> {
    // Flatten the nested Vec<Vec<String>> into a single Vec<String>
    let flattened: Vec<String> = hex_values.into_iter().flatten().collect();

    // Initialize a vector to store the resulting bytes.
    let mut result = Vec::with_capacity(flattened.len());

    // Convert each hex string to a u8 value.
    for hex in flattened {
        // Check that the hex string is exactly 2 characters long (one byte)
        if hex.len() != 2 {
            eprintln!("❌ Invalid hex string length (expected 2, got {}): {}", hex.len(), hex);
            return None;
        }
        match u8::from_str_radix(&hex, 16) {
            Ok(byte) => result.push(byte),
            Err(e) => {
                eprintln!("❌ Failed to convert hex '{}' to u8: {}", hex, e);
                return None;
            }
        }
    }
    Some(result)
}



// Read file and convert to hex
pub fn read_character_by_character_with_converting_to_hex(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BLOCK_SIZE];

    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        }

        let hex_block: String = buffer[..bytes_read]
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(" ");

        println!("Block: {}", hex_block);
    }

    Ok(())
}

pub fn gf_mult(mut a: u8, mut b: u8) -> u8 {
    let mut p: u8 = 0;
    let irreducible: u8 = 0x1B;

    for _ in 0..8 {
        if (b & 1) != 0 {
            p ^= a;
        }

        let carry = (a & 0x80) != 0;
        a <<= 1; // Corrected shift operation
        if carry {
            a ^= irreducible;
        }

        b >>= 1;
    }
    p
}

pub fn gf_inverse(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }

    let mut t0 = 0;
    let mut t1 = 1;
    let mut r0: u16 = 0x11B; // Extended to u16 for modulo operations
    let mut r1 = a as u16;

    while r1 != 1 {
        let mut q = 0;
        let mut r = r0;

        while r >= r1 {
            r ^= r1;
            q ^= 1;
        }

        r0 = r1;
        r1 = r;

        let t = t0 ^ gf_mult(q as u8, t1);
        t0 = t1;
        t1 = t;
    }

    t1
}

pub fn process_hex_matrix(hex_blocks: Vec<Vec<String>>) -> Vec<Vec<String>> {
    hex_blocks
        .iter()
        .map(|row| {
            row.iter()
                .map(|hex_str| {
                    let byte = u8::from_str_radix(hex_str, 16).expect("Invalid hex value");
                    let inverse = gf_inverse(byte);
                    format!("{:02X}", inverse) // Fixed format issue
                })
                .collect::<Vec<String>>()
        })
        .collect()
}

pub fn read_32_bytes(filename: &str) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut blocks = Vec::new();

    loop {
        let mut buffer = [0u8; BLOCK_SIZE];
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }
        blocks.push(buffer[..bytes_read].to_vec());
    }

    Ok(blocks)
}

pub fn convert_to_hex(byte_blocks: Vec<Vec<u8>>) -> Vec<Vec<String>> {
    byte_blocks
        .iter()
        .map(|block| block.iter().map(|&b| format!("{:02X}", b)).collect())
        .collect()
}

pub fn read_by_blocks(filename: &str) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; BLOCK_SIZE];

    while let Ok(_) = reader.read_exact(&mut buffer) {
        let hex_block: String = buffer
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect::<Vec<String>>()
            .join(" ");

        println!("Block: {}", hex_block);
    }

    Ok(())
}

// AES Key Expansion (fixed errors)
pub fn key_expansion(key: &[u8; 16]) -> [[u8; 4]; 44] {
    let mut expanded_key = [[0u8; 4]; 44];

    for i in 0..4 {
        expanded_key[i] = [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];
    }

    for i in 4..44 {
        let mut temp = expanded_key[i - 1];

        if i % 4 == 0 {
            temp.rotate_left(1);
            for j in 0..4 {
                temp[j] = gf_mult(temp[j], 2); // Apply GF multiplication
            }
            temp[0] ^= 1 << (i / 4 - 1);
        }

        for j in 0..4 {
            expanded_key[i][j] = expanded_key[i - 4][j] ^ temp[j];
        }
    }

    expanded_key
}

