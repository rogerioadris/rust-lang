use std::io::{Error, ErrorKind};

use crate::byte::Byte;

#[derive(Copy, Clone, Debug)]
pub struct I2c<'l> {
    address: u8,
    pub byte: Byte<'l>,
}

impl<'l> I2c<'l> {
    pub fn default(address: u8) -> I2c<'l> {
        I2c {
            address,
            byte: Byte::default(),
        }
    }

    pub fn new(address: u8, byte: Byte<'l>) -> I2c<'l> {
        I2c { address, byte }
    }

    pub fn write(&mut self) -> Result<(u8, u8), Error> {
        let status = (self.address, self.byte.number());
        match self.address {
            1..=128 => {
                // Write data i2c device
                Ok(status)
            }
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Address: {}, Byte: {}", status.0, status.1),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::I2c;
    use std::io::ErrorKind;

    #[test]
    fn it_invalid_address() {
        let mut i2c = I2c::default(129);
        let result = i2c.write().map_err(|e| e.kind());
        let expected = Err(ErrorKind::InvalidData);
        assert_eq!(expected, result);

        let mut i2c = I2c::default(0);
        let result = i2c.write().map_err(|e| e.kind());
        let expected = Err(ErrorKind::InvalidData);
        assert_eq!(expected, result);
    }

    #[test]
    fn it_valid_address() {
        let mut i2c = I2c::default(1);
        let result = i2c.write().unwrap();
        let expected = (i2c.address, i2c.byte.number());
        assert_eq!(expected, result);

        let mut i2c = I2c::default(128);
        let result = i2c.write().unwrap();
        let expected = (i2c.address, i2c.byte.number());
        assert_eq!(expected, result);
    }
}
