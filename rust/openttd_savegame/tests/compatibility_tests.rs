/// Compatibility tests using real OpenTTD save files
use openttd_savegame::{ChunkData, CompressionType, SavegameReader};
use std::fs;
use std::path::Path;

fn test_savegame(path: &Path) {
    println!("Testing savegame: {}", path.display());

    // Read the file
    let data = fs::read(path).expect(&format!("Failed to read {}", path.display()));

    // Parse the savegame
    let reader = match SavegameReader::new(&data) {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed to parse {}: {:?}", path.display(), e);
        }
    };

    // Check header
    let header = reader.header();
    println!("  Version: {}", header.version);
    println!("  Compression: {:?}", header.compression);
    println!("  Flags: 0x{:04x}", header.flags);

    // Try to read chunks
    let chunks = reader.read_chunks().expect("Failed to read chunks");
    println!("  Found {} chunks", chunks.len());

    // Print chunk summary
    for chunk in &chunks {
        let data_info = match &chunk.data {
            ChunkData::Riff(data) => format!("RIFF, {} bytes", data.len()),
            ChunkData::Array(items) => format!("Array, {} items", items.len()),
            ChunkData::Table { records, .. } => format!("Table, {} records", records.len()),
        };
        println!("    {}: {}", chunk.tag, data_info);
    }

    // Verify we found expected chunks
    assert!(chunks.len() > 0, "No chunks found in savegame");

    // The test saves might be minimal and not have all chunks
    // Just verify we can parse them without errors
    println!(
        "  Successfully parsed savegame with {} chunks",
        chunks.len()
    );
}

#[test]
fn test_regression_save() {
    let path = Path::new("../../regression/regression/test.sav");
    if path.exists() {
        test_savegame(path);
    } else {
        eprintln!("Warning: {} not found, skipping test", path.display());
    }
}

#[test]
fn test_stationlist_save() {
    let path = Path::new("../../regression/stationlist/test.sav");
    if path.exists() {
        test_savegame(path);
    } else {
        eprintln!("Warning: {} not found, skipping test", path.display());
    }
}

#[test]
fn test_create_and_read_savegame() {
    use openttd_savegame::SavegameWriter;

    // Create a minimal savegame
    let mut writer = SavegameWriter::new(295, CompressionType::Zlib);

    // Add some test chunks
    writer.add_riff_chunk(b"MAPS", b"Map data here").unwrap();
    writer.add_riff_chunk(b"PLYR", b"Player data").unwrap();
    writer.add_riff_chunk(b"VEHS", b"Vehicle data").unwrap();

    // Finalize
    let data = writer.finalize().unwrap();

    // Now read it back
    let reader = SavegameReader::new(&data).unwrap();
    assert_eq!(reader.header().version, 295);
    assert_eq!(reader.header().compression, CompressionType::Zlib);

    let chunks = reader.read_chunks().unwrap();
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0].tag, "MAPS");
    assert_eq!(chunks[1].tag, "PLYR");
    assert_eq!(chunks[2].tag, "VEHS");
}
