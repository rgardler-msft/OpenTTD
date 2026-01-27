use crate::{
    Alignment, ButtonWidget, ContainerWidget, LabelWidget, PanelWidget, Rect, Widget, WidgetID,
    Window, WindowID, WindowManager,
};
use openttd_gfx::Colour;
use std::collections::HashMap;

/// Widget IDs for the main menu
pub struct MainMenuWidgets;

impl MainMenuWidgets {
    pub const WINDOW: WindowID = 1000;
    pub const PANEL: WidgetID = 1001;
    pub const TITLE: WidgetID = 1002;
    pub const NEW_GAME: WidgetID = 1003;
    pub const LOAD_GAME: WidgetID = 1004;
    pub const PLAY_SCENARIO: WidgetID = 1005;
    pub const PLAY_HEIGHTMAP: WidgetID = 1006;
    pub const EDIT_SCENARIO: WidgetID = 1007;
    pub const MULTIPLAYER: WidgetID = 1008;
    pub const OPTIONS: WidgetID = 1009;
    pub const HIGHSCORE: WidgetID = 1010;
    pub const HELP: WidgetID = 1011;
    pub const CONTENT_DOWNLOAD: WidgetID = 1012;
    pub const EXIT: WidgetID = 1013;
    pub const VERSION_LABEL: WidgetID = 1014;
}

/// Main menu window with proper event handling
pub struct MainMenuWindow {
    window: Window,
    button_actions: HashMap<WidgetID, String>,
}

impl MainMenuWindow {
    pub fn new() -> Self {
        let mut window = Window::new(
            MainMenuWidgets::WINDOW,
            "OpenTTD",
            Rect::new(200, 100, 400, 520),
        );

        // Create main vertical container for all menu items
        let mut container = ContainerWidget::new_vertical(2000)
            .with_spacing(10)
            .with_padding(20);

        // Add title/logo area (placeholder for now)
        let mut title_label = LabelWidget::new(MainMenuWidgets::TITLE, "OpenTTD")
            .with_alignment(Alignment::Center)
            .with_colour(Colour::WHITE);
        title_label.set_rect(Rect::new(0, 0, 360, 30));
        container.add_child(Box::new(title_label));

        // Single Player section label
        let mut sp_label = LabelWidget::new(2001, "Single Player")
            .with_alignment(Alignment::Left)
            .with_colour(Colour::ui_highlight());
        sp_label.set_rect(Rect::new(0, 0, 360, 20));
        container.add_child(Box::new(sp_label));

        // New Game button
        let mut new_game = ButtonWidget::new(MainMenuWidgets::NEW_GAME, "New Game");
        new_game.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(new_game));

