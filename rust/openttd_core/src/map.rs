//! Map and tile types matching OpenTTD C++ structures for save compatibility.
//!
//! This module defines the core map and tile structures that mirror the C++
//! implementation exactly to maintain savegame compatibility.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fmt;

/// Type-safe wrapper for tile indices (matches C++ StrongType<uint32_t, TileIndexTag>)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct TileIndex(pub u32);

impl TileIndex {
    /// Invalid tile marker (matches C++ INVALID_TILE)
    pub const INVALID: TileIndex = TileIndex(0xFFFF_FFFF);

    /// Check if this is a valid tile index
    pub fn is_valid(self) -> bool {
        self.0 != Self::INVALID.0
    }
}

impl Default for TileIndex {
    fn default() -> Self {
        Self::INVALID
    }
}

impl fmt::Display for TileIndex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TileIndex({})", self.0)
    }
}

/// Tile types (matches C++ TileType enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TileType {
    Clear = 0,
    Railway = 1,
    Road = 2,
    House = 3,
    Trees = 4,
    Station = 5,
    Water = 6,
    Void = 7,
    Industry = 8,
    TunnelBridge = 9,
    Object = 10,
}

/// Tropic zones (matches C++ TropicZone enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TropicZone {
    Normal = 0,
    Desert = 1,
    Rainforest = 2,
}

/// Direction enum (matches C++ Direction)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Direction {
    North = 0,
    NorthEast = 1,
    East = 2,
    SouthEast = 3,
    South = 4,
    SouthWest = 5,
    West = 6,
    NorthWest = 7,
    Invalid = 0xFF,
}

/// Base tile data structure (8 bytes, matches C++ Tile::TileBase)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct TileBase {
    /// Tile type (bits 4-7), bridge above (2-3), climate zone (0-1)
    pub type_height: u8,
    /// Height of the northern corner
    pub height: u8,
    /// Index to town/industry/station (depends on tile type)
    pub m2: u16,
    /// Owner information
    pub m1: u8,
    /// General purpose data field 3
    pub m3: u8,
    /// General purpose data field 4
    pub m4: u8,
    /// General purpose data field 5
    pub m5: u8,
}

impl TileBase {
    /// Extract tile type from type_height field
    pub fn tile_type(&self) -> TileType {
        // Bits 4-7 contain the tile type
        let type_val = (self.type_height >> 4) & 0x0F;
        // Safety: We mask to 4 bits so max value is 15, all valid TileTypes are 0-10
        unsafe { std::mem::transmute(type_val.min(10)) }
    }

    /// Set tile type in type_height field
    pub fn set_tile_type(&mut self, tile_type: TileType) {
        self.type_height = (self.type_height & 0x0F) | ((tile_type as u8) << 4);
    }

    /// Get bridge above bits (2-3)
    pub fn bridge_above(&self) -> u8 {
        (self.type_height >> 2) & 0x03
    }

    /// Set bridge above bits
    pub fn set_bridge_above(&mut self, bridge: u8) {
        self.type_height = (self.type_height & 0xF3) | ((bridge & 0x03) << 2);
    }

    /// Get climate zone bits (0-1)
    pub fn climate_zone(&self) -> TropicZone {
        let zone = self.type_height & 0x03;
        // Safety: We mask to 2 bits so max value is 3, TropicZone values are 0-2
        unsafe { std::mem::transmute(zone.min(2)) }
    }

    /// Set climate zone bits
    pub fn set_climate_zone(&mut self, zone: TropicZone) {
        self.type_height = (self.type_height & 0xFC) | (zone as u8);
    }
}

/// Extended tile data (4 bytes, matches C++ Tile::TileExtended)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct TileExtended {
    /// General purpose field 6 (NewGRF support)
    pub m6: u8,
    /// General purpose field 7 (NewGRF support)
    pub m7: u8,
    /// General purpose field 8
    pub m8: u16,
}

/// Complete tile structure (12 bytes total, matches C++ Tile)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[repr(C)]
pub struct Tile {
    /// Base tile information (8 bytes)
    pub base: TileBase,
    /// Extended tile information (4 bytes)
    pub extended: TileExtended,
}

impl Tile {
    /// Create a new void tile
    pub fn new_void() -> Self {
        Self {
            base: TileBase {
                type_height: (TileType::Void as u8) << 4,
                height: 0,
                m2: 0,
                m1: 0,
                m3: 0,
                m4: 0,
                m5: 0,
            },
            extended: TileExtended {
                m6: 0,
                m7: 0,
                m8: 0,
            },
        }
    }

