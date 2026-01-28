//! OpenTTD Main Executable
//!
//! This is the main entry point for the OpenTTD game.
//! Currently implements the main menu as a starting point for the Rust migration.

use openttd_gfx::GfxContext;
use openttd_gui::{MainMenuWindow, WindowManager};
use openttd_video::{event::Event, Sdl2Driver};
use sdl2::mouse::MouseButton;
use std::time::Duration;

/// Handle main menu button clicks and return the action to perform
fn handle_main_menu_action(window_manager: &mut WindowManager, x: i32, y: i32) -> Option<String> {
    // Check if main menu window is present
    if window_manager
        .get_window(openttd_gui::MainMenuWidgets::WINDOW)
        .is_none()
    {
        return None;
    }

    // Use the original main menu click handler for now
    // In the future we'll refactor to have better event handling

    // Check which widget was clicked based on position
    // This is a temporary solution until we have proper widget click detection

    let main_menu_rect = openttd_gfx::Rect::new(200, 100, 400, 520);
    if !main_menu_rect.contains_point(x, y) {
        return None;
    }

    // Translate to window coordinates
    let local_x = x - main_menu_rect.x;
    let local_y = y - main_menu_rect.y - 20; // Account for title bar

    // Check if within button x range (padding 20, button width 340)
    if local_x < 20 || local_x > 360 {
        return None;
    }

    // Calculate which button based on y position
    // Layout: title(30) + spacing(10) + label(20) = 60 pixels before first button
    // Then buttons are 30 pixels high with 10 pixel spacing

    let button_areas = vec![
        (60, 90, "NEW_GAME"),           // New Game
        (100, 130, "LOAD_GAME"),        // Load Game
        (140, 170, "PLAY_SCENARIO"),    // Play Scenario
        (180, 210, "PLAY_HEIGHTMAP"),   // Play Heightmap
        (220, 250, "SCENARIO_EDITOR"),  // Scenario Editor
        (260, 290, "HIGHSCORE"),        // Highscore
        (320, 350, "MULTIPLAYER"),      // Multiplayer (after spacer and label)
        (390, 420, "OPTIONS"),          // Options (after spacer and label)
        (430, 460, "CONTENT_DOWNLOAD"), // Content Download
        (470, 500, "HELP"),             // Help
        (530, 560, "EXIT_APPLICATION"), // Exit (after spacer)
    ];

    for (y_min, y_max, action) in button_areas {
        if local_y >= y_min && local_y <= y_max {
            println!("Main menu button clicked: {}", action);
            return Some(action.to_string());
        }
    }

    None
}

/// Handle world generation window button clicks and return the action to perform
fn handle_world_gen_action(window_manager: &mut WindowManager, x: i32, y: i32) -> Option<String> {
    // Check if world gen window is present
    if window_manager
        .get_window(openttd_gui::WORLD_GEN_WINDOW_ID)
        .is_none()
    {
        return None;
    }

    // World gen window is centered at (100, 50, 600, 500)
    let world_gen_rect = openttd_gfx::Rect::new(100, 50, 600, 500);
    if !world_gen_rect.contains_point(x, y) {
        return None;
    }

    // Translate to window coordinates
    let local_x = x - world_gen_rect.x;
    let local_y = y - world_gen_rect.y - 20; // Account for title bar

    // Check for Generate and Cancel buttons at bottom (470, 200/300, 80x30)
    if local_y >= 450 && local_y <= 480 {
        if local_x >= 180 && local_x <= 260 {
            // Generate button
            println!("Generate button clicked");
            return Some("GENERATE_WORLD".to_string());
        } else if local_x >= 280 && local_x <= 360 {
            // Cancel button
            println!("Cancel button clicked");
            return Some("CANCEL_WORLD_GEN".to_string());
        }
    }

    // Check for climate buttons (y: 30-60, x: 20/170/320/470)
    if local_y >= 30 && local_y <= 60 {
        if local_x >= 20 && local_x <= 160 {
            println!("Temperate climate selected");
            return Some("CLIMATE_TEMPERATE".to_string());
        } else if local_x >= 170 && local_x <= 310 {
            println!("Arctic climate selected");
            return Some("CLIMATE_ARCTIC".to_string());
        } else if local_x >= 320 && local_x <= 460 {
            println!("Tropical climate selected");
            return Some("CLIMATE_TROPICAL".to_string());
        } else if local_x >= 470 && local_x <= 560 {
            println!("Toyland climate selected");
            return Some("CLIMATE_TOYLAND".to_string());
        }
    }

    // Check for map size, terrain type, and other dropdown button clicks
    // For now, these will cycle through options when clicked

    // Map size X button (y: 100-130, x: 180-260)
    if local_y >= 100 && local_y <= 130 && local_x >= 180 && local_x <= 260 {
        println!("Map size X button clicked - cycling through sizes");
        return Some("CYCLE_MAP_SIZE_X".to_string());
    }

    // Map size Y button (y: 100-130, x: 430-510)
    if local_y >= 100 && local_y <= 130 && local_x >= 430 && local_x <= 510 {
        println!("Map size Y button clicked - cycling through sizes");
        return Some("CYCLE_MAP_SIZE_Y".to_string());
    }

    // Terrain type button (y: 150-180, x: 180-360)
    if local_y >= 150 && local_y <= 180 && local_x >= 180 && local_x <= 360 {
        println!("Terrain type button clicked - cycling through types");
        return Some("CYCLE_TERRAIN_TYPE".to_string());
    }

    // Sea level button (y: 200-230, x: 180-360)
    if local_y >= 200 && local_y <= 230 && local_x >= 180 && local_x <= 360 {
        println!("Sea level button clicked - cycling through levels");
        return Some("CYCLE_SEA_LEVEL".to_string());
    }

    // Year selector buttons (y: 350-380)
    if local_y >= 350 && local_y <= 380 {
        // Down button (x: 180-210)
        if local_x >= 180 && local_x <= 210 {
            println!("Year down button clicked");
            return Some("YEAR_DOWN".to_string());
        }
        // Up button (x: 290-320)
        else if local_x >= 290 && local_x <= 320 {
            println!("Year up button clicked");
            return Some("YEAR_UP".to_string());
        }
    }

    None
}

