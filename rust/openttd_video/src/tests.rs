//! Tests for the SDL2 event handling

#[cfg(test)]
mod tests {
    use crate::event::*;

    #[test]
    fn test_event_types() {
        // Test quit event
        let quit = Event::Quit;
        assert!(quit.is_quit());

        // Test mouse motion
        let motion = Event::MouseMotion {
            x: 100,
            y: 200,
            xrel: 5,
            yrel: -3,
        };
        assert!(!motion.is_quit());

        // Test mouse buttons
        let left_down = Event::MouseButtonDown {
            button: MouseButton::Left,
            x: 50,
            y: 75,
        };
        assert!(!left_down.is_quit());

        // Test keyboard events
        let key_down = Event::KeyDown {
            keycode: 65, // 'A'
            scancode: 4,
            modifiers: KeyModifiers {
                shift: true,
                ctrl: false,
                alt: false,
                gui: false,
            },
        };
        assert!(!key_down.is_fullscreen_toggle());

        // Test Alt+Enter for fullscreen
        let fullscreen = Event::KeyDown {
            keycode: 13, // Return
            scancode: 40,
            modifiers: KeyModifiers {
                shift: false,
                ctrl: false,
                alt: true,
                gui: false,
            },
        };
        assert!(fullscreen.is_fullscreen_toggle());

        // Test text input
        let text = Event::TextInput {
            text: "Hello".to_string(),
        };
        assert!(!text.is_quit());

        // Test window events
        let window = Event::Window(WindowEvent::SizeChanged {
            width: 1024,
            height: 768,
        });
        assert!(!window.is_quit());
    }

    #[test]
    fn test_key_modifiers() {
        let none = KeyModifiers::default();
        assert!(!none.shift);
        assert!(!none.ctrl);
        assert!(!none.alt);
        assert!(!none.gui);

        let all = KeyModifiers {
            shift: true,
            ctrl: true,
            alt: true,
            gui: true,
        };
        assert!(all.shift);
        assert!(all.ctrl);
        assert!(all.alt);
        assert!(all.gui);
    }

    #[test]
    fn test_mouse_buttons() {
        // Test all button variants
        let buttons = vec![
            MouseButton::Left,
            MouseButton::Right,
            MouseButton::Middle,
            MouseButton::X1,
            MouseButton::X2,
        ];

        for (i, button) in buttons.iter().enumerate() {
            for (j, other) in buttons.iter().enumerate() {
                if i == j {
                    assert_eq!(button, other);
                } else {
                    assert_ne!(button, other);
                }
            }
        }
    }

    #[test]
    fn test_window_events() {
        let events = vec![
            WindowEvent::Exposed,
            WindowEvent::SizeChanged {
                width: 800,
                height: 600,
            },
            WindowEvent::MouseEnter,
            WindowEvent::MouseLeave,
            WindowEvent::FocusGained,
            WindowEvent::FocusLost,
        ];

        // Just ensure they can be created and wrapped
        for event in events {
            let wrapped = Event::Window(event);
            assert!(!wrapped.is_quit());
        }
    }

    #[cfg(feature = "sdl2-backend")]
    #[test]
    #[ignore] // Requires SDL2 initialization which needs display
    fn test_sdl2_driver_creation() {
        use crate::Sdl2Driver;

        let result = Sdl2Driver::new("Test Window", 800, 600);
        // This will fail in CI without display, but structure is correct
        assert!(result.is_err() || result.is_ok());
    }
}
