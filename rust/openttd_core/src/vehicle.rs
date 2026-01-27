//! Vehicle data structures for OpenTTD
//!
//! This module contains the core vehicle structures that are saved in savegames.
//! All structures must maintain exact C++ compatibility for save/load.

use crate::map::TileIndex;
use crate::types::{
    CalendarDate, CalendarYear, CargoType, EconomyDate, EngineID, GroupID, Money, OwnerID,
    StationID, Tick, UnitID, VehicleID,
};
use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

// Vehicle type constants
pub const VEHICLE_LENGTH: u32 = 8;
pub const TILE_AXIAL_DISTANCE: u32 = 192;
pub const TILE_CORNER_DISTANCE: u32 = 128;
pub const GROUND_ACCELERATION: i32 = 9800;

/// Vehicle types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum VehicleType {
    Train = 0,
    Road = 1,
    Ship = 2,
    Aircraft = 3,
    Effect = 4,
    Disaster = 5,
    Invalid = 0xFF,
}

/// Direction enum matching C++
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum Direction {
    N = 0,
    NE = 1,
    E = 2,
    SE = 3,
    S = 4,
    SW = 5,
    W = 6,
    NW = 7,
    Invalid = 0xFF,
}

/// Vehicle states bitflags
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct VehicleStates: u8 {
        const HIDDEN = 1 << 0;
        const STOPPED = 1 << 1;
        const UNCLICKABLE = 1 << 2;
        const DEFAULT_PALETTE = 1 << 3;
        const TRAIN_SLOWING = 1 << 4;
        const SHADOW = 1 << 5;
        const AIRCRAFT_BROKEN = 1 << 6;
        const CRASHED = 1 << 7;
    }
}

/// Ground vehicle subtype flags
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum GroundVehicleSubtype {
    Front = 0,
    ArticulatedPart = 1,
    Wagon = 2,
    Engine = 3,
    FreeWagon = 4,
    Multiheaded = 5,
}

/// Visual effect spawn models
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum VisualEffectSpawnModel {
    None = 0,
    Steam = 1,
    Diesel = 2,
    Electric = 3,
}

/// Acceleration models
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum AccelerationModel {
    Original = 0,
    Realistic = 1,
}

/// Engine image type contexts
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum EngineImageType {
    OnMap = 0x00,
    InDepot = 0x10,
    InDetails = 0x11,
    InList = 0x12,
    Purchase = 0x20,
    Preview = 0x21,
}

/// Vehicle random triggers
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct VehicleRandomTriggers: u8 {
        const NEW_CARGO = 1 << 0;
        const DEPOT = 1 << 1;
        const EMPTY = 1 << 2;
        const ANY_NEW_CARGO = 1 << 3;
        const CALLBACK32 = 1 << 4;
    }
}

/// NewGRF cache for often-queried values
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewGRFCache {
    pub position_consist_length: u32,
    pub position_same_id_length: u32,
    pub consist_cargo_information: u32,
    pub company_information: u32,
    pub position_in_vehicle: u32,
    pub cache_valid: u8,
}

/// Vehicle cache for common values
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct VehicleCache {
    pub cached_max_speed: u16,
    pub cached_cargo_age_period: u16,
    pub cached_vis_effect: u8,
}

/// Sprite ID type
pub type SpriteID = u32;
pub type PaletteID = u32;
pub type TextEffectID = u16;

/// Palette sprite ID combination
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PalSpriteID {
    pub sprite: SpriteID,
    pub pal: PaletteID,
}

/// Vehicle sprite sequence
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VehicleSpriteSeq {
    pub seq: [PalSpriteID; 8],
    pub count: u32,
}

impl Default for VehicleSpriteSeq {
    fn default() -> Self {
        Self {
            seq: [PalSpriteID { sprite: 0, pal: 0 }; 8],
            count: 0,
        }
    }
}

