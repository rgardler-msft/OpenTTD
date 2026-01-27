use crate::error::CoreError;

pub struct BigEndianReader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> BigEndianReader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    pub fn read_u8(&mut self) -> Result<u8, CoreError> {
        if self.remaining() < 1 {
            return Err(CoreError::UnexpectedEof);
        }
        let value = self.buf[self.pos];
        self.pos += 1;
        Ok(value)
    }

    pub fn read_i8(&mut self) -> Result<i8, CoreError> {
        Ok(self.read_u8()? as i8)
    }

    pub fn read_u16(&mut self) -> Result<u16, CoreError> {
        let bytes = self.read_exact::<2>()?;
        Ok(u16::from_be_bytes(bytes))
    }

    pub fn read_i16(&mut self) -> Result<i16, CoreError> {
        let bytes = self.read_exact::<2>()?;
        Ok(i16::from_be_bytes(bytes))
    }

    pub fn read_u24(&mut self) -> Result<u32, CoreError> {
        if self.remaining() < 3 {
            return Err(CoreError::UnexpectedEof);
        }
        let b1 = self.buf[self.pos] as u32;
        let b2 = self.buf[self.pos + 1] as u32;
        let b3 = self.buf[self.pos + 2] as u32;
        self.pos += 3;
        Ok((b1 << 16) | (b2 << 8) | b3)
    }

    pub fn read_u32(&mut self) -> Result<u32, CoreError> {
        let bytes = self.read_exact::<4>()?;
        Ok(u32::from_be_bytes(bytes))
    }

    pub fn read_i32(&mut self) -> Result<i32, CoreError> {
        let bytes = self.read_exact::<4>()?;
        Ok(i32::from_be_bytes(bytes))
    }

    pub fn read_u64(&mut self) -> Result<u64, CoreError> {
        let bytes = self.read_exact::<8>()?;
        Ok(u64::from_be_bytes(bytes))
    }

    pub fn read_i64(&mut self) -> Result<i64, CoreError> {
        let bytes = self.read_exact::<8>()?;
        Ok(i64::from_be_bytes(bytes))
    }

    pub fn read_exact<const N: usize>(&mut self) -> Result<[u8; N], CoreError> {
        if self.remaining() < N {
            return Err(CoreError::UnexpectedEof);
        }
        let mut out = [0u8; N];
        out.copy_from_slice(&self.buf[self.pos..self.pos + N]);
        self.pos += N;
        Ok(out)
    }
}
