use openttd_core::endian::BigEndianReader;
use openttd_core::error::CoreError;
use thiserror::Error;

use crate::types::CompressionType;

#[derive(Debug, Error)]
pub enum SavegameError {
    #[error("core error: {0}")]
    Core(#[from] CoreError),
    #[error("invalid magic: {0}")]
    InvalidMagic(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SavegameHeader {
    pub compression: CompressionType,
    pub version: u16,
    pub flags: u16,
}

impl SavegameHeader {
    pub fn parse(buf: &[u8]) -> Result<Self, SavegameError> {
        let mut reader = BigEndianReader::new(buf);
        let magic = reader.read_exact::<4>()?;
        let compression = match &magic {
            b"OTTD" => CompressionType::Lzo,
            b"OTTN" => CompressionType::None,
            b"OTTZ" => CompressionType::Zlib,
            b"OTTX" => CompressionType::Lzma,
            _ => {
                return Err(SavegameError::InvalidMagic(
                    String::from_utf8_lossy(&magic).to_string(),
                ))
            }
        };
        let version = reader.read_u16()?;
        let flags = reader.read_u16()?;
        Ok(Self {
            compression,
            version,
            flags,
        })
    }
}
