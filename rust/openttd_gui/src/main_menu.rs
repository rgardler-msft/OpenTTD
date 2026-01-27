use crate::{
    Alignment, ButtonWidget, ContainerWidget, LabelWidget, PanelWidget, Rect, Widget, WidgetID,
    Window, WindowID,
};
use openttd_gfx::Colour;

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
}

/// Create the main menu window
pub fn create_main_menu_window() -> Window {
    let mut window = Window::new(
        MainMenuWidgets::WINDOW,
        "OpenTTD",
        Rect::new(200, 100, 400, 500),
    );

    // Create main vertical container for all menu items
    let mut container = ContainerWidget::new_vertical(2000)
        .with_spacing(10)
        .with_padding(20);

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

    // Play Scenario button (temporarily testing Date Selector)
    let mut play_scenario =
        ButtonWidget::new(MainMenuWidgets::PLAY_SCENARIO, "Date Selector (Test)");
    play_scenario.set_rect(Rect::new(0, 0, 340, 30));
    container.add_child(Box::new(play_scenario));

    // Play Heightmap button (temporarily testing League Table)
    let mut play_heightmap =
        ButtonWidget::new(MainMenuWidgets::PLAY_HEIGHTMAP, "League Table (Test)");
    play_heightmap.set_rect(Rect::new(0, 0, 340, 30));
    container.add_child(Box::new(play_heightmap));

    // Scenario Editor button
    let mut edit_scenario = ButtonWidget::new(MainMenuWidgets::EDIT_SCENARIO, "Scenario Editor");
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

    // Set the container as the root widget
    window.set_root_widget(Box::new(container));

    window
}

/// Handle main menu button clicks
pub fn handle_main_menu_click(widget_id: WidgetID) -> Option<String> {
    match widget_id {
        MainMenuWidgets::NEW_GAME => {
            println!("New Game clicked");
            Some("NEW_GAME".to_string())
        }
        MainMenuWidgets::LOAD_GAME => {
            println!("Load Game clicked");
            Some("LOAD_GAME".to_string())
        }
        MainMenuWidgets::PLAY_SCENARIO => {
            println!("Date Selector test clicked");
            Some("DATE_SELECTOR_TEST".to_string())
        }
        MainMenuWidgets::PLAY_HEIGHTMAP => {
            println!("League Table test clicked");
            Some("LEAGUE_TABLE_TEST".to_string())
        }
        MainMenuWidgets::EDIT_SCENARIO => {
            println!("Scenario Editor clicked");
            Some("SCENARIO_EDITOR".to_string())
        }
        MainMenuWidgets::MULTIPLAYER => {
            println!("Multiplayer clicked");
            Some("MULTIPLAYER".to_string())
        }
        MainMenuWidgets::OPTIONS => {
            println!("Options clicked");
            Some("OPTIONS".to_string())
        }
        MainMenuWidgets::HIGHSCORE => {
            println!("Highscore clicked - opening highscore window");
            Some("HIGHSCORE".to_string())
        }
        MainMenuWidgets::HELP => {
            println!("Help clicked");
            Some("HELP".to_string())
        }
        MainMenuWidgets::CONTENT_DOWNLOAD => {
            println!("Content Download clicked");
            Some("CONTENT_DOWNLOAD".to_string())
        }
        MainMenuWidgets::EXIT => {
            println!("Exit clicked");
            Some("EXIT".to_string())
        }
        _ => None,
    }
}
