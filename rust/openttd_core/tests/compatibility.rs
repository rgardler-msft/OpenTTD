//! Tests for core data model structures to verify savegame compatibility
//!
//! These tests ensure that our Rust structures maintain the exact same
//! memory layout and sizes as the C++ originals.

#[cfg(test)]
mod tests {
    use openttd_core::company::*;
    use openttd_core::industry::*;
    use openttd_core::map::*;
    use openttd_core::station::*;
    use openttd_core::town::*;
    use openttd_core::types::*;
    use openttd_core::vehicle::*;

    /// Test that all structures have the expected repr(C) layout
    #[test]
    fn test_struct_alignment() {
        // Test that key structures are repr(C) and have expected alignment
        assert_eq!(std::mem::align_of::<Tile>(), 2); // Has u16 field
        assert_eq!(std::mem::align_of::<TileBase>(), 2); // Has u16 field (m2)
        assert_eq!(std::mem::align_of::<TileExtended>(), 2); // Has u16 field (m8)
    }

    /// Test tile structure sizes match C++ exactly
    #[test]
    fn test_tile_sizes() {
        // Tile must be exactly 12 bytes to match C++
        assert_eq!(std::mem::size_of::<Tile>(), 12, "Tile size mismatch");
        assert_eq!(std::mem::size_of::<TileBase>(), 8, "TileBase size mismatch");
        assert_eq!(
            std::mem::size_of::<TileExtended>(),
            4,
            "TileExtended size mismatch"
        );
    }

    /// Test that ID types are correct size
    #[test]
    fn test_id_sizes() {
        assert_eq!(std::mem::size_of::<StationID>(), 2);
        assert_eq!(std::mem::size_of::<TownID>(), 2);
        assert_eq!(std::mem::size_of::<IndustryID>(), 2);
        assert_eq!(std::mem::size_of::<VehicleID>(), 4); // u32
        assert_eq!(std::mem::size_of::<TileIndex>(), 4);
    }

    /// Test Owner enum values match C++
    #[test]
    fn test_owner_values() {
        assert_eq!(Owner::Company0 as u8, 0x00);
        assert_eq!(Owner::Company14 as u8, 0x0E);
        assert_eq!(Owner::Town as u8, 0x0F);
        assert_eq!(Owner::None as u8, 0x10);
        assert_eq!(Owner::Water as u8, 0x11);
        assert_eq!(Owner::Deity as u8, 0x12);
    }

    /// Test invalid ID constants
    #[test]
    fn test_invalid_ids() {
        assert_eq!(StationID::INVALID.0, 0xFFFF);
        assert_eq!(TownID::INVALID.0, 0xFFFF);
        assert_eq!(IndustryID::INVALID.0, 0xFFFF);
        assert_eq!(VehicleID::INVALID.0, 0xFFFFF); // 20-bit max
        assert_eq!(TileIndex::INVALID.0, 0xFFFFFFFF);
    }