        // Load Game button
        let mut load_game = ButtonWidget::new(MainMenuWidgets::LOAD_GAME, "Load Game");
        load_game.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(load_game));

        // Play Scenario button
        let mut play_scenario = ButtonWidget::new(MainMenuWidgets::PLAY_SCENARIO, "Play Scenario");
        play_scenario.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(play_scenario));

        // Play Heightmap button
        let mut play_heightmap =
            ButtonWidget::new(MainMenuWidgets::PLAY_HEIGHTMAP, "Play Heightmap");
        play_heightmap.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(play_heightmap));

        // Scenario Editor button
        let mut edit_scenario =
            ButtonWidget::new(MainMenuWidgets::EDIT_SCENARIO, "Scenario Editor");
        edit_scenario.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(edit_scenario));

        // Highscore button
        let mut highscore = ButtonWidget::new(MainMenuWidgets::HIGHSCORE, "Highscore");
        highscore.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(highscore));

        // Add spacer
        let mut spacer1 = PanelWidget::new(2010);
        spacer1.set_rect(Rect::new(0, 0, 1, 20));
        spacer1.set_visible(false);
        container.add_child(Box::new(spacer1));

        // Multiplayer section label
        let mut mp_label = LabelWidget::new(2002, "Multiplayer")
            .with_alignment(Alignment::Left)
            .with_colour(Colour::ui_highlight());
        mp_label.set_rect(Rect::new(0, 0, 360, 20));
        container.add_child(Box::new(mp_label));

        // Multiplayer button
        let mut multiplayer = ButtonWidget::new(MainMenuWidgets::MULTIPLAYER, "Multiplayer");
        multiplayer.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(multiplayer));

        // Add spacer
        let mut spacer2 = PanelWidget::new(2011);
        spacer2.set_rect(Rect::new(0, 0, 1, 20));
        spacer2.set_visible(false);
        container.add_child(Box::new(spacer2));

        // Other section label
        let mut other_label = LabelWidget::new(2003, "Other")
            .with_alignment(Alignment::Left)
            .with_colour(Colour::ui_highlight());
        other_label.set_rect(Rect::new(0, 0, 360, 20));
        container.add_child(Box::new(other_label));

        // Options button
        let mut options = ButtonWidget::new(MainMenuWidgets::OPTIONS, "Game Options");
        options.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(options));

        // Content Download button
        let mut content_download =
            ButtonWidget::new(MainMenuWidgets::CONTENT_DOWNLOAD, "Check Online Content");
        content_download.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(content_download));

        // Help button
        let mut help = ButtonWidget::new(MainMenuWidgets::HELP, "Help & Manual");
        help.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(help));

        // Add spacer
        let mut spacer3 = PanelWidget::new(2012);
        spacer3.set_rect(Rect::new(0, 0, 1, 20));
        spacer3.set_visible(false);
        container.add_child(Box::new(spacer3));

        // Exit button
        let mut exit = ButtonWidget::new(MainMenuWidgets::EXIT, "Exit");
        exit.set_rect(Rect::new(0, 0, 340, 30));
        container.add_child(Box::new(exit));

        // Add spacer before version
        let mut spacer4 = PanelWidget::new(2013);
        spacer4.set_rect(Rect::new(0, 0, 1, 10));
        spacer4.set_visible(false);
        container.add_child(Box::new(spacer4));

        // Version label at the bottom
        let mut version_label = LabelWidget::new(MainMenuWidgets::VERSION_LABEL, "v0.1.0-rust")
            .with_alignment(Alignment::Center)
            .with_colour(Colour::GREY);
        version_label.set_rect(Rect::new(0, 0, 360, 15));
        container.add_child(Box::new(version_label));

        // Set the container as the root widget
        window.set_root_widget(Box::new(container));

        // Setup button action mappings
        let mut button_actions = HashMap::new();
        button_actions.insert(MainMenuWidgets::NEW_GAME, "NEW_GAME".to_string());
        button_actions.insert(MainMenuWidgets::LOAD_GAME, "LOAD_GAME".to_string());
        button_actions.insert(MainMenuWidgets::PLAY_SCENARIO, "PLAY_SCENARIO".to_string());
        button_actions.insert(
            MainMenuWidgets::PLAY_HEIGHTMAP,
            "PLAY_HEIGHTMAP".to_string(),
        );
        button_actions.insert(
            MainMenuWidgets::EDIT_SCENARIO,
            "SCENARIO_EDITOR".to_string(),
        );
        button_actions.insert(MainMenuWidgets::MULTIPLAYER, "MULTIPLAYER".to_string());
        button_actions.insert(MainMenuWidgets::OPTIONS, "OPTIONS".to_string());
        button_actions.insert(MainMenuWidgets::HIGHSCORE, "HIGHSCORE".to_string());
        button_actions.insert(MainMenuWidgets::HELP, "HELP".to_string());
        button_actions.insert(
            MainMenuWidgets::CONTENT_DOWNLOAD,
            "CONTENT_DOWNLOAD".to_string(),
        );
        button_actions.insert(MainMenuWidgets::EXIT, "EXIT_APPLICATION".to_string());

        Self {
            window,
            button_actions,
        }
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn into_window(self) -> Window {
        self.window
    }

    /// Check which button was clicked and return the action
    pub fn check_button_click(&self, x: i32, y: i32) -> Option<String> {
        // Check each button to see if it contains the point
        // This is a simplified approach - in a real implementation we'd traverse the widget tree

        // For now, we'll use hardcoded positions based on our layout
        // The window is at (200, 100) with width 400
        // Buttons start at y=100 (after title and label) with height 30 and spacing 10

        if !self.window.rect.contains_point(x, y) {
            return None;
        }

        // Translate to window coordinates
        let local_x = x - self.window.rect.x;
        let local_y = y - self.window.rect.y - 20; // Account for title bar

        // Check if within button x range (padding 20, button width 340)
        if local_x < 20 || local_x > 360 {
            return None;
        }

        // Calculate which button based on y position
        // Layout: title(30) + spacing(10) + label(20) = 60 pixels before first button
        // Then buttons are 30 pixels high with 10 pixel spacing

        let button_areas = vec![
            (60, 90, MainMenuWidgets::NEW_GAME),           // New Game
            (100, 130, MainMenuWidgets::LOAD_GAME),        // Load Game
            (140, 170, MainMenuWidgets::PLAY_SCENARIO),    // Play Scenario
            (180, 210, MainMenuWidgets::PLAY_HEIGHTMAP),   // Play Heightmap
            (220, 250, MainMenuWidgets::EDIT_SCENARIO),    // Scenario Editor
            (260, 290, MainMenuWidgets::HIGHSCORE),        // Highscore
            (320, 340, MainMenuWidgets::MULTIPLAYER),      // Multiplayer (after spacer and label)
            (380, 410, MainMenuWidgets::OPTIONS),          // Options (after spacer and label)
            (420, 450, MainMenuWidgets::CONTENT_DOWNLOAD), // Content Download
            (460, 490, MainMenuWidgets::HELP),             // Help
            (520, 550, MainMenuWidgets::EXIT),             // Exit (after spacer)
        ];

        for (y_min, y_max, widget_id) in button_areas {
            if local_y >= y_min && local_y <= y_max {
                if let Some(action) = self.button_actions.get(&widget_id) {
                    println!("Button clicked: {} -> {}", widget_id, action);
                    return Some(action.clone());
                }
            }
        }

        None
    }
}

/// Create and show the main menu window
pub fn show_main_menu(window_manager: &mut WindowManager) -> WindowID {
    let main_menu = MainMenuWindow::new();
    window_manager.add_window(main_menu.window)
}
