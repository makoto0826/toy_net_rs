use std::io::Write;

use crate::errors::Error;

const ECHO_REQUEST_TYPE: u8 = 8;
const ECHO_REQUEST_CODE: u8 = 0;

const ECHO_REPLY_MESSAGE_TYPE: u8 = 0;
const ECHO_REPLY_MESSAGE_CODE: u8 = 0;

pub static ALPHABET: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();

#[derive(Debug)]
pub struct EchoRequest<'a> {
    pub sequence: u16,
    pub identity: u16,
    pub payload: &'a [u8],
}

impl<'a> EchoRequest<'a> {
    pub fn encode(&self, data: &mut [u8]) -> Result<(), Error> {
        // data[0]   TYPE
        // data[1]   CODE
        // data[2-3] CHECKSUM
        // data[4-5] IDENTITY
        // data[6-7] SEQUENCE
        // data[8-]  PAYLOAD

        data[0] = ECHO_REQUEST_TYPE;
        data[1] = ECHO_REQUEST_CODE;
        data[4] = (self.identity >> 8) as u8;
        data[5] = (self.identity & 0xFF) as u8;
        data[6] = (self.sequence >> 8) as u8;
        data[7] = (self.sequence & 0xFF) as u8;

        if let Err(_) = (&mut data[8..]).write(&self.payload) {
            return Err(Error::InvalidPacket);
        }

        checksum(data);
        Ok(())
    }
}

pub struct EchoReply<'a> {
    pub sequence: u16,
    pub identity: u16,
    pub payload: &'a [u8],
}

impl<'a> EchoReply<'a> {
    pub fn decode(data: &'a [u8]) -> Result<Self, Error> {
        if data[0] != ECHO_REPLY_MESSAGE_TYPE || data[1] != ECHO_REPLY_MESSAGE_CODE {
            return Err(Error::InvalidPacket);
        }

        let identity = u16::from(data[5]) + (u16::from(data[4]) << 8);
        let sequence = u16::from(data[7]) + (u16::from(data[6]) << 8);

        Ok(Self {
            identity,
            sequence,
            payload: &data[8..],
        })
    }
}

fn checksum(data: &mut [u8]) {
    let mut sum: u32 = 0;

    for word in data.chunks(2) {
        let mut part = u16::from(word[0]) << 8;

        if word.len() > 1 {
            part += u16::from(word[1]);
        }

        sum = sum.wrapping_add(u32::from(part));
    }

    while (sum >> 16) > 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }

    let sum = !sum as u16;
    data[2] = (sum >> 8) as u8;
    data[3] = (sum & 0xFF) as u8;
}
