//! Example SDL2 event loop demonstration
//!
//! Run with: cargo run --example event_loop --features sdl2-backend

#[cfg(feature = "sdl2-backend")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use openttd_video::{Event, Sdl2Driver};
    use std::time::Duration;

    env_logger::init();

    println!("Creating SDL2 window...");
    let mut driver = Sdl2Driver::new("OpenTTD Event Loop Demo", 800, 600)?;

    println!("Window created. Press ESC or close window to quit.");
    println!("Press Alt+Enter to toggle fullscreen.");
    println!("\nEvent log:");

    let mut running = true;
    let mut event_count = 0;

    while running {
        // Poll for events
        while let Some(event) = driver.poll_event() {
            event_count += 1;

            match &event {
                Event::Quit => {
                    println!("[{}] Quit event received", event_count);
                    running = false;
                }
                Event::KeyDown {
                    keycode, modifiers, ..
                } => {
                    println!(
                        "[{}] Key down: keycode={}, shift={}, ctrl={}, alt={}",
                        event_count, keycode, modifiers.shift, modifiers.ctrl, modifiers.alt
                    );

                    // Check for ESC key (keycode 27)
                    if *keycode == 27 {
                        println!("[{}] ESC pressed, exiting...", event_count);
                        running = false;
                    }

                    // Check for fullscreen toggle
                    if event.is_fullscreen_toggle() {
                        println!("[{}] Toggling fullscreen...", event_count);
                        driver.toggle_fullscreen()?;
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    // Only log every 10th motion event to avoid spam
                    if event_count % 10 == 0 {
                        println!("[{}] Mouse at ({}, {})", event_count, x, y);
                    }
                }
                Event::MouseButtonDown { button, x, y } => {
                    println!(
                        "[{}] Mouse button {:?} down at ({}, {})",
                        event_count, button, x, y
                    );
                }
                Event::MouseButtonUp { button, x, y } => {
                    println!(
                        "[{}] Mouse button {:?} up at ({}, {})",
                        event_count, button, x, y
                    );
                }
                Event::MouseWheel { x, y } => {
                    println!("[{}] Mouse wheel: x={}, y={}", event_count, x, y);
                }
                Event::TextInput { text } => {
                    println!("[{}] Text input: \"{}\"", event_count, text);
                }
                Event::Window(window_event) => {
                    println!("[{}] Window event: {:?}", event_count, window_event);
                }
                _ => {
                    // Other events
                    println!("[{}] Event: {:?}", event_count, event);
                }
            }
        }

        // Small delay to prevent CPU spinning
        std::thread::sleep(Duration::from_millis(1));
    }

    println!("\nTotal events processed: {}", event_count);
    println!("Goodbye!");

    Ok(())
}

#[cfg(not(feature = "sdl2-backend"))]
fn main() {
    eprintln!("This example requires the sdl2-backend feature.");
    eprintln!("Run with: cargo run --example event_loop --features sdl2-backend");
}
