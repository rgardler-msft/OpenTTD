//! Example demonstrating the OpenTTD GUI system with windows and widgets
//!
//! Note: This is a simplified example that demonstrates the GUI widget system
//! without actual rendering. In a real application, the GfxContext would be
//! integrated with the video driver.

use openttd_gfx::{Colour, Rect};
use openttd_gui::{ButtonWidget, ContainerWidget, LabelWidget, Window, WindowManager};
use sdl2::mouse::MouseButton;

fn create_main_menu_window() -> Window {
    let mut window = Window::new(
        1,
        "OpenTTD",
        Rect {
            x: 200,
            y: 100,
            width: 400,
            height: 400,
        },
    );

    // Create vertical container for menu items
    let mut main_container = ContainerWidget::new_vertical(100);

    // Add title label
    let title = Box::new(
        LabelWidget::new(101, "OPENTTD")
            .with_colour(Colour::rgba(255, 200, 100, 255))
            .with_alignment(openttd_gui::Alignment::Center),
    );
    main_container.add_child(title);

    // Add menu buttons
    let new_game_btn = Box::new(
        ButtonWidget::new(102, "New Game").with_callback(|| println!("New Game clicked!")),
    );
    main_container.add_child(new_game_btn);

    let load_game_btn = Box::new(
        ButtonWidget::new(103, "Load Game").with_callback(|| println!("Load Game clicked!")),
    );
    main_container.add_child(load_game_btn);

    let multiplayer_btn = Box::new(
        ButtonWidget::new(104, "Multiplayer").with_callback(|| println!("Multiplayer clicked!")),
    );
    main_container.add_child(multiplayer_btn);

    let scenario_btn = Box::new(
        ButtonWidget::new(105, "Scenario Editor")
            .with_callback(|| println!("Scenario Editor clicked!")),
    );
    main_container.add_child(scenario_btn);

    let options_btn =
        Box::new(ButtonWidget::new(106, "Options").with_callback(|| println!("Options clicked!")));
    main_container.add_child(options_btn);

    let quit_btn =
        Box::new(ButtonWidget::new(107, "Quit").with_callback(|| println!("Quit clicked!")));
    main_container.add_child(quit_btn);

    // Set the container as the window's root widget
    window.set_root_widget(Box::new(main_container));

    window
}

fn create_options_window() -> Window {
    let mut window = Window::new(
        2,
        "Options",
        Rect {
            x: 250,
            y: 200,
            width: 300,
            height: 150,
        },
    );
    window.modal = true;

    // Create vertical container
    let mut container = ContainerWidget::new_vertical(200);

    // Add some option widgets
    let label = Box::new(LabelWidget::new(201, "Game Options"));
    container.add_child(label);

    let mut button_container = ContainerWidget::new_horizontal(202);

    let ok_btn = Box::new(ButtonWidget::new(203, "OK").with_callback(|| println!("OK clicked!")));
    button_container.add_child(ok_btn);

    let cancel_btn =
        Box::new(ButtonWidget::new(204, "Cancel").with_callback(|| println!("Cancel clicked!")));
    button_container.add_child(cancel_btn);

    container.add_child(Box::new(button_container));

    window.set_root_widget(Box::new(container));
    window
}

fn main() {
    println!("OpenTTD GUI Demo");
    println!("================");
    println!();
    println!("This is a demonstration of the GUI widget system.");
    println!("In a real application, these would be rendered on screen.");
    println!();

    // Create window manager
    let mut window_manager = WindowManager::new();

    // Create and add main menu window
    let main_menu = create_main_menu_window();
    let main_id = window_manager.add_window(main_menu);
    println!("Created main menu window with ID: {:?}", main_id);

    // Create and add options window
    let options = create_options_window();
    let options_id = window_manager.add_window(options);
    println!("Created options window with ID: {:?}", options_id);

    // Simulate some interactions
    println!("\nSimulating mouse click at (300, 200):");
    let handled = window_manager.on_click(300, 200, MouseButton::Left);
    println!("  Click handled: {}", handled);

    println!("\nSimulating mouse move to (350, 250):");
    window_manager.on_mouse_move(350, 250);

    println!(
        "\nWindow manager has {} windows",
        window_manager.window_count()
    );

    // Note: In a real application, we would:
    // 1. Create a proper SDL2 window and get its handle
    // 2. Create a GfxContext from that window
    // 3. Run a game loop that:
    //    - Polls events from the video driver
    //    - Passes events to the window manager
    //    - Draws all windows using the GfxContext
    //    - Presents the rendered frame
}
