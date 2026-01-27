//! Station data structures for OpenTTD
//!
//! This module contains station structures that are saved in savegames.
//! All structures must maintain exact C++ compatibility for save/load.

use crate::map::TileIndex;
use crate::types::{
    CalendarDate, CargoType, EconomyDate, IndustryID, Owner, StationID, StringID, TownID,
    VehicleID, INVALID_STRING_ID,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;

/// Station facility types (matches C++ StationFacility enum)
pub type StationFacility = u8;

pub const FACIL_NONE: StationFacility = 0;
pub const FACIL_TRAIN: StationFacility = 1 << 0;
pub const FACIL_TRUCK_STOP: StationFacility = 1 << 1;
pub const FACIL_BUS_STOP: StationFacility = 1 << 2;
pub const FACIL_AIRPORT: StationFacility = 1 << 3;
pub const FACIL_DOCK: StationFacility = 1 << 4;
pub const FACIL_WAYPOINT: StationFacility = 1 << 7;

/// Station types (matches C++ StationType enum)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StationType {
    RailStation = 0,
    AirportStation = 1,
    TruckStation = 2,
    BusStation = 3,
    OilRig = 4,
    DockStation = 5,
    BuoyStation = 6,
    WaypointStation = 7,
}

/// Catchment area for different facilities
pub const CA_NONE: u8 = 0;
pub const CA_BUS: u8 = 3;
pub const CA_TRUCK: u8 = 3;
pub const CA_TRAIN: u8 = 4;
pub const CA_DOCK: u8 = 5;

/// Airport types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AirportType {
    Small = 0,
    Large = 1,
    Heliport = 2,
    Metropolitan = 3,
    International = 4,
    Commuter = 5,
    Helidepot = 6,
    Intercontinental = 7,
    Helistation = 8,
    Oilrig = 9,
    Invalid = 255,
}

/// Maximum number of cargo types
pub const NUM_CARGO: usize = 64;

/// Station cargo waiting information
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StationCargoWaiting {
    pub cargo: CargoType,
    pub amount: u16,           // Amount waiting
    pub source: StationID,     // Source station for cargo routing
    pub days_in_transit: u16,  // Days cargo has been waiting
    pub rating: u8,            // Station rating for this cargo (0-255)
    pub last_speed: u8,        // Speed of last vehicle to load
    pub last_age: u8,          // Age of last vehicle to load
    pub time_since_pickup: u8, // Time since cargo was last picked up
}

/// Good entry in station's goods list
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct GoodsEntry {
    pub acceptance: bool,       // Station accepts this cargo
    pub rating: u8,             // Station rating (0-255)
    pub last_speed: u8,         // Speed of last vehicle
    pub last_age: u8,           // Age of last vehicle
    pub amount_waiting: u16,    // Cargo amount waiting
    pub time_since_pickup: u8,  // Time since last pickup
    pub days_in_transit: u16,   // Average days in transit
    pub max_waiting_cargo: u16, // Maximum cargo ever waiting
    pub from: StationID,        // Source station for cargo
    pub via: StationID,         // Next hop station
}

/// Station specification for custom graphics
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StationSpec {
    pub cls_id: u32,    // Station class ID
    pub spec_index: u8, // Index within class
    pub grf_id: u32,    // NewGRF ID
}

/// Station rectangle for coverage area
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct StationRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl StationRect {
    pub fn is_empty(&self) -> bool {
        self.left > self.right || self.top > self.bottom
    }

    pub fn width(&self) -> u32 {
        if self.is_empty() {
            0
        } else {
            (self.right - self.left + 1) as u32
        }
    }

    pub fn height(&self) -> u32 {
        if self.is_empty() {
            0
        } else {
            (self.bottom - self.top + 1) as u32
        }
    }
}

/// Station structure (matches C++ BaseStation/Station for savegame compatibility)
#[repr(C)]
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Station {
    /// Station index/ID
    pub index: StationID,

    /// Station location (one of the tiles)
    pub xy: TileIndex,

    /// Station rectangle
    pub rect: StationRect,

    /// Station name
    pub name: StringID,

    /// Custom name string (if renamed)
    pub string_id: StringID,

    /// Town this station belongs to
    pub town: TownID,

    /// Company owning the station
    pub owner: Owner,

    /// Facilities available at station
    pub facilities: StationFacility,

    /// Airport type (if applicable)
    pub airport_type: AirportType,

    /// Airport flags/state
    pub airport_flags: u64,

    /// Rotation of airport
    pub airport_rotation: u8,

    /// Dock tile location
    pub dock_tile: TileIndex,

    /// Train station length/width
    pub train_station: StationRect,

    /// Date station was built
    pub build_date: CalendarDate,

    /// Bus/truck stop status (bit per stop)
    pub bus_stop_status: u8,
    pub truck_stop_status: u8,

    /// Had vehicle of type (bit per vehicle type)
    pub had_vehicle_of_type: u8,

    /// Cargo waiting at station
    #[serde_as(as = "[_; 64]")]
    pub goods: [GoodsEntry; NUM_CARGO],

    /// Cargo acceptance in catchment area
    #[serde_as(as = "[_; 64]")]
    pub acceptance: [u32; NUM_CARGO],

    /// Time since station was used
    pub time_since_load: u8,
    pub time_since_unload: u8,

    /// Station signs
    pub sign: StationRect,

    /// Waiting cargo by destination
    pub cargo_waiting: Vec<StationCargoWaiting>,

    /// Industries in catchment area
    pub industries_near: Vec<IndustryID>,

    /// Station specification for custom graphics
    pub spec: StationSpec,

    /// Date of last cargo pickup by type
    #[serde_as(as = "[_; 64]")]
    pub last_pickup_date: [EconomyDate; NUM_CARGO],
}

