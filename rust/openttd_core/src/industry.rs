//! Industry data structures for OpenTTD
//!
//! This module contains industry structures that are saved in savegames.
//! All structures must maintain exact C++ compatibility for save/load.

use crate::map::TileIndex;
use crate::types::{CalendarDate, CargoType, EconomyDate, IndustryID, Owner, StationID, TownID};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;

/// Industry types - basic industry categories
pub const INDUSTRYTYPE_COAL_MINE: u16 = 0;
pub const INDUSTRYTYPE_POWER_STATION: u16 = 1;
pub const INDUSTRYTYPE_SAWMILL: u16 = 2;
pub const INDUSTRYTYPE_FOREST: u16 = 3;
pub const INDUSTRYTYPE_OIL_REFINERY: u16 = 4;
pub const INDUSTRYTYPE_OIL_RIG: u16 = 5;
pub const INDUSTRYTYPE_FACTORY: u16 = 6;
pub const INDUSTRYTYPE_PRINTING_WORKS: u16 = 7;
pub const INDUSTRYTYPE_STEEL_MILL: u16 = 8;
pub const INDUSTRYTYPE_FARM: u16 = 9;
pub const INDUSTRYTYPE_COPPER_ORE_MINE: u16 = 10;
pub const INDUSTRYTYPE_OIL_WELLS: u16 = 11;
pub const INDUSTRYTYPE_BANK: u16 = 12;
pub const INDUSTRYTYPE_FOOD_PROCESSING: u16 = 13;
pub const INDUSTRYTYPE_PAPER_MILL: u16 = 14;
pub const INDUSTRYTYPE_GOLD_MINE: u16 = 15;
pub const INDUSTRYTYPE_BANK_TROPIC: u16 = 16;
pub const INDUSTRYTYPE_DIAMOND_MINE: u16 = 17;
pub const INDUSTRYTYPE_IRON_ORE_MINE: u16 = 18;
pub const INDUSTRYTYPE_WATER_SUPPLY: u16 = 19;
pub const INDUSTRYTYPE_WATER_TOWER: u16 = 20;
pub const INDUSTRYTYPE_INVALID: u16 = 0xFFFF;

/// Industry type ID
pub type IndustryType = u16;

/// Industry behavior flags (matches C++ IndustryBehaviour enum)
pub type IndustryBehaviour = u32;

pub const INDUSTRYBEH_NONE: IndustryBehaviour = 0;
pub const INDUSTRYBEH_PLANT_FIELDS: IndustryBehaviour = 1 << 0;
pub const INDUSTRYBEH_CUT_TREES: IndustryBehaviour = 1 << 1;
pub const INDUSTRYBEH_BUILT_ONWATER: IndustryBehaviour = 1 << 2;
pub const INDUSTRYBEH_TOWN_REQUIRED: IndustryBehaviour = 1 << 3;
pub const INDUSTRYBEH_ONLY_NEARTOWN: IndustryBehaviour = 1 << 4;
pub const INDUSTRYBEH_ONLY_INTOWN: IndustryBehaviour = 1 << 5;
pub const INDUSTRYBEH_NO_PRODUCTION: IndustryBehaviour = 1 << 6;
pub const INDUSTRYBEH_BEFORE_1950: IndustryBehaviour = 1 << 7;
pub const INDUSTRYBEH_AFTER_1960: IndustryBehaviour = 1 << 8;
pub const INDUSTRYBEH_HELICOPTER_STATION: IndustryBehaviour = 1 << 9;
pub const INDUSTRYBEH_CAN_SUBSIDENCE: IndustryBehaviour = 1 << 10;

/// Industry control flags (matches C++ IndustryControlFlags enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum IndustryControlFlags {
    None = 0,
    NoProductionIncrease = 1 << 0,
    NoProductionDecrease = 1 << 1,
    NoClosing = 1 << 2,
}

/// Production callback version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ProductionCallbackVersion {
    Original = 0,
    Version1 = 1,
    Version2 = 2,
}

/// Maximum cargo inputs/outputs for an industry
pub const INDUSTRY_NUM_INPUTS: usize = 16;
pub const INDUSTRY_NUM_OUTPUTS: usize = 16;

/// Industry cargo slot for input/output
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndustryCargo {
    pub cargo: CargoType,
    pub waiting: u16,        // Amount waiting (for inputs)
    pub production_rate: u8, // Production rate (for outputs)
    pub last_accepted: u32,  // Last accepted amount (for inputs)
}

/// Industry production statistics
#[repr(C)]
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndustryProduction {
    #[serde_as(as = "[_; 16]")]
    pub produced: [u16; INDUSTRY_NUM_OUTPUTS], // Production amount per output
    #[serde_as(as = "[_; 16]")]
    pub transported: [u16; INDUSTRY_NUM_OUTPUTS], // Amount transported
    pub history: [[u8; 12]; INDUSTRY_NUM_OUTPUTS], // Monthly production history
}

impl Default for IndustryProduction {
    fn default() -> Self {
        Self {
            produced: [0; INDUSTRY_NUM_OUTPUTS],
            transported: [0; INDUSTRY_NUM_OUTPUTS],
            history: [[0; 12]; INDUSTRY_NUM_OUTPUTS],
        }
    }
}

