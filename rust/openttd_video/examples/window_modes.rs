//! Example demonstrating window modes and resolution handling
//!
//! Run with: cargo run --example window_modes -p openttd_video

use openttd_video::sdl2_driver::WindowMode;
use openttd_video::{Event, Sdl2Driver, WindowEvent};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    println!("OpenTTD Window Modes Example");
    println!("============================");
    println!("Controls:");
    println!("  ESC          - Quit");
    println!("  Alt+Enter    - Toggle fullscreen");
    println!("  F            - Fullscreen (real)");
    println!("  D            - Fullscreen (desktop)");
    println!("  W            - Windowed mode");
    println!("  R            - List resolutions");
    println!("  1-9          - Change to preset resolution");
    println!("  Resize window - Test resize handling");
    println!();

    // Create SDL2 driver with initial window
    let mut driver = Sdl2Driver::new("OpenTTD Window Modes Test", 1024, 768)?;

    println!("Window created at 1024x768");
    println!("Available resolutions:");
    let resolutions = driver.get_available_resolutions().to_vec(); // Clone the resolutions
    for (i, res) in resolutions.iter().take(9).enumerate() {
        println!("  {} - {}x{}", i + 1, res.width, res.height);
    }
    println!();

    // Display current display size
    if let Ok(display_size) = driver.get_display_size() {
        println!(
            "Current display: {}x{}",
            display_size.width, display_size.height
        );
    }

    println!("Starting event loop...");
    println!();

    let mut running = true;
    let mut event_count = 0;

    while running {
        // Poll for events
        if let Some(event) = driver.poll_event() {
            event_count += 1;

            match event {
                Event::Quit => {
                    println!("Quit event received");
                    running = false;
                }

                Event::KeyDown {
                    keycode,
                    modifiers: _,
                    ..
                } => {
                    // Check for Alt+Enter
                    if event.is_fullscreen_toggle() {
                        println!("[{}] Toggling fullscreen", event_count);
                        driver.toggle_fullscreen()?;
                        let mode = driver.get_window_mode();
                        let (w, h) = driver.window_size();
                        println!("  -> Mode: {:?}, Size: {}x{}", mode, w, h);
                    }

                    // Check for ESC
                    if keycode == 27 {
                        println!("[{}] ESC pressed, quitting", event_count);
                        running = false;
                    }

                    // Window mode controls (using ASCII codes)
                    match keycode {
                        102 | 70 => {
                            // F key
                            println!("[{}] Setting fullscreen mode", event_count);
                            driver.set_window_mode(WindowMode::Fullscreen)?;
                            let (w, h) = driver.window_size();
                            println!("  -> Fullscreen at {}x{}", w, h);
                        }
                        100 | 68 => {
                            // D key
                            println!("[{}] Setting desktop fullscreen mode", event_count);
                            driver.set_window_mode(WindowMode::FullscreenDesktop)?;
                            let (w, h) = driver.window_size();
                            println!("  -> Desktop fullscreen at {}x{}", w, h);
                        }
                        119 | 87 => {
                            // W key
                            println!("[{}] Setting windowed mode", event_count);
                            driver.set_window_mode(WindowMode::Windowed)?;
                            let (w, h) = driver.window_size();
                            println!("  -> Windowed at {}x{}", w, h);
                        }
                        114 | 82 => {
                            // R key
                            println!("[{}] Available resolutions:", event_count);
                            let resolutions = driver.get_available_resolutions();
                            for (i, res) in resolutions.iter().take(9).enumerate() {
                                println!("  {} - {}x{}", i + 1, res.width, res.height);
                            }
                        }
                        49..=57 => {
                            // Number keys 1-9
                            let index = (keycode - 49) as usize;
                            let resolutions = driver.get_available_resolutions().to_vec(); // Clone to avoid borrow issues
                            if index < resolutions.len() && index < 9 {
                                let res = &resolutions[index];
                                println!(
                                    "[{}] Changing resolution to {}x{}",
                                    event_count, res.width, res.height
                                );
                                if let Err(e) = driver.change_resolution(res.width, res.height) {
                                    eprintln!("  -> Failed: {}", e);
                                } else {
                                    let (w, h) = driver.window_size();
                                    println!("  -> Resolution changed to {}x{}", w, h);
                                }
                            }
                        }
                        _ => {}
                    }
                }

                Event::Window(win_event) => match win_event {
                    WindowEvent::SizeChanged { width, height } => {
                        println!("[{}] Window resized: {}x{}", event_count, width, height);
                        driver.handle_resize(width, height)?;

                        // Show best fullscreen resolution for new size
                        let best = driver.find_best_fullscreen_resolution(width, height);
                        println!("  -> Best fullscreen match: {}x{}", best.width, best.height);
                    }
                    WindowEvent::Exposed => {
                        println!("[{}] Window exposed (needs redraw)", event_count);
                    }
                    WindowEvent::MouseEnter => {
                        println!("[{}] Mouse entered window", event_count);
                    }
                    WindowEvent::MouseLeave => {
                        println!("[{}] Mouse left window", event_count);
                    }
                    WindowEvent::FocusGained => {
                        println!("[{}] Window gained focus", event_count);
                    }
                    WindowEvent::FocusLost => {
                        println!("[{}] Window lost focus", event_count);
                    }
                },

                _ => {
                    // Ignore other events for this example
                }
            }
        }

        // Small sleep to avoid busy-waiting
        std::thread::sleep(Duration::from_millis(10));
    }

    println!();
    println!(
        "Window modes test completed. Events processed: {}",
        event_count
    );

    Ok(())
}
