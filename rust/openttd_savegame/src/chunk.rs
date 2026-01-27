/// Chunk handling for OpenTTD savegames
use crate::gamma;
use openttd_core::endian::BigEndianReader;
use openttd_core::error::CoreError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkType {
    Riff = 0,        // Binary blob chunk
    Array = 1,       // Sequential array (deprecated)
    SparseArray = 2, // Sparse array (deprecated)
    Table = 3,       // Self-describing table
    SparseTable = 4, // Self-describing sparse table
}

impl TryFrom<u8> for ChunkType {
    type Error = CoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0x0F {
            0 => Ok(ChunkType::Riff),
            1 => Ok(ChunkType::Array),
            2 => Ok(ChunkType::SparseArray),
            3 => Ok(ChunkType::Table),
            4 => Ok(ChunkType::SparseTable),
            _ => Err(CoreError::InvalidData("Invalid chunk type".into())),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkHeader {
    pub tag: [u8; 4],
    pub chunk_type: ChunkType,
    pub mode_byte: u8,
}

impl ChunkHeader {
    pub fn parse(buf: &[u8]) -> Result<(Self, usize), CoreError> {
        if buf.len() < 5 {
            return Err(CoreError::BufferTooSmall);
        }

        let mut tag = [0u8; 4];
        tag.copy_from_slice(&buf[0..4]);

        // Check for end-of-savegame marker
        if tag == [0, 0, 0, 0] {
            return Ok((
                Self {
                    tag,
                    chunk_type: ChunkType::Riff,
                    mode_byte: 0,
                },
                5,
            ));
        }

        let mode_byte = buf[4];
        let chunk_type = ChunkType::try_from(mode_byte)?;

        Ok((
            Self {
                tag,
                chunk_type,
                mode_byte,
            },
            5,
        ))
    }

    pub fn is_end_marker(&self) -> bool {
        self.tag == [0, 0, 0, 0]
    }

    pub fn tag_string(&self) -> String {
        String::from_utf8_lossy(&self.tag).to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    I8 = 1,
    U8 = 2,
    I16 = 3,
    U16 = 4,
    I32 = 5,
    U32 = 6,
    I64 = 7,
    U64 = 8,
    StringId = 9,
    String = 10,
    Struct = 11,
}

impl TryFrom<u8> for DataType {
    type Error = CoreError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value & 0x0F {
            1 => Ok(DataType::I8),
            2 => Ok(DataType::U8),
            3 => Ok(DataType::I16),
            4 => Ok(DataType::U16),
            5 => Ok(DataType::I32),
            6 => Ok(DataType::U32),
            7 => Ok(DataType::I64),
            8 => Ok(DataType::U64),
            9 => Ok(DataType::StringId),
            10 => Ok(DataType::String),
            11 => Ok(DataType::Struct),
            _ => Err(CoreError::InvalidData(format!(
                "Invalid data type: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableField {
    pub data_type: DataType,
    pub key: String,
    pub is_list: bool, // 0x10 flag indicates if it's a list
}

#[derive(Debug, Clone)]
pub struct TableHeader {
    pub fields: Vec<TableField>,
}

impl TableHeader {
    pub fn parse(buf: &[u8]) -> Result<(Self, usize), CoreError> {
        let mut offset = 0;
        let mut fields = Vec::new();

        // Read header size
        let (header_size, bytes_read) = gamma::decode_gamma(&buf[offset..])?;
        offset += bytes_read;

        if header_size == 0 {
            return Err(CoreError::InvalidData("Table has no header".into()));
        }

        let header_end = offset + (header_size as usize) - 1;

        // Read field definitions
        while offset < header_end {
            let type_byte = buf[offset];
            offset += 1;

            if type_byte == 0 {
                // End of field list
                break;
            }

            let is_list = (type_byte & 0x10) != 0;
            let data_type = DataType::try_from(type_byte)?;

            // Read key length
            let (key_length, bytes_read) = gamma::decode_gamma(&buf[offset..])?;
            offset += bytes_read;

            // Read key
            let key = String::from_utf8(buf[offset..offset + key_length as usize].to_vec())
                .map_err(|e| CoreError::InvalidData(e.to_string()))?;
            offset += key_length as usize;

            fields.push(TableField {
                data_type,
                key,
                is_list,
            });
        }

        Ok((Self { fields }, offset))
    }
}

/// Parse a RIFF chunk
pub fn parse_riff_chunk(header: &ChunkHeader, buf: &[u8]) -> Result<(Vec<u8>, usize), CoreError> {
    let mut offset = 0;

    // Read the length (24-bit value + upper bits from mode)
    if buf.len() < 3 {
        return Err(CoreError::BufferTooSmall);
    }

    let mut reader = BigEndianReader::new(&buf[offset..]);
    let length_low = reader.read_u24()? as usize;
    offset += 3;

    // Upper 4 bits of mode byte contribute to length
    let length = length_low | (((header.mode_byte >> 4) as usize) << 24);

    if buf.len() < offset + length {
        return Err(CoreError::BufferTooSmall);
    }

    let data = buf[offset..offset + length].to_vec();
    Ok((data, offset + length))
}

/// Parse an ARRAY or SPARSE_ARRAY chunk
pub fn parse_array_chunk(
    header: &ChunkHeader,
    buf: &[u8],
) -> Result<(Vec<(usize, Vec<u8>)>, usize), CoreError> {
    let mut offset = 0;
    let mut items = Vec::new();
    let mut implicit_index = 0;

    loop {
        // Read item size
        let (size_plus_one, bytes_read) = gamma::decode_gamma(&buf[offset..])?;
        offset += bytes_read;

        if size_plus_one == 0 {
            // End of array
            break;
        }

        let size = (size_plus_one - 1) as usize;

        let index = if header.chunk_type == ChunkType::SparseArray {
            // Read explicit index
            let (idx, bytes_read) = gamma::decode_gamma(&buf[offset..])?;
            offset += bytes_read;
            idx as usize
        } else {
            // Use implicit index for regular array
            let idx = implicit_index;
            implicit_index += 1;
            idx
        };

        if size > 0 {
            // Read item data
            if buf.len() < offset + size {
                return Err(CoreError::BufferTooSmall);
            }

            let data = buf[offset..offset + size].to_vec();
            offset += size;
            items.push((index, data));
        } else if header.chunk_type == ChunkType::Array {
            // Empty slot in regular array
            implicit_index += 1;
        }
    }

    Ok((items, offset))
}

/// Parse a TABLE or SPARSE_TABLE chunk
pub fn parse_table_chunk(
    header: &ChunkHeader,
    buf: &[u8],
) -> Result<(TableHeader, Vec<(usize, Vec<u8>)>, usize), CoreError> {
    let mut offset = 0;

    // Parse table header
    let (table_header, bytes_read) = TableHeader::parse(&buf[offset..])?;
    offset += bytes_read;

    // Parse records (similar to array parsing)
    let mut items = Vec::new();
    let mut implicit_index = 0;

    loop {
        // Read record size
        let (size_plus_one, bytes_read) = gamma::decode_gamma(&buf[offset..])?;
        offset += bytes_read;

        if size_plus_one == 0 {
            // End of records
            break;
        }

        let size = (size_plus_one - 1) as usize;

        let index = if header.chunk_type == ChunkType::SparseTable {
            // Read explicit index
            let (idx, bytes_read) = gamma::decode_gamma(&buf[offset..])?;
            offset += bytes_read;
            idx as usize
        } else {
            // Use implicit index for regular table
            let idx = implicit_index;
            implicit_index += 1;
            idx
        };

        if size > 0 {
            // Read record data
            if buf.len() < offset + size {
                return Err(CoreError::BufferTooSmall);
            }

            let data = buf[offset..offset + size].to_vec();
            offset += size;
            items.push((index, data));
        } else if header.chunk_type == ChunkType::Table {
            // Empty slot in regular table
            implicit_index += 1;
        }
    }

    Ok((table_header, items, offset))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_header_parse() {
        // Test normal chunk header
        let buf = b"MAPS\x00additional_data";
        let (header, bytes_read) = ChunkHeader::parse(buf).unwrap();
        assert_eq!(header.tag, *b"MAPS");
        assert_eq!(header.chunk_type, ChunkType::Riff);
        assert_eq!(bytes_read, 5);
        assert_eq!(header.tag_string(), "MAPS");
    }

    #[test]
    fn test_end_marker() {
        // Test end-of-savegame marker
        let buf = b"\x00\x00\x00\x00\x00";
        let (header, _) = ChunkHeader::parse(buf).unwrap();
        assert!(header.is_end_marker());
    }

    #[test]
    fn test_chunk_types() {
        assert_eq!(ChunkType::try_from(0).unwrap(), ChunkType::Riff);
        assert_eq!(ChunkType::try_from(1).unwrap(), ChunkType::Array);
        assert_eq!(ChunkType::try_from(2).unwrap(), ChunkType::SparseArray);
        assert_eq!(ChunkType::try_from(3).unwrap(), ChunkType::Table);
        assert_eq!(ChunkType::try_from(4).unwrap(), ChunkType::SparseTable);
        assert!(ChunkType::try_from(15).is_err());
    }

    #[test]
    fn test_data_types() {
        assert_eq!(DataType::try_from(1).unwrap(), DataType::I8);
        assert_eq!(DataType::try_from(2).unwrap(), DataType::U8);
        assert_eq!(DataType::try_from(10).unwrap(), DataType::String);
        assert_eq!(DataType::try_from(11).unwrap(), DataType::Struct);
        assert!(DataType::try_from(0).is_err());
        assert!(DataType::try_from(12).is_err());
    }
}
