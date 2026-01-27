/// OpenTTD savegame reader and writer
use crate::chunk::{
    parse_array_chunk, parse_riff_chunk, parse_table_chunk, ChunkHeader, ChunkType,
};
use crate::header;
use crate::types::CompressionType;
use flate2::read::ZlibDecoder;
use openttd_core::error::CoreError;
use std::io::Read;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SavegameError {
    #[error("core error: {0}")]
    Core(#[from] CoreError),
    #[error("header error: {0}")]
    Header(#[from] header::SavegameError),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("lzma error: {0}")]
    Lzma(#[from] lzma_rs::error::Error),
    #[error("unsupported compression: {0:?}")]
    UnsupportedCompression(CompressionType),
    #[error("invalid savegame format")]
    InvalidFormat,
}

/// A parsed chunk from a savegame
#[derive(Debug)]
pub struct Chunk {
    pub tag: String,
    pub chunk_type: ChunkType,
    pub data: ChunkData,
}

#[derive(Debug)]
pub enum ChunkData {
    Riff(Vec<u8>),
    Array(Vec<(usize, Vec<u8>)>),
    Table {
        header: crate::chunk::TableHeader,
        records: Vec<(usize, Vec<u8>)>,
    },
}

/// Main savegame reader
pub struct SavegameReader {
    header: header::SavegameHeader,
    decompressed_data: Vec<u8>,
}

impl SavegameReader {
    /// Create a new reader from raw savegame data
    pub fn new(data: &[u8]) -> Result<Self, SavegameError> {
        // Parse header
        let header = header::SavegameHeader::parse(data)?;

        // Extract compressed data (skip 8-byte header)
        if data.len() < 8 {
            return Err(SavegameError::InvalidFormat);
        }
        let compressed_data = &data[8..];

        // Decompress based on compression type
        let decompressed_data = match header.compression {
            CompressionType::None => compressed_data.to_vec(),
            CompressionType::Zlib => {
                let mut decoder = ZlibDecoder::new(compressed_data);
                let mut decompressed = Vec::new();
                decoder.read_to_end(&mut decompressed)?;
                decompressed
            }
            CompressionType::Lzma => {
                // OTTX format uses XZ compression (LZMA2)
                let mut decompressed = Vec::new();
                lzma_rs::xz_decompress(&mut compressed_data.as_ref(), &mut decompressed)?;
                decompressed
            }
            CompressionType::Lzo => {
                return Err(SavegameError::UnsupportedCompression(CompressionType::Lzo));
            }
        };

        Ok(Self {
            header,
            decompressed_data,
        })
    }

    /// Get the savegame header
    pub fn header(&self) -> &header::SavegameHeader {
        &self.header
    }

    /// Read all chunks from the savegame
    pub fn read_chunks(&self) -> Result<Vec<Chunk>, SavegameError> {
        let mut chunks = Vec::new();
        let mut offset = 0;
        let data = &self.decompressed_data;

        loop {
            // Check if we have enough data for a header
            if offset + 5 > data.len() {
                break;
            }

            // Parse chunk header
            let (header, bytes_read) = ChunkHeader::parse(&data[offset..])?;
            offset += bytes_read;

            // Check for end marker
            if header.is_end_marker() {
                break;
            }

            // Parse chunk data based on type
            let chunk_data = match header.chunk_type {
                ChunkType::Riff => {
                    let (data, bytes_read) = parse_riff_chunk(&header, &data[offset..])?;
                    offset += bytes_read;
                    ChunkData::Riff(data)
                }
                ChunkType::Array | ChunkType::SparseArray => {
                    let (items, bytes_read) = parse_array_chunk(&header, &data[offset..])?;
                    offset += bytes_read;
                    ChunkData::Array(items)
                }
                ChunkType::Table | ChunkType::SparseTable => {
                    let (table_header, records, bytes_read) =
                        parse_table_chunk(&header, &data[offset..])?;
                    offset += bytes_read;
                    ChunkData::Table {
                        header: table_header,
                        records,
                    }
                }
            };

            chunks.push(Chunk {
                tag: header.tag_string(),
                chunk_type: header.chunk_type,
                data: chunk_data,
            });
        }

        Ok(chunks)
    }
}

/// Writer functionality for creating savegames
pub struct SavegameWriter {
    header: header::SavegameHeader,
    chunks: Vec<u8>,
}

impl SavegameWriter {
    /// Create a new savegame writer
    pub fn new(version: u16, compression: CompressionType) -> Self {
        Self {
            header: header::SavegameHeader {
                compression,
                version,
                flags: 0,
            },
            chunks: Vec::new(),
        }
    }

    /// Add a RIFF chunk
    pub fn add_riff_chunk(&mut self, tag: &[u8; 4], data: &[u8]) -> Result<(), SavegameError> {
        // Write chunk tag
        self.chunks.extend_from_slice(tag);

        // Calculate mode byte and length encoding
        let length = data.len();
        let mode_byte = if length < (1 << 24) {
            // Length fits in 24 bits
            0x00 // RIFF type in lower 4 bits
        } else {
            // Use upper 4 bits for high part of length
            ((length >> 24) as u8) << 4
        };
        self.chunks.push(mode_byte);

        // Write 24-bit length (big endian, as per OpenTTD format)
        self.chunks.push(((length >> 16) & 0xFF) as u8);
        self.chunks.push(((length >> 8) & 0xFF) as u8);
        self.chunks.push((length & 0xFF) as u8);

        // Write data
        self.chunks.extend_from_slice(data);

        Ok(())
    }

    /// Finalize the savegame and return the compressed data
    pub fn finalize(mut self) -> Result<Vec<u8>, SavegameError> {
        // Add end-of-savegame marker
        self.chunks.extend_from_slice(&[0, 0, 0, 0, 0]);

        // Build header using the header's write method
        let mut result = self.header.write();

        // Compress chunk data
        let compressed = match self.header.compression {
            CompressionType::None => self.chunks,
            CompressionType::Zlib => {
                use flate2::write::ZlibEncoder;
                use flate2::Compression;
                use std::io::Write;

                let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
                encoder.write_all(&self.chunks)?;
                encoder.finish()?
            }
            CompressionType::Lzma => {
                let mut compressed = Vec::new();
                lzma_rs::lzma_compress(&mut self.chunks.as_slice(), &mut compressed)?;
                compressed
            }
            CompressionType::Lzo => {
                return Err(SavegameError::UnsupportedCompression(CompressionType::Lzo));
            }
        };

        result.extend_from_slice(&compressed);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {
        // Create a simple savegame
        let mut writer = SavegameWriter::new(295, CompressionType::None);
        writer.add_riff_chunk(b"TEST", b"Hello, World!").unwrap();
        let data = writer.finalize().unwrap();

        // Read it back
        let reader = SavegameReader::new(&data).unwrap();
        assert_eq!(reader.header().version, 295);
        assert_eq!(reader.header().compression, CompressionType::None);

        let chunks = reader.read_chunks().unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].tag, "TEST");

        match &chunks[0].data {
            ChunkData::Riff(data) => {
                assert_eq!(data, b"Hello, World!");
            }
            _ => panic!("Expected RIFF chunk"),
        }
    }

    #[test]
    fn test_compressed_round_trip() {
        // Test with zlib compression
        let mut writer = SavegameWriter::new(295, CompressionType::Zlib);
        writer
            .add_riff_chunk(b"DATA", b"Compressed data test")
            .unwrap();
        let data = writer.finalize().unwrap();

        // Verify header
        assert_eq!(&data[0..4], b"OTTZ");

        // Read it back
        let reader = SavegameReader::new(&data).unwrap();
        assert_eq!(reader.header().compression, CompressionType::Zlib);

        let chunks = reader.read_chunks().unwrap();
        assert_eq!(chunks.len(), 1);

        match &chunks[0].data {
            ChunkData::Riff(data) => {
                assert_eq!(data, b"Compressed data test");
            }
            _ => panic!("Expected RIFF chunk"),
        }
    }
}
