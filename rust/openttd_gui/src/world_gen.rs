//! World Generation GUI
//!
//! This module provides the world generation configuration window that allows
//! players to set up parameters for generating a new game world.

use crate::{
    ButtonWidget, ContainerWidget, LabelWidget, PanelWidget, Rect, Widget, WidgetID, Window,
    WindowID, WindowManager,
};
use openttd_gfx::{Colour, GfxContext};

/// Window ID for the world generation window
pub const WORLD_GEN_WINDOW_ID: WindowID = 6000;

/// Widget IDs for world generation window
#[derive(Debug, Clone, Copy)]
pub enum WorldGenWidgets {
    Window = 6000,
    Panel = 6001,

    // Climate selection
    ClimateTemperate = 6010,
    ClimateArctic = 6011,
    ClimateTropical = 6012,
    ClimateToyland = 6013,

    // Map configuration
    MapSizeLabel = 6020,
    MapSizeXDropdown = 6021,
    MapSizeYDropdown = 6022,

    TerrainTypeLabel = 6030,
    TerrainTypeDropdown = 6031,

    SeaLevelLabel = 6040,
    SeaLevelDropdown = 6041,

    // Generation settings
    NumTownsLabel = 6050,
    NumTownsDropdown = 6051,

    NumIndustriesLabel = 6060,
    NumIndustriesDropdown = 6061,

    StartDateLabel = 6070,
    StartDateText = 6071,
    StartDateDown = 6072,
    StartDateUp = 6073,

    // Control buttons
    GenerateButton = 6100,
    CancelButton = 6101,

    // Advanced settings
    RandomSeedLabel = 6110,
    RandomSeedText = 6111,
}

/// Climate types available in OpenTTD
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Climate {
    Temperate,
    Arctic,
    Tropical,
    Toyland,
}

impl Climate {
    pub fn name(&self) -> &'static str {
        match self {
            Climate::Temperate => "Temperate",
            Climate::Arctic => "Arctic",
            Climate::Tropical => "Tropical",
            Climate::Toyland => "Toyland",
        }
    }
}

/// Map size options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapSize {
    Size64,
    Size128,
    Size256,
    Size512,
    Size1024,
    Size2048,
    Size4096,
}

impl MapSize {
    pub fn value(&self) -> u32 {
        match self {
            MapSize::Size64 => 64,
            MapSize::Size128 => 128,
            MapSize::Size256 => 256,
            MapSize::Size512 => 512,
            MapSize::Size1024 => 1024,
            MapSize::Size2048 => 2048,
            MapSize::Size4096 => 4096,
        }
    }

    pub fn name(&self) -> String {
        format!("{}", self.value())
    }
}

/// Terrain type options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerrainType {
    VeryFlat,
    Flat,
    Hilly,
    Mountainous,
    Custom,
}

impl TerrainType {
    pub fn name(&self) -> &'static str {
        match self {
            TerrainType::VeryFlat => "Very Flat",
            TerrainType::Flat => "Flat",
            TerrainType::Hilly => "Hilly",
            TerrainType::Mountainous => "Mountainous",
            TerrainType::Custom => "Custom",
        }
    }
}

/// Sea level options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeaLevel {
    VeryLow,
    Low,
    Medium,
    High,
    Custom,
}

impl SeaLevel {
    pub fn name(&self) -> &'static str {
        match self {
            SeaLevel::VeryLow => "Very Low",
            SeaLevel::Low => "Low",
            SeaLevel::Medium => "Medium",
            SeaLevel::High => "High",
            SeaLevel::Custom => "Custom",
        }
    }
}

/// Number of towns options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TownCount {
    VeryLow,
    Low,
    Normal,
    High,
    Custom,
}

impl TownCount {
    pub fn name(&self) -> &'static str {
        match self {
            TownCount::VeryLow => "Very Low",
            TownCount::Low => "Low",
            TownCount::Normal => "Normal",
            TownCount::High => "High",
            TownCount::Custom => "Custom",
        }
    }
}