    /// Create a new clear tile at specified height
    pub fn new_clear(height: u8) -> Self {
        Self {
            base: TileBase {
                type_height: (TileType::Clear as u8) << 4,
                height,
                m2: 0,
                m1: 0,
                m3: 0,
                m4: 0,
                m5: 0,
            },
            extended: TileExtended {
                m6: 0,
                m7: 0,
                m8: 0,
            },
        }
    }
}

/// Map dimensions and management (matches C++ Map static class)
pub struct Map {
    /// Logarithmic X size (actual size is 1 << log_x)
    pub log_x: u32,
    /// Logarithmic Y size (actual size is 1 << log_y)
    pub log_y: u32,
    /// Actual X dimension
    pub size_x: u32,
    /// Actual Y dimension
    pub size_y: u32,
    /// Total number of tiles
    pub size: u32,
    /// Mask for wrapping tile indices
    pub tile_mask: u32,
    /// The actual tile data
    pub tiles: Vec<Tile>,
}

impl Map {
    /// Create a new map with given log dimensions
    /// Size must be between 6 (64x64) and 12 (4096x4096)
    pub fn new(log_x: u32, log_y: u32) -> Result<Self, String> {
        if log_x < 6 || log_x > 12 || log_y < 6 || log_y > 12 {
            return Err(format!("Invalid map dimensions: 2^{} x 2^{}", log_x, log_y));
        }

        let size_x = 1 << log_x;
        let size_y = 1 << log_y;
        let size = size_x * size_y;
        let tile_mask = size - 1;

        // Initialize all tiles as void
        let tiles = vec![Tile::new_void(); size as usize];

        Ok(Map {
            log_x,
            log_y,
            size_x,
            size_y,
            size,
            tile_mask,
            tiles,
        })
    }

    /// Get a tile by index
    pub fn get_tile(&self, index: TileIndex) -> Option<&Tile> {
        if index.is_valid() && index.0 < self.size {
            Some(&self.tiles[index.0 as usize])
        } else {
            None
        }
    }

    /// Get a mutable tile by index
    pub fn get_tile_mut(&mut self, index: TileIndex) -> Option<&mut Tile> {
        if index.is_valid() && index.0 < self.size {
            Some(&mut self.tiles[index.0 as usize])
        } else {
            None
        }
    }

    /// Convert X,Y coordinates to tile index
    pub fn tile_xy(&self, x: u32, y: u32) -> TileIndex {
        if x < self.size_x && y < self.size_y {
            TileIndex(y * self.size_x + x)
        } else {
            TileIndex::INVALID
        }
    }

    /// Get X coordinate from tile index
    pub fn tile_x(&self, index: TileIndex) -> u32 {
        index.0 % self.size_x
    }

    /// Get Y coordinate from tile index
    pub fn tile_y(&self, index: TileIndex) -> u32 {
        index.0 / self.size_x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_size() {
        // Ensure tile structures have correct sizes for save compatibility
        assert_eq!(std::mem::size_of::<TileBase>(), 8);
        assert_eq!(std::mem::size_of::<TileExtended>(), 4);
        assert_eq!(std::mem::size_of::<Tile>(), 12);
    }

    #[test]
    fn test_tile_type_packing() {
        let mut tile = Tile::new_void();

        // Test setting and getting tile type
        tile.base.set_tile_type(TileType::Railway);
        assert_eq!(tile.base.tile_type(), TileType::Railway);

        // Test bridge bits don't interfere
        tile.base.set_bridge_above(3);
        assert_eq!(tile.base.tile_type(), TileType::Railway);
        assert_eq!(tile.base.bridge_above(), 3);

        // Test climate zone bits don't interfere
        tile.base.set_climate_zone(TropicZone::Desert);
        assert_eq!(tile.base.tile_type(), TileType::Railway);
        assert_eq!(tile.base.bridge_above(), 3);
        assert_eq!(tile.base.climate_zone(), TropicZone::Desert);
    }

    #[test]
    fn test_map_creation() {
        // Test valid map sizes
        let map = Map::new(8, 8).unwrap(); // 256x256
        assert_eq!(map.size_x, 256);
        assert_eq!(map.size_y, 256);
        assert_eq!(map.size, 65536);
        assert_eq!(map.tiles.len(), 65536);

        // Test invalid sizes
        assert!(Map::new(5, 8).is_err()); // Too small
        assert!(Map::new(13, 8).is_err()); // Too large
    }

    #[test]
    fn test_tile_indexing() {
        let map = Map::new(8, 8).unwrap();

        // Test coordinate conversion
        let index = map.tile_xy(100, 50);
        assert_eq!(map.tile_x(index), 100);
        assert_eq!(map.tile_y(index), 50);

        // Test invalid coordinates
        let invalid = map.tile_xy(300, 50);
        assert_eq!(invalid, TileIndex::INVALID);
    }
}
