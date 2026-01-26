use openttd_savegame::header::SavegameHeader;
use openttd_savegame::types::CompressionType;

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
fn parse_invalid_magic() {
    let mut buf = Vec::new();
    buf.extend_from_slice(b"NOPE");
    buf.extend_from_slice(&0x0001u16.to_be_bytes());
    buf.extend_from_slice(&0x0000u16.to_be_bytes());

    let err = SavegameHeader::parse(&buf).expect_err("should fail");
    let message = err.to_string();
    assert!(message.contains("invalid magic"));
}
