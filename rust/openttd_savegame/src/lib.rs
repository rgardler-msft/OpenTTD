pub mod chunk;
pub mod gamma;
pub mod header;
pub mod savegame;
pub mod types;

// Re-export main types
pub use header::{SavegameError as HeaderError, SavegameHeader};
pub use savegame::{Chunk, ChunkData, SavegameReader, SavegameWriter};
pub use types::CompressionType;