/// Number of industries options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndustryCount {
    None,
    VeryLow,
    Low,
    Normal,
    High,
}

impl IndustryCount {
    pub fn name(&self) -> &'static str {
        match self {
            IndustryCount::None => "None",
            IndustryCount::VeryLow => "Very Low",
            IndustryCount::Low => "Low",
            IndustryCount::Normal => "Normal",
            IndustryCount::High => "High",
        }
    }
}

/// World generation configuration
pub struct WorldGenConfig {
    pub climate: Climate,
    pub map_size_x: MapSize,
    pub map_size_y: MapSize,
    pub terrain_type: TerrainType,
    pub sea_level: SeaLevel,
    pub town_count: TownCount,
    pub industry_count: IndustryCount,
    pub start_year: u32,
    pub random_seed: u32,
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        Self {
            climate: Climate::Temperate,
            map_size_x: MapSize::Size512,
            map_size_y: MapSize::Size512,
            terrain_type: TerrainType::Hilly,
            sea_level: SeaLevel::Medium,
            town_count: TownCount::Normal,
            industry_count: IndustryCount::Normal,
            start_year: 1950,
            random_seed: 0, // 0 means random
        }
    }
}

/// World generation window
pub struct WorldGenWindow {
    config: WorldGenConfig,
}

impl WorldGenWindow {
    pub fn new() -> Self {
        Self {
            config: WorldGenConfig::default(),
        }
    }