/// Sprite bounds
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpriteBounds {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

/// Rectangle coordinates
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// Mutable sprite cache
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MutableSpriteCache {
    pub last_direction: Direction,
    pub revalidate_before_draw: bool,
    pub is_viewport_candidate: bool,
    pub old_coord: Rect,
    pub sprite_seq: VehicleSpriteSeq,
}

impl Default for MutableSpriteCache {
    fn default() -> Self {
        Self {
            last_direction: Direction::Invalid,
            revalidate_before_draw: false,
            is_viewport_candidate: false,
            old_coord: Rect::default(),
            sprite_seq: VehicleSpriteSeq::default(),
        }
    }
}

/// Order structure (simplified for now)
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Order {
    pub type_flags: u8,
    pub dest: u16,
    pub flags: u8,
    pub refit_cargo: u8,
    pub wait_time: u16,
    pub travel_time: u16,
    pub max_speed: u16,
}

/// Main vehicle structure
///
/// This represents the core vehicle data that is saved in savegames.
/// The structure must maintain exact compatibility with C++ for save/load.
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    // Pool item fields (would be inherited in C++)
    pub index: VehicleID,
    pub type_: VehicleType,

    // Chain pointers (in savegame these are indices)
    pub next: Option<VehicleID>,
    pub first: Option<VehicleID>,
    pub next_shared: Option<VehicleID>,

    // Position and movement
    pub tile: TileIndex,
    pub dest_tile: TileIndex,
    pub x_pos: i32,
    pub y_pos: i32,
    pub z_pos: i32,
    pub direction: Direction,

    // Financial
    pub profit_this_year: Money,
    pub profit_last_year: Money,
    pub value: Money,

    // Age and service
    pub build_year: CalendarYear,
    pub age: CalendarDate,
    pub economy_age: EconomyDate,
    pub max_age: CalendarDate,
    pub date_of_last_service: EconomyDate,
    pub date_of_last_service_newgrf: CalendarDate,

    // Reliability and breakdowns
    pub reliability: u16,
    pub reliability_spd_dec: u16,
    pub breakdown_ctr: u8,
    pub breakdown_delay: u8,
    pub breakdowns_since_last_service: u8,
    pub breakdown_chance: u8,

    // Owner and identification
    pub owner: OwnerID,
    pub spritenum: u8,
    pub bounds: SpriteBounds,
    pub engine_type: EngineID,
    pub unitnumber: UnitID,
    pub group_id: GroupID,

    // Movement and physics
    pub cur_speed: u16,
    pub subspeed: u8,
    pub acceleration: u8,
    pub motion_counter: u32,
    pub progress: u8,

    // Cargo
    pub cargo_type: CargoType,
    pub cargo_subtype: u8,
    pub cargo_cap: u16,
    pub refit_cap: u16,
    pub cargo_age_counter: u16,

    // Stations
    pub last_station_visited: StationID,
    pub last_loading_station: StationID,
    pub last_loading_tick: Tick,

    // Randomization
    pub waiting_random_triggers: VehicleRandomTriggers,
    pub random_bits: u16,

    // Status and orders
    pub vehstatus: VehicleStates,
    pub subtype: u8,
    pub current_order: Order,

    // Counters
    pub day_counter: u8,
    pub tick_counter: u8,
    pub running_ticks: u8,
    pub load_unload_ticks: u16,

    // Caches
    pub grf_cache: NewGRFCache,
    pub vcache: VehicleCache,

    // Sprite cache
    pub sprite_cache: MutableSpriteCache,

    // Text effect
    pub fill_percent_te_id: TextEffectID,
}