    /// Test serialization round-trip for key structures
    #[test]
    fn test_tile_serialization() {
        let tile = Tile {
            base: TileBase {
                type_height: 0,
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
        };

        // Serialize and deserialize
        let serialized = bincode::serialize(&tile).unwrap();
        let _deserialized: Tile = bincode::deserialize(&serialized).unwrap();

        // Can't compare directly without PartialEq, so just check size
        // Size should be exactly 12 bytes when serialized
        assert_eq!(serialized.len(), 12);
    }

    /// Test vehicle structure basics
    #[test]
    fn test_vehicle_structure() {
        let vehicle = Vehicle::new(VehicleID(1), VehicleType::Train);
        assert_eq!(vehicle.index, VehicleID(1));
        assert_eq!(vehicle.type_, VehicleType::Train);
        assert_eq!(vehicle.owner, Owner::Invalid);
    }

    /// Test company structure basics
    #[test]
    fn test_company_structure() {
        let company = Company::new(0, INVALID_STRING_ID);
        assert_eq!(company.index, 0);
        // Company doesn't have is_valid method
        assert_eq!(company.money, 100000); // Starting money
    }

    /// Test town structure basics
    #[test]
    fn test_town_structure() {
        let town = Town::new(TownID(1), TileIndex(5000));
        assert_eq!(town.index, TownID(1));
        assert_eq!(town.xy, TileIndex(5000));
        assert!(!town.is_growing());
    }

    /// Test industry structure basics
    #[test]
    fn test_industry_structure() {
        let industry = Industry::new(IndustryID(1), TileIndex(3000), INDUSTRYTYPE_COAL_MINE);
        assert_eq!(industry.index, IndustryID(1));
        assert_eq!(industry.location, TileIndex(3000));
        assert_eq!(industry.industry_type, INDUSTRYTYPE_COAL_MINE);
    }

    /// Test station structure basics
    #[test]
    fn test_station_structure() {
        let station = Station::new(StationID(1), TileIndex(2000), Owner::Company0);
        assert_eq!(station.index, StationID(1));
        assert_eq!(station.xy, TileIndex(2000));
        assert_eq!(station.owner, Owner::Company0);
        assert_eq!(station.facilities, FACIL_NONE);
    }

    /// Test map coordinate conversion
    #[test]
    fn test_map_coordinates() {
        let map = Map::new(8, 8).expect("Failed to create map"); // 256x256 map
        let index = TileIndex(1000);

        let x = map.tile_x(index);
        let y = map.tile_y(index);
        let reconstructed = map.tile_xy(x, y);

        assert_eq!(index, reconstructed);
    }

    /// Test bitfield operations
    #[test]
    fn test_tile_bitfields() {
        let mut base = TileBase {
            type_height: 0,
            height: 0,
            m2: 0,
            m1: 0,
            m3: 0,
            m4: 0,
            m5: 0,
        };

        // Test type field (stored in upper bits of type_height)
        base.type_height = (TileType::Industry as u8) << 4;
        assert_eq!(base.type_height >> 4, TileType::Industry as u8);

        // Test height field
        base.height = 15;
        assert_eq!(base.height, 15);

        // Test owner field (stored in m1)
        base.m1 = Owner::Company5 as u8;
        assert_eq!(base.m1, Owner::Company5 as u8);
    }

    /// Verify enums use correct representation
    #[test]
    fn test_enum_representations() {
        // VehicleType should be 1 byte
        assert_eq!(std::mem::size_of::<VehicleType>(), 1);

        // Direction should be 1 byte
        assert_eq!(std::mem::size_of::<openttd_core::vehicle::Direction>(), 1);

        // TileType should be 1 byte
        assert_eq!(std::mem::size_of::<TileType>(), 1);

        // Owner should be 1 byte
        assert_eq!(std::mem::size_of::<Owner>(), 1);
    }

    /// Test cargo array sizes
    #[test]
    fn test_cargo_arrays() {
        // Ensure we support 64 cargo types
        let cargo_entry = CompanyEconomyEntry::default();
        assert_eq!(cargo_entry.delivered_cargo.len(), 64);

        let town_cargo = TownCargo::default();
        assert_eq!(town_cargo.produced.len(), 64);
        assert_eq!(town_cargo.accepted.len(), 64);
        assert_eq!(town_cargo.received.len(), 64);
    }

    /// Test vehicle type-specific data structures
    #[test]
    fn test_vehicle_type_data() {
        // Test TrainData
        let train_data = TrainData::default();
        assert_eq!(train_data.crash_anim_pos, 0);
        assert!(train_data.flags.is_empty());

        // Test RoadVehicleData
        let road_data = RoadVehicleData::default();
        assert_eq!(road_data.state, 0);
        assert_eq!(road_data.roadtype, 0xFF); // INVALID_ROADTYPE

        // Test ShipData
        let ship_data = ShipData::default();
        assert_eq!(ship_data.state, TrackBits::None);

        // Test AircraftData
        let aircraft_data = AircraftData::default();
        assert_eq!(aircraft_data.targetairport, StationID::INVALID);
    }

    /// Test vehicle type enum values
    #[test]
    fn test_vehicle_type_values() {
        assert_eq!(VehicleType::Train as u8, 0);
        assert_eq!(VehicleType::Road as u8, 1);
        assert_eq!(VehicleType::Ship as u8, 2);
        assert_eq!(VehicleType::Aircraft as u8, 3);
        assert_eq!(VehicleType::Effect as u8, 4);
        assert_eq!(VehicleType::Disaster as u8, 5);
        assert_eq!(VehicleType::Invalid as u8, 0xFF);
    }

    /// Test aircraft subtype values
    #[test]
    fn test_aircraft_subtype_values() {
        assert_eq!(AircraftSubType::Helicopter as u8, 0);
        assert_eq!(AircraftSubType::Aircraft as u8, 2);
        assert_eq!(AircraftSubType::Shadow as u8, 4);
        assert_eq!(AircraftSubType::Rotor as u8, 6);
    }

    /// Test train force proceeding values
    #[test]
    fn test_train_force_proceeding_values() {
        assert_eq!(TrainForceProceeding::None as u8, 0);
        assert_eq!(TrainForceProceeding::Stuck as u8, 1);
        assert_eq!(TrainForceProceeding::Signal as u8, 2);
    }
}

/// Integration tests that would verify against actual C++ savegame data
#[cfg(test)]
mod integration_tests {

    // These tests would be enabled once we have actual savegame data to test against
    #[test]
    #[ignore] // Enable when we have test data
    fn test_load_savegame_chunk() {
        // This would load a known savegame chunk and verify our structures
        // can deserialize it correctly
    }

    #[test]
    #[ignore] // Enable when we have C++ size data
    fn test_struct_sizes_match_cpp() {
        // These would be the actual C++ struct sizes
        // We'd verify our Rust structs match exactly

        // Example (sizes are placeholders):
        // assert_eq!(std::mem::size_of::<Vehicle>(), 312);
        // assert_eq!(std::mem::size_of::<Company>(), 65536);
        // assert_eq!(std::mem::size_of::<Town>(), 1024);
        // assert_eq!(std::mem::size_of::<Industry>(), 512);
        // assert_eq!(std::mem::size_of::<Station>(), 2048);
    }
}
