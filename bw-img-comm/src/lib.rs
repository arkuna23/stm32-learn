#![no_std]

pub const FULL_DATA_BYTE: u8 = 0x03;
pub const COMM_ACK_BYTE: u8 = 0x04;

/// Two communications per signal, first is the opcode, second is the data(if exists)
#[derive(Eq, PartialEq, Debug)]
pub enum Signal<'a> {
    FullData(&'a [u8]),
    CommACK,
}

#[derive(Debug)]
pub enum Error {
    InvalidOpcode,
    MissingData,
}

impl<'a> Signal<'a> {
    /// Convert the signal to a byte array
    /// return a tuple of the first byte(opcode) and the rest of the bytes
    pub fn to_bytes(&self) -> (u8, Option<&[u8]>) {
        match self {
            Signal::FullData(data) => (FULL_DATA_BYTE, Some(data)),
            Signal::CommACK => (COMM_ACK_BYTE, None),
        }
    }

    pub fn new(op: u8, data: Option<&'a [u8]>) -> Result<Self, Error> {
        Ok(match op {
            FULL_DATA_BYTE => Signal::FullData(data.ok_or(Error::MissingData)?),
            COMM_ACK_BYTE => Signal::CommACK,
            _ => return Err(Error::InvalidOpcode),
        })
    }

    #[inline(always)]
    pub fn has_data(opcode: u8) -> bool {
        opcode == FULL_DATA_BYTE
    }
}