impl Station {
    /// Create a new station
    pub fn new(index: StationID, location: TileIndex, owner: Owner) -> Self {
        Self {
            index,
            xy: location,
            rect: StationRect::default(),
            name: INVALID_STRING_ID,
            string_id: INVALID_STRING_ID,
            town: TownID::INVALID,
            owner,
            facilities: FACIL_NONE,
            airport_type: AirportType::Invalid,
            airport_flags: 0,
            airport_rotation: 0,
            dock_tile: TileIndex::INVALID,
            train_station: StationRect::default(),
            build_date: CalendarDate(0),
            bus_stop_status: 0,
            truck_stop_status: 0,
            had_vehicle_of_type: 0,
            goods: [GoodsEntry::default(); NUM_CARGO],
            acceptance: [0; NUM_CARGO],
            time_since_load: 255,
            time_since_unload: 255,
            sign: StationRect::default(),
            cargo_waiting: Vec::new(),
            industries_near: Vec::new(),
            spec: StationSpec::default(),
            last_pickup_date: [EconomyDate(0); NUM_CARGO],
        }
    }

    /// Check if station has specific facilities
    pub fn has_facilities(&self, facilities: StationFacility) -> bool {
        self.facilities & facilities != 0
    }

    /// Check if station is a waypoint
    pub fn is_waypoint(&self) -> bool {
        self.facilities == FACIL_WAYPOINT
    }

    /// Check if station has an airport
    pub fn has_airport(&self) -> bool {
        self.has_facilities(FACIL_AIRPORT)
    }

    /// Check if station has a dock
    pub fn has_dock(&self) -> bool {
        self.has_facilities(FACIL_DOCK)
    }

    /// Check if station accepts cargo type
    pub fn accepts_cargo(&self, cargo: CargoType) -> bool {
        cargo.as_u16() < NUM_CARGO as u16 && self.goods[cargo.as_usize()].acceptance
    }

    /// Get amount of cargo waiting
    pub fn get_waiting_cargo(&self, cargo: CargoType) -> u16 {
        if cargo.as_u16() < NUM_CARGO as u16 {
            self.goods[cargo.as_usize()].amount_waiting
        } else {
            0
        }
    }

    /// Get station rating for cargo
    pub fn get_rating(&self, cargo: CargoType) -> u8 {
        if cargo.as_u16() < NUM_CARGO as u16 {
            self.goods[cargo.as_usize()].rating
        } else {
            0
        }
    }

    /// Get catchment radius
    pub fn get_catchment_radius(&self) -> u8 {
        let mut radius = CA_NONE;

        if self.has_facilities(FACIL_TRAIN) {
            radius = radius.max(CA_TRAIN);
        }
        if self.has_facilities(FACIL_DOCK) {
            radius = radius.max(CA_DOCK);
        }
        if self.has_facilities(FACIL_BUS_STOP) {
            radius = radius.max(CA_BUS);
        }
        if self.has_facilities(FACIL_TRUCK_STOP) {
            radius = radius.max(CA_TRUCK);
        }
        if self.has_airport() {
            // Airport catchment depends on type
            radius = radius.max(match self.airport_type {
                AirportType::Small | AirportType::Heliport => 4,
                AirportType::Large | AirportType::Metropolitan => 6,
                AirportType::International => 8,
                AirportType::Intercontinental => 10,
                _ => 4,
            });
        }

        radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_creation() {
        let station = Station::new(StationID(1), TileIndex(1000), Owner::Company0);
        assert_eq!(station.index, StationID(1));
        assert_eq!(station.xy, TileIndex(1000));
        assert_eq!(station.owner, Owner::Company0);
        assert_eq!(station.facilities, FACIL_NONE);
    }

    #[test]
    fn test_station_facilities() {
        let mut station = Station::new(StationID(1), TileIndex(1000), Owner::Company0);

        station.facilities = FACIL_TRAIN | FACIL_BUS_STOP;
        assert!(station.has_facilities(FACIL_TRAIN));
        assert!(station.has_facilities(FACIL_BUS_STOP));
        assert!(!station.has_facilities(FACIL_AIRPORT));
        assert!(!station.is_waypoint());

        station.facilities = FACIL_WAYPOINT;
        assert!(station.is_waypoint());
    }

    #[test]
    fn test_station_rect() {
        let rect = StationRect {
            left: 10,
            top: 20,
            right: 30,
            bottom: 40,
        };

        assert!(!rect.is_empty());
        assert_eq!(rect.width(), 21);
        assert_eq!(rect.height(), 21);
    }
}
