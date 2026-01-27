//! Toolbar Window - Main game toolbar
//!
//! This module implements the toolbar that appears at the top of the main game window.
//! Based on the original toolbar_gui.cpp implementation.

use crate::{
    Alignment, ButtonWidget, ContainerWidget, LabelWidget, Rect, Widget, WidgetID, Window,
    WindowID, WindowManager,
};
use openttd_gfx::{Colour, GfxContext};

/// Window ID for the toolbar
pub const TOOLBAR_WINDOW_ID: WindowID = 5000;

/// Widget IDs for toolbar buttons
pub struct ToolbarWidgets;

impl ToolbarWidgets {
    pub const PAUSE: WidgetID = 5001;
    pub const FAST_FORWARD: WidgetID = 5002;
    pub const OPTIONS: WidgetID = 5003;
    pub const SAVE: WidgetID = 5004;
    pub const LOAD: WidgetID = 5005;
    pub const MAP: WidgetID = 5006;
    pub const TOWN_DIRECTORY: WidgetID = 5007;
    pub const SUBSIDIES: WidgetID = 5008;
    pub const STATIONS: WidgetID = 5009;
    pub const FINANCES: WidgetID = 5010;
    pub const COMPANY: WidgetID = 5011;
    pub const GRAPHS: WidgetID = 5012;
    pub const LEAGUE: WidgetID = 5013;
    pub const INDUSTRIES: WidgetID = 5014;
    pub const TRAINS: WidgetID = 5015;
    pub const ROAD_VEHICLES: WidgetID = 5016;
    pub const SHIPS: WidgetID = 5017;
    pub const AIRCRAFT: WidgetID = 5018;
    pub const ZOOM_IN: WidgetID = 5019;
    pub const ZOOM_OUT: WidgetID = 5020;
    pub const BUILD_RAIL: WidgetID = 5021;
    pub const BUILD_ROAD: WidgetID = 5022;
    pub const BUILD_WATER: WidgetID = 5023;
    pub const BUILD_AIRPORT: WidgetID = 5024;
    pub const MUSIC: WidgetID = 5025;
    pub const NEWS: WidgetID = 5026;
    pub const HELP: WidgetID = 5027;
}

/// Toolbar window
pub struct ToolbarWindow {
    window: Window,
}

impl ToolbarWindow {
    /// Create a new toolbar window
    pub fn new(screen_width: u32) -> Self {
        let toolbar_height = 35;
        let mut window = Window::new(
            TOOLBAR_WINDOW_ID,
            "", // No title for toolbar
            Rect::new(0, 0, screen_width, toolbar_height),
        );

        // Make toolbar non-closeable and non-resizable
        window.closeable = false;
        window.resizable = false;

        // Create horizontal container for toolbar buttons
        let mut container = ContainerWidget::new_horizontal(6000)
            .with_spacing(2)
            .with_padding(2);

        // Add toolbar buttons (simplified version - not all buttons from original)

        // Pause button
        let mut pause = ButtonWidget::new(ToolbarWidgets::PAUSE, "⏸");
        pause.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(pause));

