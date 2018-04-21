use bytes::{BigEndian, Buf};

#[derive(Debug)]
pub(crate) enum Error {
    IncompleteMessage,
    IntegerOverflow
}

pub(crate) struct Message<B> {
    bytes: B
}

impl<B> Message<B> {
    pub fn new(bytes: B) -> Message<B> {
        Message { bytes }
    }
}

impl<B: Buf> Message<B> {
    #[inline]
    pub fn read_uint(&mut self) -> Result<u64, Error> {
        if self.bytes.remaining() < 1 {
            return Err(Error::IncompleteMessage);
        }
        let u7_or_len = self.bytes.get_u8();
        if u7_or_len < 128 {
            return Ok(u7_or_len as u64);
        }
        let len = !u7_or_len;
        if self.bytes.remaining() < len as usize {
            return Err(Error::IncompleteMessage);
        }
        Ok(self.bytes.get_uint::<BigEndian>(len as usize + 1))
    }

    #[inline]
    pub fn read_int(&mut self) -> Result<i64, Error> {
        let bits = self.read_uint()?;
        if bits & 1 == 0 {
            Ok((bits >> 1) as i64)
        } else {
            Ok(-1 + -((bits >> 1) as i64))
        }
    }

    #[inline]
    pub fn read_float(&mut self) -> Result<f64, Error> {
        let bits = self.read_uint()?;
        Ok(f64::from_bits(bits.swap_bytes()))
    }

    #[inline]
    pub fn read_bool(&mut self) -> Result<bool, Error> {
        match self.read_uint()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::IntegerOverflow)
        }
    }
}
