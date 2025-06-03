use crate::components::stun1::{StunAttribute, StunHeader, XorMappedAddress};

pub fn parse_stun_attributes(buf: &[u8]) -> Result,Vec<StunAttribute>, String> {
    let mut attrs = Vec::new();
    let mut pos = 0;

    while pos + 4 <= buf.len() {
        let attr_type = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        let attr_len = u16::from_be_bytes([buf[pos + 2], buf[pos + 3]]) as usize;
        pos += 4;

        if pos+ attr_len > buf.len() {
            return Err("Attribute length exceeds buffer".to_string());
        }

        let attr_value = &buf[pos..pos + attr_len];
        pos += attr_len;

        let padding = (4 - (attr_len % 4)) % 4;
        pos += padding;

        attrs.push(StunAttribute {
            ATTR_Type: attr_type,
            Value: attr_value.to_vec(),
            Length: attr_len as u16,
        })
    }
    Ok(attrs)
}
