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
    pub fn into_window(self) -> Window {
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
            "🌲 Temperate",
        )));
        climate_buttons.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::ClimateArctic as WidgetID,
            "❄️ Arctic",
        )));
        climate_buttons.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::ClimateTropical as WidgetID,
            "🌴 Tropical",
        )));
        climate_buttons.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::ClimateToyland as WidgetID,
            "🎪 Toyland",
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
            "512",
        )));
        map_size_section.add_child(Box::new(LabelWidget::new(6211, "x")));
        map_size_section.add_child(Box::new(ButtonWidget::new(
            WorldGenWidgets::MapSizeYDropdown as WidgetID,
            "512",
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
            "Hilly",
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
            "Medium",
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
            "Normal",
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
            "Normal",
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
            "1950",
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
            "Random",
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
}

/// Show the world generation window
pub fn show_world_gen(window_manager: &mut WindowManager) -> WindowID {
    let world_gen = WorldGenWindow::new();
    window_manager.add_window(world_gen.into_window())
}