    /// Convert to a Window for the window manager
    pub fn build_window(&self) -> Window {
        let mut window = Window::new(
            WORLD_GEN_WINDOW_ID,
            "World Generation",
            Rect::new(100, 50, 600, 500),
        );

        // Create main panel
        let panel = PanelWidget::new(WorldGenWidgets::Panel as WidgetID)
            .with_colour(Colour::ui_background());

        // Create main vertical container
        let mut main_container = ContainerWidget::new_vertical(6200)
            .with_padding(10)
            .with_spacing(10);

        // Climate selection section
        let mut climate_section = ContainerWidget::new_vertical(6201);
        climate_section.add_child(Box::new(
            LabelWidget::new(6202, "Select Climate:").with_colour(Colour::ui_text()),
        ));

        let mut climate_buttons = ContainerWidget::new_horizontal(6203).with_spacing(10);

        climate_buttons.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::ClimateTemperate as WidgetID,
            climate_button_label(Climate::Temperate, self.config.climate),
        )));
        climate_buttons.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::ClimateArctic as WidgetID,
            climate_button_label(Climate::Arctic, self.config.climate),
        )));
        climate_buttons.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::ClimateTropical as WidgetID,
            climate_button_label(Climate::Tropical, self.config.climate),
        )));
        climate_buttons.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::ClimateToyland as WidgetID,
            climate_button_label(Climate::Toyland, self.config.climate),
        )));

        climate_section.add_child(Box::new(climate_buttons));
        main_container.add_child(Box::new(climate_section));

        // Map size section
        let mut map_size_section = ContainerWidget::new_horizontal(6210);
        map_size_section.add_child(Box::new(LabelWidget::new(
            WorldGenWidgets::MapSizeLabel as WidgetID,
            "Map Size:",
        )));
        map_size_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::MapSizeXDropdown as WidgetID,
            &self.config.map_size_x.name(),
        )));
        map_size_section.add_child(Box::new(LabelWidget::new(6211, "x")));
        map_size_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::MapSizeYDropdown as WidgetID,
            &self.config.map_size_y.name(),
        )));
        main_container.add_child(Box::new(map_size_section));

        // Terrain type section
        let mut terrain_section = ContainerWidget::new_horizontal(6220);
        terrain_section.add_child(Box::new(LabelWidget::new(
            WorldGenWidgets::TerrainTypeLabel as WidgetID,
            "Terrain Type:",
        )));
        terrain_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::TerrainTypeDropdown as WidgetID,
            self.config.terrain_type.name(),
        )));
        main_container.add_child(Box::new(terrain_section));

        // Sea level section
        let mut sea_level_section = ContainerWidget::new_horizontal(6230);
        sea_level_section.add_child(Box::new(LabelWidget::new(
            WorldGenWidgets::SeaLevelLabel as WidgetID,
            "Sea Level:",
        )));
        sea_level_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::SeaLevelDropdown as WidgetID,
            self.config.sea_level.name(),
        )));
        main_container.add_child(Box::new(sea_level_section));

        // Number of towns section
        let mut towns_section = ContainerWidget::new_horizontal(6240);
        towns_section.add_child(Box::new(LabelWidget::new(
            WorldGenWidgets::NumTownsLabel as WidgetID,
            "Number of Towns:",
        )));
        towns_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::NumTownsDropdown as WidgetID,
            self.config.town_count.name(),
        )));
        main_container.add_child(Box::new(towns_section));

        // Number of industries section
        let mut industries_section = ContainerWidget::new_horizontal(6250);
        industries_section.add_child(Box::new(LabelWidget::new(
            WorldGenWidgets::NumIndustriesLabel as WidgetID,
            "Number of Industries:",
        )));
        industries_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::NumIndustriesDropdown as WidgetID,
            self.config.industry_count.name(),
        )));
        main_container.add_child(Box::new(industries_section));

        // Start date section
        let mut date_section = ContainerWidget::new_horizontal(6260);
        date_section.add_child(Box::new(LabelWidget::new(
            WorldGenWidgets::StartDateLabel as WidgetID,
            "Start Year:",
        )));
        date_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::StartDateDown as WidgetID,
            "◀",
        )));
        date_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::StartDateText as WidgetID,
            &format!("{}", self.config.start_year),
        )));
        date_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::StartDateUp as WidgetID,
            "▶",
        )));
        main_container.add_child(Box::new(date_section));

        // Random seed section
        let mut seed_section = ContainerWidget::new_horizontal(6270);
        seed_section.add_child(Box::new(LabelWidget::new(
            WorldGenWidgets::RandomSeedLabel as WidgetID,
            "Random Seed:",
        )));
        seed_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::RandomSeedText as WidgetID,
            &random_seed_label(self.config.random_seed),
        )));
        main_container.add_child(Box::new(seed_section));

        // Control buttons
        let mut button_section = ContainerWidget::new_horizontal(6280).with_spacing(20);
        button_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::GenerateButton as WidgetID,
            "🎲 Generate",
        )));
        button_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::CancelButton as WidgetID,
            "❌ Cancel",
        )));
        main_container.add_child(Box::new(button_section));

        window.set_root_widget(Box::new(main_container));
        window
    }

    pub fn apply_action(&mut self, action: &str) -> bool {
        match action {
            "CLIMATE_TEMPERATE" => self.config.climate = Climate::Temperate,
            "CLIMATE_ARCTIC" => self.config.climate = Climate::Arctic,
            "CLIMATE_TROPICAL" => self.config.climate = Climate::Tropical,
            "CLIMATE_TOYLAND" => self.config.climate = Climate::Toyland,
            "CYCLE_MAP_SIZE_X" => self.config.map_size_x = next_map_size(self.config.map_size_x),
            "CYCLE_MAP_SIZE_Y" => self.config.map_size_y = next_map_size(self.config.map_size_y),
            "CYCLE_TERRAIN_TYPE" => {
                self.config.terrain_type = next_terrain_type(self.config.terrain_type)
            }
            "CYCLE_SEA_LEVEL" => self.config.sea_level = next_sea_level(self.config.sea_level),
            "CYCLE_NUM_TOWNS" => self.config.town_count = next_town_count(self.config.town_count),
            "CYCLE_NUM_INDUSTRIES" => {
                self.config.industry_count = next_industry_count(self.config.industry_count)
            }
            "YEAR_UP" => self.config.start_year = self.config.start_year.saturating_add(1),
            "YEAR_DOWN" => self.config.start_year = self.config.start_year.saturating_sub(1),
            _ => return false,
        }
        true
    }
}

