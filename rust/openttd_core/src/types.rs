//! Core type definitions matching OpenTTD C++ types for save compatibility.
//!
//! This module provides fundamental types used throughout the game.

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Company/Owner ID type (matches C++ Owner typedef and CompanyID)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Owner {
    /// Company IDs 0-14
    Company0 = 0,
    Company1 = 1,
    Company2 = 2,
    Company3 = 3,
    Company4 = 4,
    Company5 = 5,
    Company6 = 6,
    Company7 = 7,
    Company8 = 8,
    Company9 = 9,
    Company10 = 10,
    Company11 = 11,
    Company12 = 12,
    Company13 = 13,
    Company14 = 14,
    /// Town owner (0x0F)
    Town = 0x0F,
    /// No owner (0x10)
    None = 0x10,
    /// Water owner (0x11)
    Water = 0x11,
    /// Deity/GameScript owner (0x12)
    Deity = 0x12,
    /// Invalid owner (0xFF)
    Invalid = 0xFF,
}

impl Owner {
    /// Check if this is a valid company ID (0-14)
    pub fn is_company(&self) -> bool {
        (*self as u8) < 15
    }

    /// Get company ID if this is a company owner
    pub fn company_id(&self) -> Option<u8> {
        if self.is_company() {
            Some(*self as u8)
        } else {
            None
        }
    }

    /// Create Owner from company ID
    pub fn from_company_id(id: u8) -> Self {
        assert!(id <= 14, "Invalid company ID {}", id);
        // SAFETY: We've validated id is 0-14, which are valid Company values
        unsafe { std::mem::transmute(id) }
    }
}

impl Default for Owner {
    fn default() -> Self {
        Owner::None
    }
}

/// Company ID type alias
pub type CompanyID = Owner;

/// Station ID type (matches C++ StationID typedef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct StationID(pub u16);

impl StationID {
    pub const INVALID: StationID = StationID(0xFFFF);
    pub const MAX_STATIONS: usize = 64000;

    pub fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0 && self.0 < Self::MAX_STATIONS as u16
    }
}

impl Default for StationID {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Town ID type (matches C++ TownID typedef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct TownID(pub u16);

impl TownID {
    pub const INVALID: TownID = TownID(0xFFFF);
    pub const MAX_TOWNS: usize = 64000;

    pub fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0 && self.0 < Self::MAX_TOWNS as u16
    }
}

impl Default for TownID {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Industry ID type (matches C++ IndustryID typedef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct IndustryID(pub u16);

impl IndustryID {
    pub const INVALID: IndustryID = IndustryID(0xFFFF);
    pub const MAX_INDUSTRIES: usize = 64000;

    pub fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0 && self.0 < Self::MAX_INDUSTRIES as u16
    }
}

impl Default for IndustryID {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Vehicle ID type (matches C++ VehicleID typedef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct VehicleID(pub u32);

impl VehicleID {
    pub const INVALID: VehicleID = VehicleID(0xFFFFF);
    pub const NEW: VehicleID = VehicleID(0xFF000);
    pub const MAX_VEHICLES: usize = 0xFFFFF;

    pub fn is_valid(&self) -> bool {
        self.0 < Self::MAX_VEHICLES as u32
    }
}

impl Default for VehicleID {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Engine ID type (matches C++ EngineID typedef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct EngineID(pub u16);

impl EngineID {
    pub const INVALID: EngineID = EngineID(0xFFFF);
}

/// Unit number for vehicles (matches C++ UnitID)
pub type UnitID = u16;

/// Group ID type (matches C++ GroupID typedef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct GroupID(pub u16);

impl GroupID {
    pub const INVALID: GroupID = GroupID(0xFFFF);
    pub const DEFAULT: GroupID = GroupID(0xFFFE);
    pub const ALL: GroupID = GroupID(0xFFFD);
}

/// Cargo type ID (matches C++ CargoType typedef)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(transparent)]
pub struct CargoType(pub u8);

impl CargoType {
    pub const INVALID: CargoType = CargoType(0xFF);
    pub const NUM_CARGO: usize = 64;

    pub fn is_valid(&self) -> bool {
        self.0 < Self::NUM_CARGO as u8
    }

    /// Convert to u16 for comparisons
    pub fn as_u16(&self) -> u16 {
        self.0 as u16
    }

    /// Convert to usize for array indexing
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

impl Default for CargoType {
    fn default() -> Self {
        Self::INVALID
    }
}

/// Money type (matches C++ Money typedef int64_t)
pub type Money = i64;

/// String ID type for referencing game strings (matches C++ StringID uint16_t)
pub type StringID = u16;

/// Invalid string ID constant
pub const INVALID_STRING_ID: StringID = 0xFFFF;

/// Company mask for bitfield operations (matches C++ CompanyMask uint16_t)
pub type CompanyMask = u16;

/// Date types (matches C++ date system)
pub mod dates {
    use serde::{Deserialize, Serialize};

    /// Calendar date (days since year 0)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[repr(transparent)]
    pub struct CalendarDate(pub i32);

    /// Calendar year
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[repr(transparent)]
    pub struct CalendarYear(pub i32);

    /// Economy date (days since economy started)
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[repr(transparent)]
    pub struct EconomyDate(pub i32);

    /// Economy year
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
    #[repr(transparent)]
    pub struct EconomyYear(pub i32);

    pub const INVALID_DATE: i32 = -1;
}

// Re-export date types at module level for convenience
pub use dates::{CalendarDate, CalendarYear, EconomyDate, EconomyYear};

/// Tick counter type (matches C++ TimerGameTick::TickCounter)
pub type Tick = u64;

/// Destination ID for orders (matches C++ DestinationID)
pub type DestinationID = u16;

/// Owner ID is just an alias for Owner enum
pub type OwnerID = Owner;

/// Colours enum (matches C++ Colours)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Colours {
    DarkBlue = 0,
    PaleGreen = 1,
    Pink = 2,
    Yellow = 3,
    Red = 4,
    LightBlue = 5,
    Green = 6,
    DarkGreen = 7,
    Blue = 8,
    Cream = 9,
    Mauve = 10,
    Purple = 11,
    Orange = 12,
    Brown = 13,
    Grey = 14,
    White = 15,
    End = 16,
    Invalid = 0xFF,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owner_values() {
        // Verify specific owner values match C++
        assert_eq!(Owner::Town as u8, 0x0F);
        assert_eq!(Owner::None as u8, 0x10);
        assert_eq!(Owner::Water as u8, 0x11);
        assert_eq!(Owner::Deity as u8, 0x12);
        assert_eq!(Owner::Invalid as u8, 0xFF);

        // Test company detection
        assert!(Owner::Company0.is_company());
        assert!(Owner::Company14.is_company());
        assert!(!Owner::Town.is_company());
        assert!(!Owner::None.is_company());
    }

    #[test]
    fn test_id_types() {
        // Test invalid markers
        assert!(!StationID::INVALID.is_valid());
        assert!(!TownID::INVALID.is_valid());
        assert!(!IndustryID::INVALID.is_valid());
        assert!(!VehicleID::INVALID.is_valid());

        // Test valid IDs
        assert!(StationID(100).is_valid());
        assert!(TownID(500).is_valid());
        assert!(IndustryID(1000).is_valid());
        assert!(VehicleID(5000).is_valid());
    }
}
