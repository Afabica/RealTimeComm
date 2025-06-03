// Crypto, XOR logic, etc.
 

//pub fn validate_auth(attrs: &[Atribute], key_store: &HashMap<String, Vec<u8>>) -> bool {
//    
//}

pub fn build_error_code(code: u16, phrase: &str) -> Vec<u8> {
    let mut attr = vec![0x00, 0x09];
    let length = 4 + phrase.len();
    attr.extend_from_slice(&(length as u16).to_be_bytes());
    attr.extend_from_slice(&[0, 0, (code / 100) as u8, (code % 100) as u8]);
    attr.extend_from_slice(phrase.as_bytes());
    while attr.len() % 4 != 0 {
        attr.push(0); 
    }

    attr
}

//pub fn compute_fingerprint(data: &[u8]) -> u32 {
//    let mut digest = crc32::Digest::new(crc32::IEEE);
//    digest.write(data);
//    digest.sum32() ^ 0x5354553e // XOR with 'STUN'
//}

 
