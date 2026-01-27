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
                                println!("New Game - not yet implemented");
                                // TODO: Implement new game functionality
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
