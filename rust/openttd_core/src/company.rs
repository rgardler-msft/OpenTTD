//! Company data structures for OpenTTD
//!
//! This module contains company structures that are saved in savegames.
//! All structures must maintain exact C++ compatibility for save/load.

use crate::map::TileIndex;
use crate::types::{
    CalendarYear, Colours, CompanyMask, EconomyYear, Money, Owner, StringID, INVALID_STRING_ID,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::serde_as;

/// Maximum loan default value
pub const COMPANY_MAX_LOAN_DEFAULT: Money = i64::MIN;

/// Maximum history quarters for economic data
pub const MAX_HISTORY_QUARTERS: usize = 24;

/// Livery scheme types
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum LiveryScheme {
    Default = 0,
    SteamEngine = 1,
    DieselEngine = 2,
    ElectricEngine = 3,
    MonorailEngine = 4,
    MaglevEngine = 5,
    DMU = 6,
    EMU = 7,
    PassengerWagon = 8,
    FreightWagon = 9,
    Bus = 10,
    Truck = 11,
    PassengerShip = 12,
    FreightShip = 13,
    Helicopter = 14,
    SmallPlane = 15,
    LargePlane = 16,
    PassengerTram = 17,
    FreightTram = 18,
    RoadVehicle = 19,
    End = 20,
}

pub const LS_END: usize = LiveryScheme::End as usize;

/// Livery settings for a vehicle type
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Livery {
    pub in_use: u8,  // 0 = not used, 1 = used
    pub colour1: u8, // First colour
    pub colour2: u8, // Second colour
}

impl Default for Livery {
    fn default() -> Self {
        Self {
            in_use: 0,
            colour1: 0,
            colour2: 0,
        }
    }
}

/// Company economy entry for quarterly statistics
#[repr(C)]
#[serde_as]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanyEconomyEntry {
    pub income: Money,
    pub expenses: Money,
    #[serde_as(as = "[_; 64]")]
    pub delivered_cargo: [u32; 64], // CargoArray - 64 cargo types
    pub performance_history: i32, // Company score (0-1000)
    pub company_value: Money,
}

impl Default for CompanyEconomyEntry {
    fn default() -> Self {
        Self {
            income: 0,
            expenses: 0,
            delivered_cargo: [0; 64],
            performance_history: 0,
            company_value: 0,
        }
    }
}

/// Rail type bits
pub type RailTypes = u16;

/// Road type bits
pub type RoadTypes = u64;

/// Company infrastructure counts
#[repr(C)]
#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanyInfrastructure {
    #[serde_as(as = "[_; 16]")]
    pub rail: [u32; 16], // Count for each rail type (RAILTYPE_END = 16)
    #[serde_as(as = "[_; 64]")]
    pub road: [u32; 64], // Count for each road type (ROADTYPE_END = 64)
    pub signal: u32,  // Signal count
    pub water: u32,   // Canal/water count
    pub station: u32, // Station tile count
    pub airport: u32, // Airport count
}

impl Default for CompanyInfrastructure {
    fn default() -> Self {
        Self {
            rail: [0; 16],
            road: [0; 64],
            signal: 0,
            water: 0,
            station: 0,
            airport: 0,
        }
    }
}

impl CompanyInfrastructure {
    /// Get total rail infrastructure
    pub fn get_rail_total(&self) -> u32 {
        self.rail.iter().sum()
    }

    /// Get total road infrastructure
    pub fn get_road_total(&self) -> u32 {
        // First 32 entries are road types
        self.road[..32].iter().sum()
    }

    /// Get total tram infrastructure
    pub fn get_tram_total(&self) -> u32 {
        // Last 32 entries are tram types
        self.road[32..].iter().sum()
    }
}

/// Company manager face
pub type CompanyManagerFace = u32;

/// Expense categories
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
pub enum ExpensesType {
    Construction = 0,
    NewVehicles = 1,
    TrainRunCost = 2,
    RoadVehRunCost = 3,
    AircraftRunCost = 4,
    ShipRunCost = 5,
    PropertyMaint = 6,
    TrainIncome = 7,
    RoadVehIncome = 8,
    AircraftIncome = 9,
    ShipIncome = 10,
    LoanInt = 11,
    Other = 12,
    End = 13,
}

/// Expenses array type
pub type Expenses = [Money; ExpensesType::End as usize];

/// Company settings (simplified for now)
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CompanySettings {
    pub engine_renew: bool,
    pub engine_renew_months: i16,
    pub engine_renew_money: Money,
    pub renew_keep_length: bool,
    pub vehicle_breakdowns: u8,
    pub servint_ispercent: bool,
    pub servint_trains: u16,
    pub servint_roadveh: u16,
    pub servint_aircraft: u16,
    pub servint_ships: u16,
}