        // Fast forward button
        let mut fast_forward = ButtonWidget::new(ToolbarWidgets::FAST_FORWARD, "⏩");
        fast_forward.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(fast_forward));

        // Separator
        let mut sep1 = LabelWidget::new(6001, "|").with_colour(Colour::GREY);
        sep1.set_rect(Rect::new(0, 0, 10, 30));
        container.add_child(Box::new(sep1));

        // Options button
        let mut options = ButtonWidget::new(ToolbarWidgets::OPTIONS, "⚙");
        options.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(options));

        // Save button
        let mut save = ButtonWidget::new(ToolbarWidgets::SAVE, "💾");
        save.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(save));

        // Load button
        let mut load = ButtonWidget::new(ToolbarWidgets::LOAD, "📁");
        load.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(load));

        // Separator
        let mut sep2 = LabelWidget::new(6002, "|").with_colour(Colour::GREY);
        sep2.set_rect(Rect::new(0, 0, 10, 30));
        container.add_child(Box::new(sep2));

        // Map button
        let mut map = ButtonWidget::new(ToolbarWidgets::MAP, "🗺");
        map.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(map));

        // Towns button
        let mut towns = ButtonWidget::new(ToolbarWidgets::TOWN_DIRECTORY, "🏘");
        towns.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(towns));

        // Finances button
        let mut finances = ButtonWidget::new(ToolbarWidgets::FINANCES, "💰");
        finances.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(finances));

        // Company button
        let mut company = ButtonWidget::new(ToolbarWidgets::COMPANY, "🏢");
        company.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(company));

        // Graphs button
        let mut graphs = ButtonWidget::new(ToolbarWidgets::GRAPHS, "📊");
        graphs.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(graphs));

        // League button
        let mut league = ButtonWidget::new(ToolbarWidgets::LEAGUE, "🏆");
        league.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(league));

        // Separator
        let mut sep3 = LabelWidget::new(6003, "|").with_colour(Colour::GREY);
        sep3.set_rect(Rect::new(0, 0, 10, 30));
        container.add_child(Box::new(sep3));

        // Train button
        let mut trains = ButtonWidget::new(ToolbarWidgets::TRAINS, "🚂");
        trains.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(trains));

        // Road vehicles button
        let mut road = ButtonWidget::new(ToolbarWidgets::ROAD_VEHICLES, "🚛");
        road.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(road));

        // Ships button
        let mut ships = ButtonWidget::new(ToolbarWidgets::SHIPS, "🚢");
        ships.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(ships));

        // Aircraft button
        let mut aircraft = ButtonWidget::new(ToolbarWidgets::AIRCRAFT, "✈");
        aircraft.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(aircraft));

        // Separator
        let mut sep4 = LabelWidget::new(6004, "|").with_colour(Colour::GREY);
        sep4.set_rect(Rect::new(0, 0, 10, 30));
        container.add_child(Box::new(sep4));

        // Zoom in button
        let mut zoom_in = ButtonWidget::new(ToolbarWidgets::ZOOM_IN, "🔍+");
        zoom_in.set_rect(Rect::new(0, 0, 35, 30));
        container.add_child(Box::new(zoom_in));

        // Zoom out button
        let mut zoom_out = ButtonWidget::new(ToolbarWidgets::ZOOM_OUT, "🔍-");
        zoom_out.set_rect(Rect::new(0, 0, 35, 30));
        container.add_child(Box::new(zoom_out));

        // Separator
        let mut sep5 = LabelWidget::new(6005, "|").with_colour(Colour::GREY);
        sep5.set_rect(Rect::new(0, 0, 10, 30));
        container.add_child(Box::new(sep5));

        // Build rail button
        let mut build_rail = ButtonWidget::new(ToolbarWidgets::BUILD_RAIL, "🛤");
        build_rail.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(build_rail));

        // Build road button
        let mut build_road = ButtonWidget::new(ToolbarWidgets::BUILD_ROAD, "🛣");
        build_road.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(build_road));

        // Build water button
        let mut build_water = ButtonWidget::new(ToolbarWidgets::BUILD_WATER, "⚓");
        build_water.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(build_water));

        // Build airport button
        let mut build_airport = ButtonWidget::new(ToolbarWidgets::BUILD_AIRPORT, "🛩");
        build_airport.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(build_airport));

        // Separator
        let mut sep6 = LabelWidget::new(6006, "|").with_colour(Colour::GREY);
        sep6.set_rect(Rect::new(0, 0, 10, 30));
        container.add_child(Box::new(sep6));

        // Music button
        let mut music = ButtonWidget::new(ToolbarWidgets::MUSIC, "🎵");
        music.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(music));

        // News button
        let mut news = ButtonWidget::new(ToolbarWidgets::NEWS, "📰");
        news.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(news));

        // Help button
        let mut help = ButtonWidget::new(ToolbarWidgets::HELP, "❓");
        help.set_rect(Rect::new(0, 0, 30, 30));
        container.add_child(Box::new(help));

        // Set the container as the root widget
        window.set_root_widget(Box::new(container));

        Self { window }
    }

    /// Get the window
    pub fn into_window(self) -> Window {
        self.window
    }

    /// Handle toolbar button clicks
    pub fn handle_click(widget_id: WidgetID) -> Option<String> {
        match widget_id {
            ToolbarWidgets::PAUSE => Some("PAUSE_GAME".to_string()),
            ToolbarWidgets::FAST_FORWARD => Some("FAST_FORWARD".to_string()),
            ToolbarWidgets::OPTIONS => Some("OPEN_OPTIONS".to_string()),
            ToolbarWidgets::SAVE => Some("SAVE_GAME".to_string()),
            ToolbarWidgets::LOAD => Some("LOAD_GAME".to_string()),
            ToolbarWidgets::MAP => Some("SHOW_MAP".to_string()),
            ToolbarWidgets::TOWN_DIRECTORY => Some("SHOW_TOWNS".to_string()),
            ToolbarWidgets::SUBSIDIES => Some("SHOW_SUBSIDIES".to_string()),
            ToolbarWidgets::STATIONS => Some("SHOW_STATIONS".to_string()),
            ToolbarWidgets::FINANCES => Some("SHOW_FINANCES".to_string()),
            ToolbarWidgets::COMPANY => Some("SHOW_COMPANY".to_string()),
            ToolbarWidgets::GRAPHS => Some("SHOW_GRAPHS".to_string()),
            ToolbarWidgets::LEAGUE => Some("SHOW_LEAGUE".to_string()),
            ToolbarWidgets::INDUSTRIES => Some("SHOW_INDUSTRIES".to_string()),
            ToolbarWidgets::TRAINS => Some("BUILD_TRAINS".to_string()),
            ToolbarWidgets::ROAD_VEHICLES => Some("BUILD_ROAD_VEHICLES".to_string()),
            ToolbarWidgets::SHIPS => Some("BUILD_SHIPS".to_string()),
            ToolbarWidgets::AIRCRAFT => Some("BUILD_AIRCRAFT".to_string()),
            ToolbarWidgets::ZOOM_IN => Some("ZOOM_IN".to_string()),
            ToolbarWidgets::ZOOM_OUT => Some("ZOOM_OUT".to_string()),
            ToolbarWidgets::BUILD_RAIL => Some("BUILD_RAIL".to_string()),
            ToolbarWidgets::BUILD_ROAD => Some("BUILD_ROAD".to_string()),
            ToolbarWidgets::BUILD_WATER => Some("BUILD_WATER".to_string()),
            ToolbarWidgets::BUILD_AIRPORT => Some("BUILD_AIRPORT".to_string()),
            ToolbarWidgets::MUSIC => Some("OPEN_MUSIC".to_string()),
            ToolbarWidgets::NEWS => Some("SHOW_NEWS".to_string()),
            ToolbarWidgets::HELP => Some("SHOW_HELP".to_string()),
            _ => None,
        }
    }
}

/// Show the toolbar window
pub fn show_toolbar(window_manager: &mut WindowManager, screen_width: u32) -> WindowID {
    let toolbar = ToolbarWindow::new(screen_width);
    window_manager.add_window(toolbar.into_window())
}
