//! OpenTTD Main Executable
//!
//! This is the main entry point for the OpenTTD game.
//! Currently implements the main menu as a starting point for the Rust migration.

use openttd_gfx::GfxContext;
use openttd_gui::{create_main_menu_window, WindowManager};
use openttd_video::{event::Event, Sdl2Driver};
use sdl2::mouse::MouseButton;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("OpenTTD - Rust Migration");
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
    let main_menu = create_main_menu_window();
    window_manager.add_window(main_menu);

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
                    // Convert SDL mouse button to our mouse button type
                    let button = MouseButton::Left;

                    // Handle click and get the clicked widget ID from the active window
                    if window_manager.on_click(x, y, button) {
                        // Check if we need to get the widget ID that was clicked
                        // For now we'll handle this through the window manager's internal state
                        // In a real implementation, we'd need to track which widget was clicked
                    }
                }
                Event::MouseButtonUp { .. } => {
                    // Mouse button up events are handled in MouseButtonDown for now
                }
                Event::KeyDown { keycode, .. } => {
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

    println!("Shutting down...");
    Ok(())
}
