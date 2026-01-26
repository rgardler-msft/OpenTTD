#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionType {
    Lzo,
    None,
    Zlib,
    Lzma,
}
