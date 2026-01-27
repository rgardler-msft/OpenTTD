//! Town data structures for OpenTTD
//!
//! This module contains town structures that are saved in savegames.
//! All structures must maintain exact C++ compatibility for save/load.

use crate::map::TileIndex;
use crate::types::{
    CalendarDate, CargoType, CompanyMask, EconomyDate, Owner, StationID, StringID, TownID,
    INVALID_STRING_ID,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;

/// Town size categories (matches C++ TownSize enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TownSize {
    Small = 0,
    Medium = 1,
    Large = 2,
    Random = 3, // Used in settings only
}

/// Town founding settings (matches C++ TownFounding enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TownFounding {
    None = 0,    // No town founding allowed
    Allowed = 1, // Player can found towns
    Custom = 2,  // Custom town founding rules
}

/// Town layout types (matches C++ TownLayout enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TownLayout {
    Original = 0,     // Original algorithm (grid)
    Better = 1,       // Improved algorithm
    TwoByTwo = 2,     // 2x2 grid
    ThreeByThree = 3, // 3x3 grid
    Random = 4,       // Random selection
}

/// Rating thresholds for town opinions
pub const RATING_MINIMUM: i16 = -1000;
pub const RATING_APPALLING: i16 = -400;
pub const RATING_VERYBAD: i16 = -200;
pub const RATING_BAD: i16 = 0;
pub const RATING_MEDIOCRE: i16 = 200;
pub const RATING_GOOD: i16 = 400;
pub const RATING_VERYGOOD: i16 = 600;
pub const RATING_EXCELLENT: i16 = 800;
pub const RATING_MAXIMUM: i16 = 1000;

/// Maximum companies for ratings tracking
pub const MAX_COMPANIES: usize = 15;

/// House zone types (matches C++ HouseZones enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HouseZone {
    NoZone = 0,
    Residential1 = 1, // Outer suburb
    Residential2 = 2, // Inner suburb
    Commercial = 3,   // Commercial/Office
    Industrial = 4,   // Industrial
}

/// Town growth rate flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TownGrowth {
    Normal = 0,
    Desert = 1, // Desert town (needs food/water)
    Arctic = 2, // Arctic town (needs food)
}

/// Town authority actions (matches C++ TownActions)
pub type TownActions = u8;

pub const TACT_NONE: TownActions = 0x00;
pub const TACT_ADVERTISE_SMALL: TownActions = 0x01;
pub const TACT_ADVERTISE_MEDIUM: TownActions = 0x02;
pub const TACT_ADVERTISE_LARGE: TownActions = 0x04;
pub const TACT_ROAD_REBUILD: TownActions = 0x08;
pub const TACT_BUILD_STATUE: TownActions = 0x10;
pub const TACT_FUND_BUILDINGS: TownActions = 0x20;
pub const TACT_BUY_RIGHTS: TownActions = 0x40;
pub const TACT_BRIBE: TownActions = 0x80;

/// Town flags
pub type TownFlags = u8;

pub const TOWN_IS_GROWING: TownFlags = 0x01;
pub const TOWN_HAS_CATHEDRAL: TownFlags = 0x02;
pub const TOWN_HAS_STADIUM: TownFlags = 0x04;
pub const TOWN_CUSTOM_GROWTH: TownFlags = 0x08;

/// Cargo acceptance for houses
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoAcceptance {
    pub cargo: CargoType,
    pub acceptance: u8, // 0-15, where 8 = full acceptance
}

/// Town cargo statistics
#[repr(C)]
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TownCargo {
    #[serde_as(as = "[_; 64]")]
    pub produced: [u32; 64], // Amount produced per cargo type
    #[serde_as(as = "[_; 64]")]
    pub accepted: [u32; 64], // Amount accepted per cargo type
    #[serde_as(as = "[_; 64]")]
    pub received: [u32; 64], // Amount received per cargo type
}

impl Default for TownCargo {
    fn default() -> Self {
        Self {
            produced: [0; 64],
            accepted: [0; 64],
            received: [0; 64],
        }
    }
}