impl Vehicle {
    /// Create a new vehicle with default values
    pub fn new(index: VehicleID, type_: VehicleType) -> Self {
        Self {
            index,
            type_,
            next: None,
            first: None,
            next_shared: None,
            tile: TileIndex::INVALID,
            dest_tile: TileIndex::INVALID,
            x_pos: 0,
            y_pos: 0,
            z_pos: 0,
            direction: Direction::Invalid,
            profit_this_year: 0,
            profit_last_year: 0,
            value: 0,
            build_year: CalendarYear(0),
            age: CalendarDate(0),
            economy_age: EconomyDate(0),
            max_age: CalendarDate(0),
            date_of_last_service: EconomyDate(0),
            date_of_last_service_newgrf: CalendarDate(0),
            reliability: 0,
            reliability_spd_dec: 0,
            breakdown_ctr: 0,
            breakdown_delay: 0,
            breakdowns_since_last_service: 0,
            breakdown_chance: 0,
            owner: OwnerID::Invalid,
            spritenum: 0,
            bounds: SpriteBounds::default(),
            engine_type: EngineID::INVALID,
            unitnumber: 0,
            group_id: GroupID::INVALID,
            cur_speed: 0,
            subspeed: 0,
            acceleration: 0,
            motion_counter: 0,
            progress: 0,
            cargo_type: CargoType::INVALID,
            cargo_subtype: 0,
            cargo_cap: 0,
            refit_cap: 0,
            cargo_age_counter: 0,
            last_station_visited: StationID::INVALID,
            last_loading_station: StationID::INVALID,
            last_loading_tick: 0,
            waiting_random_triggers: VehicleRandomTriggers::empty(),
            random_bits: 0,
            vehstatus: VehicleStates::empty(),
            subtype: 0,
            current_order: Order::default(),
            day_counter: 0,
            tick_counter: 0,
            running_ticks: 0,
            load_unload_ticks: 0,
            grf_cache: NewGRFCache::default(),
            vcache: VehicleCache::default(),
            sprite_cache: MutableSpriteCache::default(),
            fill_percent_te_id: 0xFFFF, // INVALID_TE_ID
        }
    }

    /// Check if vehicle is a front engine
    pub fn is_front(&self) -> bool {
        self.subtype == GroundVehicleSubtype::Front as u8
    }

    /// Check if vehicle is crashed
    pub fn is_crashed(&self) -> bool {
        self.vehstatus.contains(VehicleStates::CRASHED)
    }

    /// Check if vehicle is stopped
    pub fn is_stopped(&self) -> bool {
        self.vehstatus.contains(VehicleStates::STOPPED)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_vehicle_size() {
        // This test will need adjustment based on actual C++ struct size
        // For now we're just checking that our struct has a reasonable size
        let size = mem::size_of::<Vehicle>();
        println!("Vehicle struct size: {} bytes", size);
        // The actual C++ vehicle is quite large, likely 300+ bytes
        assert!(size < 1024, "Vehicle struct unexpectedly large");
    }

    #[test]
    fn test_vehicle_type_values() {
        assert_eq!(VehicleType::Train as u8, 0);
        assert_eq!(VehicleType::Road as u8, 1);
        assert_eq!(VehicleType::Ship as u8, 2);
        assert_eq!(VehicleType::Aircraft as u8, 3);
        assert_eq!(VehicleType::Invalid as u8, 0xFF);
    }

    #[test]
    fn test_direction_values() {
        assert_eq!(Direction::N as u8, 0);
        assert_eq!(Direction::E as u8, 2);
        assert_eq!(Direction::S as u8, 4);
        assert_eq!(Direction::W as u8, 6);
        assert_eq!(Direction::Invalid as u8, 0xFF);
    }

    #[test]
    fn test_vehicle_states() {
        let mut states = VehicleStates::empty();
        assert!(!states.contains(VehicleStates::CRASHED));

        states.insert(VehicleStates::CRASHED);
        assert!(states.contains(VehicleStates::CRASHED));

        states.insert(VehicleStates::STOPPED);
        assert!(states.contains(VehicleStates::STOPPED));
    }

    #[test]
    fn test_vehicle_creation() {
        let vehicle = Vehicle::new(VehicleID(42), VehicleType::Train);
        assert_eq!(vehicle.index, VehicleID(42));
        assert_eq!(vehicle.type_, VehicleType::Train);
        assert_eq!(vehicle.tile, TileIndex::INVALID);
        assert!(!vehicle.is_crashed());
        assert!(!vehicle.is_stopped());
    }
}