/// Handle toolbar button clicks and return the action to perform
fn handle_toolbar_action(window_manager: &mut WindowManager, x: i32, y: i32) -> Option<String> {
    // Check if toolbar window is present
    if window_manager
        .get_window(openttd_gui::TOOLBAR_WINDOW_ID)
        .is_none()
    {
        return None;
    }

    // Toolbar is at top of screen (0, 0, screen_width, 35)
    let toolbar_rect = openttd_gfx::Rect::new(0, 0, 800, 35);
    if !toolbar_rect.contains_point(x, y) {
        return None;
    }

    // Calculate which button was clicked
    // Buttons are 30x30 with 5px spacing, starting at x=5
    let button_x = x - 5;
    if button_x < 0 || y < 2 || y > 32 {
        return None;
    }

    // Each button is 30px wide with 5px spacing = 35px per button
    let button_index = button_x / 35;

    // Map button index to action
    let actions = vec![
        "PAUSE",          // 0: Pause
        "FAST_FORWARD",   // 1: Fast Forward
        "SAVE",           // 2: Save
        "LOAD",           // 3: Load
        "ZOOM_IN",        // 4: Zoom In
        "ZOOM_OUT",       // 5: Zoom Out
        "WORLD_MAP",      // 6: World Map
        "TOWN_DIRECTORY", // 7: Town Directory
        "SUBSIDIES",      // 8: Subsidies
        "STATION_LIST",   // 9: Station List
        "FINANCES",       // 10: Finances
        "COMPANY_INFO",   // 11: Company Info
        "GRAPH",          // 12: Graphs
        "LEAGUE",         // 13: League Table
        "NEWS",           // 14: News
        "MESSAGES",       // 15: Messages
        "SETTINGS",       // 16: Settings
        "HELP_TOOLBAR",   // 17: Help
        "MAIN_MENU",      // 18: Back to Main Menu
    ];

    if button_index >= 0 && (button_index as usize) < actions.len() {
        let action = actions[button_index as usize];
        println!("Toolbar button clicked: {}", action);
        return Some(action.to_string());
    }

    None
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("OpenTTD v0.1.0-rust");
    println!("Starting main menu...");

    // Initialize SDL2 video driver
    let mut driver = Sdl2Driver::new("OpenTTD", 800, 600)?;

    // Get SDL context for future use (e.g., TTF initialization)
    let _sdl_context = driver.sdl_context().clone();

    // Take the window from the driver to create GfxContext
    // Note: After this, the driver can only be used for event handling
    let window = driver
        .take_window()
        .ok_or("Failed to get window from driver")?;

    // Create graphics context
    let mut gfx = GfxContext::new(window)?;

    // Create window manager
    let mut window_manager = WindowManager::new();

    // Create and add main menu window
    let main_menu = MainMenuWindow::new();
    window_manager.add_window(main_menu.into_window());

    // Main game loop
    let mut running = true;
    let mut last_frame = std::time::Instant::now();

    while running {
        // Calculate delta time
        let now = std::time::Instant::now();
        let _delta = now.duration_since(last_frame);
        last_frame = now;

        // Handle events
        while let Some(event) = driver.poll_event() {
            match event {
                Event::Quit => {
                    println!("Quit event received");
                    running = false;
                }
                Event::MouseMotion { x, y, .. } => {
                    window_manager.on_mouse_move(x, y);
                }
                Event::MouseButtonDown { x, y, .. } => {
                    // Check if any special windows are open and close them
                    if window_manager
                        .get_window(openttd_gui::HIGHSCORE_WINDOW_ID)
                        .is_some()
                    {
                        let _ = window_manager.remove_window(openttd_gui::HIGHSCORE_WINDOW_ID);
                        continue;
                    }

                    if window_manager
                        .get_window(openttd_gui::DATE_SELECTOR_WINDOW_ID)
                        .is_some()
                    {
                        let _ = window_manager.remove_window(openttd_gui::DATE_SELECTOR_WINDOW_ID);
                        continue;
                    }

                    if window_manager
                        .get_window(openttd_gui::LEAGUE_WINDOW_ID)
                        .is_some()
                    {
                        let _ = window_manager.remove_window(openttd_gui::LEAGUE_WINDOW_ID);
                        continue;
                    }

                    // Check for world generation window button clicks
                    if let Some(action) = handle_world_gen_action(&mut window_manager, x, y) {
                        match action.as_str() {
                            "GENERATE_WORLD" => {
                                println!("Generating world... (not yet implemented)");
                                // For now, just close the window and show toolbar
                                let _ =
                                    window_manager.remove_window(openttd_gui::WORLD_GEN_WINDOW_ID);
                                openttd_gui::show_toolbar(&mut window_manager, 800);
                            }
                            "CANCEL_WORLD_GEN" => {
                                println!("Canceling world generation");
                                // Close world gen window and return to main menu
                                let _ =
                                    window_manager.remove_window(openttd_gui::WORLD_GEN_WINDOW_ID);
                                if let Some(main_menu) =
                                    window_manager.get_window(openttd_gui::MainMenuWidgets::WINDOW)
                                {
                                    main_menu.visible = true;
                                }
                            }
                            action if action.starts_with("CLIMATE_") => {
                                println!("Climate selection: {}", action);
                                // TODO: Update the world gen config with selected climate
                            }
                            action if action.starts_with("CYCLE_") => {
                                println!("Cycling dropdown: {}", action);
                                // TODO: Update the dropdown display with next value in sequence
                            }
                            "YEAR_UP" => {
                                println!("Incrementing start year");
                                // TODO: Increase year value and update display
                            }
                            "YEAR_DOWN" => {
                                println!("Decrementing start year");
                                // TODO: Decrease year value and update display
                            }
                            _ => {
                                println!("World gen action not yet implemented: {}", action);
                            }
                        }
                        continue;
                    }

                    // Check for toolbar button clicks first (if toolbar is visible)
                    if let Some(action) = handle_toolbar_action(&mut window_manager, x, y) {
                        match action.as_str() {
                            "MAIN_MENU" => {
                                println!("Returning to main menu");
                                // Hide toolbar
                                let _ =
                                    window_manager.remove_window(openttd_gui::TOOLBAR_WINDOW_ID);
                                // Show main menu
                                if let Some(main_menu) =
                                    window_manager.get_window(openttd_gui::MainMenuWidgets::WINDOW)
                                {
                                    main_menu.visible = true;
                                }
                            }
                            "LEAGUE" => {
                                println!("Opening league table from toolbar");
                                openttd_gui::show_league_table(&mut window_manager);
                            }
                            "PAUSE" => {
                                println!("Game paused (not yet implemented)");
                            }
                            "FAST_FORWARD" => {
                                println!("Fast forward (not yet implemented)");
                            }
                            "SAVE" => {
                                println!("Save game (not yet implemented)");
                            }
                            "LOAD" => {
                                println!("Load game (not yet implemented)");
                            }
                            "SETTINGS" => {
                                println!("Settings (not yet implemented)");
                            }
                            _ => {
                                println!("Toolbar action not yet implemented: {}", action);
                            }
                        }
                        continue;
                    }

                    // Check for main menu button clicks
                    if let Some(action) = handle_main_menu_action(&mut window_manager, x, y) {
                        match action.as_str() {
                            "EXIT_APPLICATION" => {
                                println!("Exit button clicked - shutting down");
                                running = false;
                            }
                            "HIGHSCORE" => {
                                println!("Opening highscore window");
                                openttd_gui::show_highscore_table(
                                    &mut window_manager,
                                    openttd_gui::DifficultyLevel::Custom,
                                    None,
                                );
                            }
                            "NEW_GAME" => {
                                println!("Starting new game - showing world generation window");
                                // Hide main menu
                                if let Some(main_menu) =
                                    window_manager.get_window(openttd_gui::MainMenuWidgets::WINDOW)
                                {
                                    main_menu.visible = false;
                                }
                                // Show world generation window
                                openttd_gui::show_world_gen(&mut window_manager);
                            }
                            "LOAD_GAME" => {
                                println!("Load Game - not yet implemented");
                                // TODO: Implement load game dialog
                            }
                            "PLAY_SCENARIO" => {
                                println!("Opening date selector (test)");
                                openttd_gui::show_date_selector(
                                    &mut window_manager,
                                    openttd_gui::GameDate::new(1, 1, 1950),
                                    1900,
                                    2100,
                                );
                            }
                            "PLAY_HEIGHTMAP" => {
                                println!("Opening league table (test)");
                                openttd_gui::show_league_table(&mut window_manager);
                            }
                            "SCENARIO_EDITOR" => {
                                println!("Scenario Editor - not yet implemented");
                                // TODO: Implement scenario editor
                            }
                            "MULTIPLAYER" => {
                                println!("Multiplayer - not yet implemented");
                                // TODO: Implement multiplayer browser
                            }
                            "OPTIONS" => {
                                println!("Options - not yet implemented");
                                // TODO: Implement options window
                            }
                            "HELP" => {
                                println!("Help - not yet implemented");
                                // TODO: Implement help window
                            }
                            "CONTENT_DOWNLOAD" => {
                                println!("Content Download - not yet implemented");
                                // TODO: Implement content download window
                            }
                            _ => {
                                println!("Unknown action: {}", action);
                            }
                        }
                    } else {
                        // Let window manager handle other clicks
                        let button = MouseButton::Left;
                        let _ = window_manager.on_click(x, y, button);
                    }
                }
                Event::MouseButtonUp { .. } => {
                    // Mouse button up events are handled in MouseButtonDown for now
                }
                Event::KeyDown { keycode, .. } => {
                    // Close any open dialog windows on ESC or any keypress
                    if window_manager
                        .get_window(openttd_gui::HIGHSCORE_WINDOW_ID)
                        .is_some()
                    {
                        let _ = window_manager.remove_window(openttd_gui::HIGHSCORE_WINDOW_ID);
                        continue;
                    }

                    if window_manager
                        .get_window(openttd_gui::DATE_SELECTOR_WINDOW_ID)
                        .is_some()
                    {
                        let _ = window_manager.remove_window(openttd_gui::DATE_SELECTOR_WINDOW_ID);
                        continue;
                    }

                    if window_manager
                        .get_window(openttd_gui::LEAGUE_WINDOW_ID)
                        .is_some()
                    {
                        let _ = window_manager.remove_window(openttd_gui::LEAGUE_WINDOW_ID);
                        continue;
                    }

                    if window_manager
                        .get_window(openttd_gui::WORLD_GEN_WINDOW_ID)
                        .is_some()
                    {
                        let _ = window_manager.remove_window(openttd_gui::WORLD_GEN_WINDOW_ID);
                        // Return to main menu
                        if let Some(main_menu) =
                            window_manager.get_window(openttd_gui::MainMenuWidgets::WINDOW)
                        {
                            main_menu.visible = true;
                        }
                        continue;
                    }

                    // ESC key to quit
                    if keycode == 27 {
                        println!("ESC pressed, exiting");
                        running = false;
                    }
                }
                _ => {}
            }
        }

        // Clear screen
        gfx.clear(openttd_gfx::Colour::BLACK);

        // Draw all windows
        window_manager.draw_all(&mut gfx);

        // Present frame
        gfx.present();

        // Frame rate limiting (60 FPS)
        std::thread::sleep(Duration::from_millis(16));
    }

    println!("Thank you for playing OpenTTD!");
    Ok(())
}