/// Main company structure
///
/// This represents the core company data that is saved in savegames.
/// The structure must maintain compatibility with C++ for save/load.
#[repr(C)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    // Pool item fields
    pub index: u8, // CompanyID

    // Name and identity
    pub name_2: u32,
    pub name_1: StringID,
    pub name: String,

    // President
    pub president_name_1: StringID,
    pub president_name_2: u32,
    pub president_name: String,

    // Appearance
    pub face: CompanyManagerFace,
    pub colour: Colours,

    // Financial
    pub money: Money,
    pub money_fraction: u8,
    pub current_loan: Money,
    pub max_loan: Money,

    // Locations
    pub location_of_hq: TileIndex,
    pub last_build_coordinate: TileIndex,

    // Age
    pub inaugurated_year: EconomyYear,
    pub inaugurated_year_calendar: CalendarYear,

    // Bankruptcy
    pub months_of_bankruptcy: u8,
    pub bankrupt_asked: CompanyMask,
    pub bankrupt_timeout: i16,
    pub bankrupt_value: Money,

    // Limits
    pub terraform_limit: u32,
    pub clear_limit: u32,
    pub tree_limit: u32,
    pub build_object_limit: u32,

    // AI flag
    pub is_ai: bool,

    // Statistics
    pub yearly_expenses: [Expenses; 3],
    pub cur_economy: CompanyEconomyEntry,
    pub old_economy: [CompanyEconomyEntry; MAX_HISTORY_QUARTERS],
    pub num_valid_stat_ent: u8,

    // Liveries
    pub livery: [Livery; LS_END],

    // Settings
    pub settings: CompanySettings,

    // Infrastructure
    pub infrastructure: CompanyInfrastructure,

    // Available types
    pub avail_railtypes: RailTypes,
    pub avail_roadtypes: RoadTypes,

    // Other
    pub block_preview: u8,
    pub months_empty: u8, // NOSAVE in multiplayer
}

impl Company {
    /// Create a new company with default values
    pub fn new(index: u8, name_1: StringID) -> Self {
        Self {
            index,
            name_2: 0,
            name_1,
            name: String::new(),
            president_name_1: INVALID_STRING_ID,
            president_name_2: 0,
            president_name: String::new(),
            face: 0,
            colour: Colours::Red,
            money: 100000, // Starting money
            money_fraction: 0,
            current_loan: 100000,
            max_loan: COMPANY_MAX_LOAN_DEFAULT,
            location_of_hq: TileIndex::INVALID,
            last_build_coordinate: TileIndex(0),
            inaugurated_year: EconomyYear(0),
            inaugurated_year_calendar: CalendarYear(0),
            months_of_bankruptcy: 0,
            bankrupt_asked: 0,
            bankrupt_timeout: 0,
            bankrupt_value: 0,
            terraform_limit: 0,
            clear_limit: 0,
            tree_limit: 0,
            build_object_limit: 0,
            is_ai: false,
            yearly_expenses: [[0; 13]; 3],
            cur_economy: CompanyEconomyEntry::default(),
            old_economy: [CompanyEconomyEntry::default(); MAX_HISTORY_QUARTERS],
            num_valid_stat_ent: 0,
            livery: [Livery::default(); LS_END],
            settings: CompanySettings::default(),
            infrastructure: CompanyInfrastructure::default(),
            avail_railtypes: 0,
            avail_roadtypes: 0,
            block_preview: 0,
            months_empty: 0,
        }
    }

    /// Check if company is controlled by AI
    pub fn is_ai_company(&self) -> bool {
        self.is_ai
    }

    /// Check if company is controlled by human
    pub fn is_human_company(&self) -> bool {
        !self.is_ai
    }

    /// Get company owner
    pub fn as_owner(&self) -> Owner {
        match self.index {
            0..=14 => Owner::from_company_id(self.index),
            _ => Owner::Invalid,
        }
    }

    /// Get company colour offset for recolouring
    pub fn get_company_recolour_offset(
        &self,
        livery_scheme: LiveryScheme,
        use_secondary: bool,
    ) -> u8 {
        let livery = &self.livery[livery_scheme as usize];
        if use_secondary {
            livery.colour1 + livery.colour2 * 16
        } else {
            livery.colour1
        }
    }

    /// Get maximum loan for this company
    pub fn get_max_loan(&self) -> Money {
        if self.max_loan == COMPANY_MAX_LOAN_DEFAULT {
            // Would get from game settings
            500000
        } else {
            self.max_loan
        }
    }
}

// StringID invalid constant defined in types.rs

/// Year type implementations
impl EconomyYear {
    pub fn new(year: i32) -> Self {
        EconomyYear(year)
    }
}

impl CalendarYear {
    pub fn new(year: i32) -> Self {
        CalendarYear(year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn test_company_economy_entry_size() {
        let size = mem::size_of::<CompanyEconomyEntry>();
        println!("CompanyEconomyEntry size: {} bytes", size);
        // Should be 16 + 256 + 4 + 8 = 284 bytes or similar
        assert!(size < 512, "CompanyEconomyEntry unexpectedly large");
    }

    #[test]
    fn test_livery_scheme_values() {
        assert_eq!(LiveryScheme::Default as u8, 0);
        assert_eq!(LiveryScheme::Bus as u8, 10);
        assert_eq!(LiveryScheme::End as u8, 20);
        assert_eq!(LS_END, 20);
    }

    #[test]
    fn test_company_infrastructure() {
        let mut infra = CompanyInfrastructure::default();
        infra.rail[0] = 100;
        infra.rail[1] = 50;
        assert_eq!(infra.get_rail_total(), 150);

        infra.road[0] = 200; // Road
        infra.road[32] = 75; // Tram
        assert_eq!(infra.get_road_total(), 200);
        assert_eq!(infra.get_tram_total(), 75);
    }

    #[test]
    fn test_company_creation() {
        let company = Company::new(0, 0x100);
        assert_eq!(company.index, 0);
        assert_eq!(company.name_1, 0x100);
        assert!(!company.is_ai);
        assert_eq!(company.as_owner(), Owner::Company0);
    }

    #[test]
    fn test_company_ai_flags() {
        let mut company = Company::new(5, 0);
        assert!(company.is_human_company());
        assert!(!company.is_ai_company());

        company.is_ai = true;
        assert!(!company.is_human_company());
        assert!(company.is_ai_company());
    }
}