/// Industry structure (matches C++ Industry class for savegame compatibility)
#[repr(C)]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Industry {
    /// Industry index/ID
    pub index: IndustryID,

    /// Location (top-left tile)
    pub location: TileIndex,

    /// Width and height in tiles
    pub width: u8,
    pub height: u8,

    /// Industry type
    pub industry_type: IndustryType,

    /// Associated town
    pub town: TownID,

    /// Owner (for certain industry types)
    pub owner: Owner,

    /// Production multiplier
    pub prod_level: u8,

    /// Random value for industry decisions
    pub random: u16,

    /// Cargo inputs
    #[serde_as(as = "[_; 16]")]
    pub accepts_cargo: [IndustryCargo; INDUSTRY_NUM_INPUTS],

    /// Cargo outputs  
    #[serde_as(as = "[_; 16]")]
    pub produced_cargo: [IndustryCargo; INDUSTRY_NUM_OUTPUTS],

    /// Production statistics
    pub production: IndustryProduction,

    /// Last month's production statistics
    pub last_month_production: IndustryProduction,

    /// Counter for production changes
    pub counter: u16,

    /// Industry type at last station rating update
    pub type_at_last_rating: IndustryType,

    /// Date industry was built
    pub construction_date: CalendarDate,

    /// Random color for minimap
    pub random_colour: u8,

    /// Last year this industry was serviced
    pub last_serviced_year: EconomyDate,

    /// Did this industry get any cargo delivered last month
    pub was_cargo_delivered: bool,

    /// Callback flags
    pub callback_mask: u32,

    /// Control flags
    pub control_flags: IndustryControlFlags,

    /// Text ID for production up/down messages
    pub last_text_message: u16,

    /// When industry will be built (for prospecting)
    pub construction_type: u8,

    /// Selected layout during construction
    pub selected_layout: u8,

    /// Exclusive supplier/consumer agreements
    pub exclusive_supplier: Owner,
    pub exclusive_consumer: Owner,

    /// Stations that serve this industry
    pub stations_near: Vec<StationID>,
}

impl Industry {
    /// Create a new industry
    pub fn new(index: IndustryID, location: TileIndex, industry_type: IndustryType) -> Self {
        Self {
            index,
            location,
            width: 0,
            height: 0,
            industry_type,
            town: TownID::INVALID,
            owner: Owner::None,
            prod_level: 0,
            random: 0,
            accepts_cargo: [IndustryCargo::default(); INDUSTRY_NUM_INPUTS],
            produced_cargo: [IndustryCargo::default(); INDUSTRY_NUM_OUTPUTS],
            production: IndustryProduction::default(),
            last_month_production: IndustryProduction::default(),
            counter: 0,
            type_at_last_rating: industry_type,
            construction_date: CalendarDate(0),
            random_colour: 0,
            last_serviced_year: EconomyDate(0),
            was_cargo_delivered: false,
            callback_mask: 0,
            control_flags: IndustryControlFlags::None,
            last_text_message: 0,
            construction_type: 0,
            selected_layout: 0,
            exclusive_supplier: Owner::None,
            exclusive_consumer: Owner::None,
            stations_near: Vec::new(),
        }
    }

    /// Check if industry accepts a cargo type
    pub fn accepts(&self, cargo: CargoType) -> bool {
        self.accepts_cargo.iter().any(|c| c.cargo == cargo)
    }

    /// Check if industry produces a cargo type
    pub fn produces(&self, cargo: CargoType) -> bool {
        self.produced_cargo.iter().any(|c| c.cargo == cargo)
    }

    /// Get total production
    pub fn get_total_production(&self) -> u32 {
        self.production.produced.iter().map(|&p| p as u32).sum()
    }

    /// Get total transported
    pub fn get_total_transported(&self) -> u32 {
        self.production.transported.iter().map(|&t| t as u32).sum()
    }

    /// Get transport percentage (0-100)
    pub fn get_transport_percentage(&self) -> u8 {
        let production = self.get_total_production();
        if production == 0 {
            return 0;
        }
        let transported = self.get_total_transported();
        std::cmp::min(100, (transported * 100 / production) as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_industry_size() {
        let industry_size = std::mem::size_of::<Industry>();
        // Size will vary due to Vec, but check minimum
        assert!(industry_size >= 400); // Minimum expected size
    }

    #[test]
    fn test_industry_creation() {
        let industry = Industry::new(IndustryID(1), TileIndex(1000), INDUSTRYTYPE_COAL_MINE);
        assert_eq!(industry.index, IndustryID(1));
        assert_eq!(industry.location, TileIndex(1000));
        assert_eq!(industry.industry_type, INDUSTRYTYPE_COAL_MINE);
    }

    #[test]
    fn test_cargo_operations() {
        let mut industry = Industry::new(IndustryID(1), TileIndex(1000), INDUSTRYTYPE_COAL_MINE);

        // Set up coal production
        industry.produced_cargo[0].cargo = CargoType(1); // Assuming coal = 1
        industry.accepts_cargo[0].cargo = CargoType(2); // Assuming passengers = 2

        assert!(industry.produces(CargoType(1)));
        assert!(!industry.produces(CargoType(2)));
        assert!(industry.accepts(CargoType(2)));
        assert!(!industry.accepts(CargoType(1)));
    }
}
