#![forbid(unsafe_code)]

pub const MAGIC: [u8; 4] = *b"ECHO";
pub const PROTOCOL_VERSION: u8 = 1;
pub const HEADER_LEN: usize = 32;
pub const CRC_LEN: usize = 4;
pub const MAX_PAYLOAD_LEN: usize = 16 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SensorFrame {
    pub message_type: u8,
    pub node_id: u32,
    pub sequence: u32,
    pub device_time_us: u64,
    pub subcarrier_count: u16,
    pub flags: u16,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    TooShort,
    BadMagic,
    UnsupportedVersion,
    BadHeaderLength,
    PayloadTooLarge,
    LengthMismatch,
    CrcMismatch,
}

impl SensorFrame {
    pub fn encode(&self) -> Result<Vec<u8>, DecodeError> {
        if self.payload.len() > MAX_PAYLOAD_LEN {
            return Err(DecodeError::PayloadTooLarge);
        }

        let payload_len =
            u16::try_from(self.payload.len()).map_err(|_| DecodeError::PayloadTooLarge)?;
        let mut out = Vec::with_capacity(HEADER_LEN + self.payload.len() + CRC_LEN);

        out.extend_from_slice(&MAGIC);
        out.push(PROTOCOL_VERSION);
        out.push(self.message_type);
        out.extend_from_slice(&(HEADER_LEN as u16).to_le_bytes());
        out.extend_from_slice(&self.node_id.to_le_bytes());
        out.extend_from_slice(&self.sequence.to_le_bytes());
        out.extend_from_slice(&self.device_time_us.to_le_bytes());
        out.extend_from_slice(&self.subcarrier_count.to_le_bytes());
        out.extend_from_slice(&payload_len.to_le_bytes());
        out.extend_from_slice(&self.flags.to_le_bytes());
        out.extend_from_slice(&0_u16.to_le_bytes());
        out.extend_from_slice(&self.payload);

        let crc = crc32_ieee(&out);
        out.extend_from_slice(&crc.to_le_bytes());
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
        if bytes.len() < HEADER_LEN + CRC_LEN {
            return Err(DecodeError::TooShort);
        }

        if bytes[0..4] != MAGIC {
            return Err(DecodeError::BadMagic);
        }

        if bytes[4] != PROTOCOL_VERSION {
            return Err(DecodeError::UnsupportedVersion);
        }

        let header_len = u16::from_le_bytes([bytes[6], bytes[7]]) as usize;
        if header_len != HEADER_LEN {
            return Err(DecodeError::BadHeaderLength);
        }

        let payload_len = u16::from_le_bytes([bytes[26], bytes[27]]) as usize;
        if payload_len > MAX_PAYLOAD_LEN {
            return Err(DecodeError::PayloadTooLarge);
        }

        let expected_len = HEADER_LEN + payload_len + CRC_LEN;
        if bytes.len() != expected_len {
            return Err(DecodeError::LengthMismatch);
        }

        let crc_offset = expected_len - CRC_LEN;
        let expected_crc = u32::from_le_bytes([
            bytes[crc_offset],
            bytes[crc_offset + 1],
            bytes[crc_offset + 2],
            bytes[crc_offset + 3],
        ]);
        let actual_crc = crc32_ieee(&bytes[..crc_offset]);

        if expected_crc != actual_crc {
            return Err(DecodeError::CrcMismatch);
        }

        Ok(Self {
            message_type: bytes[5],
            node_id: u32::from_le_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            sequence: u32::from_le_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            device_time_us: u64::from_le_bytes([
                bytes[16], bytes[17], bytes[18], bytes[19], bytes[20], bytes[21], bytes[22],
                bytes[23],
            ]),
            subcarrier_count: u16::from_le_bytes([bytes[24], bytes[25]]),
            flags: u16::from_le_bytes([bytes[28], bytes[29]]),
            payload: bytes[HEADER_LEN..crc_offset].to_vec(),
        })
    }
}

fn crc32_ieee(bytes: &[u8]) -> u32 {
    let mut crc = 0xffff_ffff_u32;

    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = 0_u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xedb8_8320 & mask);
        }
    }

    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame() -> SensorFrame {
        SensorFrame {
            message_type: 1,
            node_id: 7,
            sequence: 42,
            device_time_us: 123_456,
            subcarrier_count: 56,
            flags: 3,
            payload: vec![1, 2, 3, 4, 5],
        }
    }

    #[test]
    fn frame_round_trip() {
        let input = frame();
        let encoded = input.encode().unwrap();
        let decoded = SensorFrame::decode(&encoded).unwrap();

        assert_eq!(decoded, input);
    }

    #[test]
    fn corrupted_frame_is_rejected() {
        let mut encoded = frame().encode().unwrap();
        encoded[HEADER_LEN] ^= 0xff;

        assert_eq!(SensorFrame::decode(&encoded), Err(DecodeError::CrcMismatch));
    }

    #[test]
    fn bad_magic_is_rejected() {
        let mut encoded = frame().encode().unwrap();
        encoded[0] = b'X';

        assert_eq!(SensorFrame::decode(&encoded), Err(DecodeError::BadMagic));
    }

    #[test]
    fn truncated_frame_is_rejected() {
        let mut encoded = frame().encode().unwrap();
        encoded.pop();

        assert_eq!(
            SensorFrame::decode(&encoded),
            Err(DecodeError::LengthMismatch)
        );
    }
}
