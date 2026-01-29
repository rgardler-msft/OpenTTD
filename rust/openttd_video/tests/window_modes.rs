//! Integration tests for window modes and resolution handling

#[cfg(all(test, feature = "sdl2-backend"))]
mod window_mode_tests {
    use openttd_video::sdl2_driver::WindowMode;
    use openttd_video::{Resolution, Sdl2Driver};

    #[test]
    fn test_resolution_struct() {
        let res1 = Resolution::new(1920, 1080);
        let res2 = Resolution::new(1920, 1080);
        let res3 = Resolution::new(1280, 720);

        assert_eq!(res1, res2);
        assert_ne!(res1, res3);
        assert_eq!(res1.width, 1920);
        assert_eq!(res1.height, 1080);
    }

    #[test]
    fn test_window_mode_enum() {
        assert_eq!(WindowMode::Windowed, WindowMode::Windowed);
        assert_ne!(WindowMode::Windowed, WindowMode::Fullscreen);
        assert_ne!(WindowMode::Fullscreen, WindowMode::FullscreenDesktop);
        assert_ne!(WindowMode::FullscreenDesktop, WindowMode::Windowed);
    }

    #[test]
    #[ignore] // Requires display/SDL2 - run with: cargo test -- --ignored
    fn test_driver_window_modes() {
        let mut driver =
            Sdl2Driver::new("Test Window Modes", 800, 600).expect("Failed to create SDL2 driver");

        // Test initial state
        assert_eq!(driver.get_window_mode(), WindowMode::Windowed);
        assert_eq!(driver.window_size(), (800, 600));

        // Test available resolutions
        let resolutions = driver.get_available_resolutions();
        assert!(
            !resolutions.is_empty(),
            "Should find at least one resolution"
        );
        println!("Found {} resolutions", resolutions.len());

        // Test resolution change in windowed mode
        if resolutions.len() > 1 {
            driver
                .change_resolution(1024, 768)
                .expect("Failed to change resolution");
            // Note: Actual size may vary slightly due to window manager
            let (w, h) = driver.window_size();
            assert!(
                w >= 1000 && h >= 700,
                "Window should be approximately 1024x768"
            );
        }

        // Test fullscreen toggle
        driver
            .toggle_fullscreen()
            .expect("Failed to toggle fullscreen");
        assert_ne!(driver.get_window_mode(), WindowMode::Windowed);

        // Toggle back to windowed
        driver
            .toggle_fullscreen()
            .expect("Failed to toggle back to windowed");
        assert_eq!(driver.get_window_mode(), WindowMode::Windowed);

        // Test explicit window mode setting
        driver
            .set_window_mode(WindowMode::FullscreenDesktop)
            .expect("Failed to set fullscreen desktop");
        assert_eq!(driver.get_window_mode(), WindowMode::FullscreenDesktop);

        driver
            .set_window_mode(WindowMode::Windowed)
            .expect("Failed to set windowed mode");
        assert_eq!(driver.get_window_mode(), WindowMode::Windowed);
    }

    #[test]
    #[ignore] // Requires display/SDL2
    fn test_resolution_finding() {
        let driver = Sdl2Driver::new("Test Resolution Finding", 800, 600)
            .expect("Failed to create SDL2 driver");

        // Test finding exact resolution
        let best = driver.find_best_fullscreen_resolution(800, 600);
        assert!(best.width > 0 && best.height > 0);

        // Test finding close resolution
        let best = driver.find_best_fullscreen_resolution(1920, 1080);
        assert!(best.width > 0 && best.height > 0);

        // Test with unusual resolution
        let best = driver.find_best_fullscreen_resolution(777, 555);
        assert!(best.width > 0 && best.height > 0);
        println!("Best match for 777x555: {}x{}", best.width, best.height);
    }

    #[test]
    #[ignore] // Requires display/SDL2
    fn test_display_info() {
        let driver =
            Sdl2Driver::new("Test Display Info", 800, 600).expect("Failed to create SDL2 driver");

        // Test getting display size
        if let Ok(display_size) = driver.get_display_size() {
            assert!(display_size.width > 0);
            assert!(display_size.height > 0);
            println!(
                "Display size: {}x{}",
                display_size.width, display_size.height
            );
        }
    }

    #[test]
    #[ignore] // Requires display/SDL2
    fn test_resize_handling() {
        let mut driver =
            Sdl2Driver::new("Test Resize", 800, 600).expect("Failed to create SDL2 driver");

        // Simulate resize event
        driver
            .handle_resize(1280, 720)
            .expect("Failed to handle resize");

        // In windowed mode, the stored size should update
        // (though actual window size depends on window manager)
        assert_eq!(driver.get_window_mode(), WindowMode::Windowed);
    }
}
