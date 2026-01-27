use openttd_savegame::header::SavegameHeader;
use openttd_savegame::types::CompressionType;

#[test]
fn parse_ottd_header() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"OTTD");
    buf.extend_from_slice(&0x0200u16.to_be_bytes());
    buf.extend_from_slice(&0x0001u16.to_be_bytes());

    let header = SavegameHeader::parse(&buf).expect("parse header");
    assert_eq!(header.compression, CompressionType::Lzo);
    assert_eq!(header.version, 0x0200);
    assert_eq!(header.flags, 0x0001);
}

#[test]
fn parse_ottn_header() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"OTTN");
    buf.extend_from_slice(&0x0150u16.to_be_bytes());
    buf.extend_from_slice(&0x0000u16.to_be_bytes());

    let header = SavegameHeader::parse(&buf).expect("parse header");
    assert_eq!(header.compression, CompressionType::None);
    assert_eq!(header.version, 0x0150);
    assert_eq!(header.flags, 0x0000);
}

#[test]
fn parse_ottz_header() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"OTTZ");
    buf.extend_from_slice(&0x0123u16.to_be_bytes());
    buf.extend_from_slice(&0x0042u16.to_be_bytes());

    let header = SavegameHeader::parse(&buf).expect("parse header");
    assert_eq!(header.compression, CompressionType::Zlib);
    assert_eq!(header.version, 0x0123);
    assert_eq!(header.flags, 0x0042);
}

#[test]
fn parse_ottx_header() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"OTTX");
    buf.extend_from_slice(&0xFFFFu16.to_be_bytes());
    buf.extend_from_slice(&0x8000u16.to_be_bytes());

    let header = SavegameHeader::parse(&buf).expect("parse header");
    assert_eq!(header.compression, CompressionType::Lzma);
    assert_eq!(header.version, 0xFFFF);
    assert_eq!(header.flags, 0x8000);
}

#[test]
fn parse_invalid_magic() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"NOPE");
    buf.extend_from_slice(&0x0001u16.to_be_bytes());
    buf.extend_from_slice(&0x0000u16.to_be_bytes());

    let err = SavegameHeader::parse(&buf).expect_err("should fail");
    let message = err.to_string();
    assert!(message.contains("invalid magic"));
}

#[test]
fn round_trip_ottd() {
    let original = SavegameHeader {
        compression: CompressionType::Lzo,
        version: 0x0200,
        flags: 0x0001,
    };

    let bytes = original.write();
    assert_eq!(bytes.len(), 8);

    let parsed = SavegameHeader::parse(&bytes).expect("parse header");
    assert_eq!(parsed.compression, original.compression);
    assert_eq!(parsed.version, original.version);
    assert_eq!(parsed.flags, original.flags);
}

#[test]
fn round_trip_ottn() {
    let original = SavegameHeader {
        compression: CompressionType::None,
        version: 0x0150,
        flags: 0x0000,
    };

    let bytes = original.write();
    assert_eq!(bytes.len(), 8);

    let parsed = SavegameHeader::parse(&bytes).expect("parse header");
    assert_eq!(parsed.compression, original.compression);
    assert_eq!(parsed.version, original.version);
    assert_eq!(parsed.flags, original.flags);
}

#[test]
fn round_trip_ottz() {
    let original = SavegameHeader {
        compression: CompressionType::Zlib,
        version: 0x0123,
        flags: 0x0042,
    };

    let bytes = original.write();
    assert_eq!(bytes.len(), 8);

    let parsed = SavegameHeader::parse(&bytes).expect("parse header");
    assert_eq!(parsed.compression, original.compression);
    assert_eq!(parsed.version, original.version);
    assert_eq!(parsed.flags, original.flags);
}

#[test]
fn round_trip_ottx() {
    let original = SavegameHeader {
        compression: CompressionType::Lzma,
        version: 0xFFFF,
        flags: 0x8000,
    };

    let bytes = original.write();
    assert_eq!(bytes.len(), 8);

    let parsed = SavegameHeader::parse(&bytes).expect("parse header");
    assert_eq!(parsed.compression, original.compression);
    assert_eq!(parsed.version, original.version);
    assert_eq!(parsed.flags, original.flags);
}

#[test]
fn header_size_constant() {
    assert_eq!(SavegameHeader::size(), 8);
}

#[test]
fn write_header_bytes_correct() {
    let header = SavegameHeader {
        compression: CompressionType::Zlib,
        version: 0x1234,
        flags: 0x5678,
    };

    let bytes = header.write();

    // Check magic
    assert_eq!(&bytes[0..4], b"OTTZ");

    // Check version (big-endian)
    assert_eq!(bytes[4], 0x12);
    assert_eq!(bytes[5], 0x34);

    // Check flags (big-endian)
    assert_eq!(bytes[6], 0x56);
    assert_eq!(bytes[7], 0x78);
}