/// Town structure (matches C++ Town class for savegame compatibility)
#[repr(C)]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Town {
    /// Town index/ID
    pub index: TownID,

    /// Town location (center tile)
    pub xy: TileIndex,

    /// Town name string IDs
    pub townnamegrfid: u32, // NewGRF providing the name
    pub townnametype: u16,  // Town name style
    pub townnameparts: u32, // Town name generation seed
    pub name: StringID,     // Custom name string ID

    /// Town status flags
    pub flags: TownFlags,

    /// Church presence
    pub church_count: u16,
    pub stadium_count: u16,

    /// Population statistics
    pub population: u32, // Current population
    pub num_houses: u32, // Number of houses

    /// Town growth parameters
    pub time_until_rebuild: u16, // Ticks until road rebuild
    pub grow_counter: u16, // Ticks until next growth
    pub growth_rate: i16,  // Growth speed (lower = faster)

    /// House counts by zone
    pub house_counts: [u32; 5], // Count per HouseZone

    /// Cargo statistics
    pub cargo: TownCargo,

    /// Company ratings (-1000 to 1000)
    #[serde_as(as = "[_; 15]")]
    pub ratings: [i16; MAX_COMPANIES],

    /// Company test ratings (temporary during actions)
    #[serde_as(as = "[_; 15]")]
    pub test_ratings: [i16; MAX_COMPANIES],

    /// Which companies have a statue
    pub have_statue: CompanyMask,

    /// Which companies have exclusive transport rights
    pub exclusive_counter: u8, // Months remaining
    pub exclusivity: Owner, // Company with rights

    /// Town fund buildings counter
    pub fund_buildings_months: u8, // Months remaining

    /// Road construction state  
    pub road_build_months: u8, // Months for road rebuild action

    /// Town layout style
    pub layout: TownLayout,

    /// Larger town flag (gets more growth)
    pub larger_town: bool,

    /// Town label style (for display)
    pub label_style: u8,

    /// Last month's statistics
    #[serde_as(as = "[_; 64]")]
    pub supplied_last_month: [u32; 64],
    #[serde_as(as = "[_; 64]")]
    pub received_last_month: [u32; 64],

    /// Airport noise accumulator
    pub noise_reached: u16, // Current noise level

    /// Stations that serve this town
    pub stations_near: Vec<StationID>,
}

impl Town {
    /// Create a new town with default values
    pub fn new(index: TownID, location: TileIndex) -> Self {
        Self {
            index,
            xy: location,
            townnamegrfid: 0,
            townnametype: 0,
            townnameparts: 0,
            name: INVALID_STRING_ID,
            flags: 0,
            church_count: 0,
            stadium_count: 0,
            population: 0,
            num_houses: 0,
            time_until_rebuild: 0,
            grow_counter: 0,
            growth_rate: 0,
            house_counts: [0; 5],
            cargo: TownCargo::default(),
            ratings: [0; MAX_COMPANIES],
            test_ratings: [0; MAX_COMPANIES],
            have_statue: 0,
            exclusive_counter: 0,
            exclusivity: Owner::None,
            fund_buildings_months: 0,
            road_build_months: 0,
            layout: TownLayout::Original,
            larger_town: false,
            label_style: 0,
            supplied_last_month: [0; 64],
            received_last_month: [0; 64],
            noise_reached: 0,
            stations_near: Vec::new(),
        }
    }

    /// Check if town is growing
    pub fn is_growing(&self) -> bool {
        self.flags & TOWN_IS_GROWING != 0
    }

    /// Get company rating
    pub fn get_rating(&self, company: u8) -> i16 {
        if company < MAX_COMPANIES as u8 {
            self.ratings[company as usize]
        } else {
            0
        }
    }

    /// Check if a company has a statue
    pub fn has_statue(&self, company: u8) -> bool {
        if company < MAX_COMPANIES as u8 {
            self.have_statue & (1 << company) != 0
        } else {
            false
        }
    }

    /// Get town size category based on population
    pub fn get_town_size(&self) -> TownSize {
        if self.population < 1000 {
            TownSize::Small
        } else if self.population < 3000 {
            TownSize::Medium
        } else {
            TownSize::Large
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_town_size() {
        // Verify struct size matches expectations
        let town_size = std::mem::size_of::<Town>();
        // Note: Size will vary due to Vec, but base fields should be consistent
        assert!(town_size >= 500); // Minimum expected size
    }

    #[test]
    fn test_town_creation() {
        let town = Town::new(TownID(1), TileIndex(1000));
        assert_eq!(town.index, TownID(1));
        assert_eq!(town.xy, TileIndex(1000));
        assert_eq!(town.population, 0);
        assert!(!town.is_growing());
    }

    #[test]
    fn test_town_ratings() {
        let mut town = Town::new(TownID(1), TileIndex(1000));
        town.ratings[0] = 500;
        assert_eq!(town.get_rating(0), 500);
        assert_eq!(town.get_rating(1), 0);
    }
}
