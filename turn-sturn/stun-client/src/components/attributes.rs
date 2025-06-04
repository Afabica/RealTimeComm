// SUN attributes parsing

pub const ATTR_MAPPED_ADDRESS: u16 = 0x0001;
pub const ATTR_USERNAME: u16 = 0x0006;
pub const ATTR_MESSAGE_INTEGRITY: u16 = 0x0008;
pub const ATTR_ERROR_CODE: u16 = 0x0009;
pub const ATTR_UNKNOWN_ATTRIBUTES: u16 = 0x000A;
pub const ATTR_REALM: u16 = 0x0014;
pub const ATTR_NONCE: u16 = 0x0015;
pub const ATTR_XOR_MAPPED_ADDRESS: u16 = 0x0020;
pub const ATTR_SOFTWARE: u16  = 0x8022;
pub const  ATTR_ALTERNATE_SERVER: u16  = 0x8023;
pub const ATTR_FINGERPRINT: u16 = 0x8028;


#[derive(Debug, PartialEq)]
pub enum StunAttributeType {
    ATTR_MAPPED_ADDRESS,
    ATTR_USERNAME,
    ATTR_MESSAGE_INTEGRITY,
    ATTR_ERROR_CODE,
    ATTR_UNKNOWN_ATTRIBUTES,
    ATTR_REALM,
    ATTR_NONCE,
    ATTR_XOR_MAPPED_ADDRESS,
    ATTR_SOFTWARE,
    ATTR_ALTERNATE_SERVER,
    ATTR_FINGERPRINT,
    UNKNOWN(u16), // <- Add a catch-all variant
}

impl From<u16> for StunAttributeType {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => Self::ATTR_MAPPED_ADDRESS, 
            0x0006 => Self::ATTR_USERNAME,
            0x0008 => Self::ATTR_MESSAGE_INTEGRITY,
            0x0009 => Self::ATTR_ERROR_CODE,
            0x000A => Self::ATTR_UNKNOWN_ATTRIBUTES,
            0x0014 => Self::ATTR_REALM,
            0x0015 => Self::ATTR_NONCE,
            0x0020 => Self::ATTR_XOR_MAPPED_ADDRESS,
            0x822 => Self::ATTR_SOFTWARE,
            0x8023 => Self::ATTR_ALTERNATE_SERVER,
            0x8028 => Self::ATTR_FINGERPRINT,
            other => Self::UNKNOWN(other),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StunMessageTypes {
    BINDING_REQUEST,
    BINDING_SUCCESS,
    BINDING_ERROR,
    BINDING_INDICATION,
    UNKNOWN(u16),
}

impl From<u16> for StunMessageTypes {
    fn from(value: u16) -> Self {
        match value {
            0x0001 => Self::BINDING_REQUEST,
            0x0101 => Self::BINDING_SUCCESS,
            0x0111 => Self::BINDING_ERROR,
            0x0011 => Self::BINDING_INDICATION,
            other => Self::UNKNOWN(other),
        }
    }
}