/// Show the world generation window
pub fn show_world_gen(window_manager: &mut WindowManager, world_gen: &WorldGenWindow) -> WindowID {
    let window = world_gen.build_window();
    window_manager.add_window(window)
}

fn climate_button_label(climate: Climate, selected: Climate) -> String {
    if climate == selected {
        format!("▶ {}", climate.name())
    } else {
        climate.name().to_string()
    }
}

fn random_seed_label(seed: u32) -> String {
    if seed == 0 {
        "Random".to_string()
    } else {
        seed.to_string()
    }
}

fn next_map_size(size: MapSize) -> MapSize {
    match size {
        MapSize::Size64 => MapSize::Size128,
        MapSize::Size128 => MapSize::Size256,
        MapSize::Size256 => MapSize::Size512,
        MapSize::Size512 => MapSize::Size1024,
        MapSize::Size1024 => MapSize::Size2048,
        MapSize::Size2048 => MapSize::Size4096,
        MapSize::Size4096 => MapSize::Size64,
    }
}

fn next_terrain_type(terrain: TerrainType) -> TerrainType {
    match terrain {
        TerrainType::VeryFlat => TerrainType::Flat,
        TerrainType::Flat => TerrainType::Hilly,
        TerrainType::Hilly => TerrainType::Mountainous,
        TerrainType::Mountainous => TerrainType::Custom,
        TerrainType::Custom => TerrainType::VeryFlat,
    }
}

fn next_sea_level(level: SeaLevel) -> SeaLevel {
    match level {
        SeaLevel::VeryLow => SeaLevel::Low,
        SeaLevel::Low => SeaLevel::Medium,
        SeaLevel::Medium => SeaLevel::High,
        SeaLevel::High => SeaLevel::Custom,
        SeaLevel::Custom => SeaLevel::VeryLow,
    }
}

fn next_town_count(count: TownCount) -> TownCount {
    match count {
        TownCount::VeryLow => TownCount::Low,
        TownCount::Low => TownCount::Normal,
        TownCount::Normal => TownCount::High,
        TownCount::High => TownCount::Custom,
        TownCount::Custom => TownCount::VeryLow,
    }
}

fn next_industry_count(count: IndustryCount) -> IndustryCount {
    match count {
        IndustryCount::None => IndustryCount::VeryLow,
        IndustryCount::VeryLow => IndustryCount::Low,
        IndustryCount::Low => IndustryCount::Normal,
        IndustryCount::Normal => IndustryCount::High,
        IndustryCount::High => IndustryCount::None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_gen_apply_action_updates_config() {
        let mut window = WorldGenWindow::new();
        assert!(window.apply_action("CLIMATE_TROPICAL"));
        assert_eq!(window.config.climate, Climate::Tropical);

        let previous_size = window.config.map_size_x;
        assert!(window.apply_action("CYCLE_MAP_SIZE_X"));
        assert_ne!(window.config.map_size_x, previous_size);
    }

    #[test]
    fn test_world_gen_year_changes() {
        let mut window = WorldGenWindow::new();
        let start_year = window.config.start_year;
        assert!(window.apply_action("YEAR_UP"));
        assert_eq!(window.config.start_year, start_year + 1);
        assert!(window.apply_action("YEAR_DOWN"));
        assert_eq!(window.config.start_year, start_year);
    }

    #[test]
    fn test_world_gen_window_id_and_title() {
        let window = WorldGenWindow::new().build_window();
        assert_eq!(window.id, WORLD_GEN_WINDOW_ID);
        assert_eq!(window.title, "World Generation");
    }
}
