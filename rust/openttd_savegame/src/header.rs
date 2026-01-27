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

    /// Write the header to a byte buffer
    pub fn write(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(8);

        // Write magic bytes based on compression type
        let magic = match self.compression {
            CompressionType::Lzo => b"OTTD",
            CompressionType::None => b"OTTN",
            CompressionType::Zlib => b"OTTZ",
            CompressionType::Lzma => b"OTTX",
        };
        buf.extend_from_slice(magic);

        // Write version and flags in big-endian
        buf.extend_from_slice(&self.version.to_be_bytes());
        buf.extend_from_slice(&self.flags.to_be_bytes());

        buf
    }

    /// Get the expected header size in bytes
    pub const fn size() -> usize {
        8 // 4 bytes magic + 2 bytes version + 2 bytes flags
    }
}
