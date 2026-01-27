// Basic rendering example demonstrating the OpenTTD graphics library

use openttd_gfx::{ButtonState, Colour, GfxContext, Rect};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Create window
    let window = video_subsystem
        .window("OpenTTD Graphics Demo", 1024, 768)
        .position_centered()
        .resizable()
        .build()?;

    // Create graphics context
    let mut gfx = GfxContext::new(window)?;

    // Create event pump
    let mut event_pump = sdl_context.event_pump()?;

    println!("OpenTTD Graphics Demo - Rendering without text (TTF not available)");
    println!("Press ESC or click the bottom button to quit");

    // Button definitions
    struct Button {
        rect: Rect,
        label: &'static str,
        state: ButtonState,
    }

    let mut buttons = vec![
        Button {
            rect: Rect::new(50, 100, 200, 50),
            label: "New Game",
            state: ButtonState::Normal,
        },
        Button {
            rect: Rect::new(50, 170, 200, 50),
            label: "Load Game",
            state: ButtonState::Normal,
        },
        Button {
            rect: Rect::new(50, 240, 200, 50),
            label: "Settings",
            state: ButtonState::Normal,
        },
        Button {
            rect: Rect::new(50, 310, 200, 50),
            label: "Quit",
            state: ButtonState::Normal,
        },
    ];

    // Animation variables
    let mut animation_time = 0.0f32;
    let mut last_frame = Instant::now();
    let mut mouse_x = 0i32;
    let mut mouse_y = 0i32;
    let mut mouse_pressed = false;

    'running: loop {
        // Calculate delta time for animations
        let now = Instant::now();
        let delta = now.duration_since(last_frame).as_secs_f32();
        last_frame = now;
        animation_time += delta;

        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseMotion { x, y, .. } => {
                    mouse_x = x;
                    mouse_y = y;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_pressed = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    mouse_pressed = false;

                    // Check if quit button was clicked
                    if buttons[3].rect.contains_point(mouse_x, mouse_y) {
                        break 'running;
                    }
                }
                _ => {}
            }
        }

        // Update button states based on mouse position
        for button in &mut buttons {
            if button.rect.contains_point(mouse_x, mouse_y) {
                button.state = if mouse_pressed {
                    ButtonState::Pressed
                } else {
                    ButtonState::Hover
                };
            } else {
                button.state = ButtonState::Normal;
            }
        }

        // Clear screen with dark background
        gfx.clear(Colour::UI_BACKGROUND);

        // Draw header panel
        gfx.fill_rect(Rect::new(0, 0, 1024, 60), Colour::rgb(0x1A, 0x1A, 0x1A))?;

        // Draw decorative animated rectangles
        let pulse = (animation_time * 2.0).sin() * 0.5 + 0.5;
        let color_val = (pulse * 100.0 + 50.0) as u8;

        // Left panel background
        gfx.fill_rect(Rect::new(30, 80, 240, 300), Colour::rgb(0x30, 0x30, 0x30))?;
        gfx.draw_rect(
            Rect::new(30, 80, 240, 300),
            Colour::rgb(color_val, color_val, color_val),
        )?;

        // Draw buttons (without text since TTF is not available)
        for (i, button) in buttons.iter().enumerate() {
            gfx.draw_button(button.rect, button.label, button.state, None)?;

            // Draw a simple indicator for which button it is
            let indicator_size = 10;
            let indicator_x = button.rect.x + 10;
            let indicator_y = button.rect.y + (button.rect.height as i32 - indicator_size) / 2;

            // Different patterns for different buttons
            match i {
                0 => {
                    // New Game - single square
                    gfx.fill_rect(
                        Rect::new(
                            indicator_x,
                            indicator_y,
                            indicator_size as u32,
                            indicator_size as u32,
                        ),
                        Colour::GREEN,
                    )?;
                }
                1 => {
                    // Load Game - two squares
                    gfx.fill_rect(
                        Rect::new(
                            indicator_x,
                            indicator_y,
                            indicator_size as u32,
                            indicator_size as u32,
                        ),
                        Colour::BLUE,
                    )?;
                    gfx.fill_rect(
                        Rect::new(
                            indicator_x + 15,
                            indicator_y,
                            indicator_size as u32,
                            indicator_size as u32,
                        ),
                        Colour::BLUE,
                    )?;
                }
                2 => {
                    // Settings - three squares
                    for j in 0..3 {
                        gfx.fill_rect(
                            Rect::new(
                                indicator_x + j * 15,
                                indicator_y,
                                indicator_size as u32,
                                indicator_size as u32,
                            ),
                            Colour::WHITE,
                        )?;
                    }
                }
                3 => {
                    // Quit - X pattern
                    gfx.draw_line(
                        indicator_x,
                        indicator_y,
                        indicator_x + indicator_size,
                        indicator_y + indicator_size,
                        Colour::RED,
                    )?;
                    gfx.draw_line(
                        indicator_x + indicator_size,
                        indicator_y,
                        indicator_x,
                        indicator_y + indicator_size,
                        Colour::RED,
                    )?;
                }
                _ => {}
            }
        }

        // Draw info panel on the right
        gfx.fill_rect(Rect::new(300, 100, 400, 260), Colour::rgb(0x25, 0x25, 0x25))?;
        gfx.draw_rect(Rect::new(300, 100, 400, 260), Colour::GREY)?;

        // Draw some status indicators using shapes
        // Mouse position indicator
        let mouse_indicator = Rect::new(310, 140, 10, 10);
        gfx.fill_rect(mouse_indicator, Colour::GREEN)?;

        // Animation indicator (pulsing)
        let anim_color = (pulse * 255.0) as u8;
        gfx.fill_rect(
            Rect::new(310, 170, 10, 10),
            Colour::rgb(anim_color, anim_color, 0),
        )?;

        // FPS indicator (color based on performance)
        let fps = 1.0 / delta.max(0.001);
        let fps_color = if fps > 50.0 {
            Colour::GREEN
        } else if fps > 30.0 {
            Colour::rgb(255, 255, 0) // Yellow
        } else {
            Colour::RED
        };
        gfx.fill_rect(Rect::new(310, 200, 10, 10), fps_color)?;

        // Draw some sample geometric shapes
        let shapes_y = 400;

        // Filled rectangles with different colors
        gfx.fill_rect(Rect::new(50, shapes_y, 50, 50), Colour::RED)?;
        gfx.fill_rect(Rect::new(110, shapes_y, 50, 50), Colour::GREEN)?;
        gfx.fill_rect(Rect::new(170, shapes_y, 50, 50), Colour::BLUE)?;

        // Outlined rectangles
        gfx.draw_rect(Rect::new(50, shapes_y + 60, 50, 50), Colour::WHITE)?;
        gfx.draw_rect(Rect::new(110, shapes_y + 60, 50, 50), Colour::WHITE)?;
        gfx.draw_rect(Rect::new(170, shapes_y + 60, 50, 50), Colour::WHITE)?;

        // Draw lines to form a triangle
        let triangle_x = 300;
        let triangle_y = shapes_y + 30;
        gfx.draw_line(
            triangle_x,
            triangle_y,
            triangle_x + 50,
            triangle_y + 80,
            Colour::WHITE,
        )?;
        gfx.draw_line(
            triangle_x + 50,
            triangle_y + 80,
            triangle_x + 100,
            triangle_y,
            Colour::WHITE,
        )?;
        gfx.draw_line(
            triangle_x + 100,
            triangle_y,
            triangle_x,
            triangle_y,
            Colour::WHITE,
        )?;

        // Draw gradient bars
        for i in 0..10 {
            let intensity = (i * 25) as u8;
            gfx.fill_rect(
                Rect::new(450 + i * 25, shapes_y, 20, 80),
                Colour::rgb(intensity, intensity, intensity),
            )?;
        }

        // Present the frame
        gfx.present();

        // Frame rate limiting (approximately 60 FPS)
        std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())
}
