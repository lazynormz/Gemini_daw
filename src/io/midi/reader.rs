use super::error::MidiParseError;

pub struct MidiParser<'a> {
    pub data: &'a [u8],
    pub position: usize,
}

#[allow(dead_code)]
impl<'a> MidiParser<'a> {
    pub fn new(data: &'a [u8]) -> MidiParser<'a> {
        MidiParser {
            data,
            position: 0,
        }
    }

    pub fn read_u8_be(&mut self) -> Result<u8, MidiParseError> {
        if self.position >= self.data.len() {
            return Err(MidiParseError::UnexpectedEndOfData);
        }
        let value = self.data[self.position];
        self.position += 1;
        Ok(value)
    }

    pub fn read_u16_be(&mut self) -> Result<u16, MidiParseError> {
        if self.position + 1 >= self.data.len() {
            return Err(MidiParseError::UnexpectedEndOfData);
        }
        let value = u16::from_be_bytes(self.data[self.position..self.position + 2].try_into().unwrap());
        self.position += 2;
        Ok(value)
    }

    pub fn read_u32_be(&mut self) -> Result<u32, MidiParseError> {
        if self.position + 3 >= self.data.len() {
            return Err(MidiParseError::UnexpectedEndOfData);
        }
        let value = u32::from_be_bytes(self.data[self.position..self.position + 4].try_into().unwrap());
        self.position += 4;
        Ok(value)
    }

    pub fn peek_u8_be(&self) -> Result<u8, MidiParseError> {
        if self.position >= self.data.len() {
            return Err(MidiParseError::UnexpectedEndOfData);
        }
        Ok(self.data[self.position])
    }

    pub fn peek_u16_be(&self) -> Result<u16, MidiParseError> {
        if self.position + 1 >= self.data.len() {
            return Err(MidiParseError::UnexpectedEndOfData);
        }
        Ok(u16::from_be_bytes(self.data[self.position..self.position + 2].try_into().unwrap()))
    }

    pub fn peek_u32_be(&self) -> Result<u32, MidiParseError> {
        if self.position + 3 >= self.data.len() {
            return Err(MidiParseError::UnexpectedEndOfData);
        }
        Ok(u32::from_be_bytes(self.data[self.position..self.position + 4].try_into().unwrap()))
    }

    pub fn expect_bytes(&mut self, expected: &[u8]) -> Result<(), MidiParseError> {
        if self.position + expected.len() > self.data.len() {
            return Err(MidiParseError::UnexpectedEndOfData);
        }
        if &self.data[self.position..self.position + expected.len()] != expected {
            return Err(MidiParseError::InvalidHeader);
        }
        self.position += expected.len();
        Ok(())
    }

    pub fn read_vlq(&mut self) -> Result<u32, MidiParseError> {
        let mut value = 0u32;
        for _ in 0..4 {
            let byte = self.read_u8_be()?;
            value = (value << 7) | (byte as u32 & 0x7F);
            if byte & 0x80 == 0 {
                break;
            }
        }
        Ok(value)
    }
}