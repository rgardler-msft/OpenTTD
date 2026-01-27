//! SDL2 video driver implementation

#[cfg(feature = "sdl2-backend")]
pub mod sdl2_impl {
    use crate::event::{Event, KeyModifiers, MouseButton, Result, VideoError, WindowEvent};
    use log::{debug, info, warn};
    use sdl2::event::Event as SdlEvent;
    use sdl2::event::WindowEvent as SdlWindowEvent;
    use sdl2::mouse::MouseButton as SdlMouseButton;
    use sdl2::video::Window;
    use sdl2::EventPump;
    use std::time::Duration;

    /// SDL2 video driver
    pub struct Sdl2Driver {
        sdl_context: sdl2::Sdl,
        #[allow(dead_code)]
        video_subsystem: sdl2::VideoSubsystem,
        window: Window,
        event_pump: EventPump,
        cursor_visible: bool,
    }

    impl Sdl2Driver {
        /// Initialize SDL2 video driver
        pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
            info!("Initializing SDL2 video driver");

            let sdl_context = sdl2::init().map_err(|e| VideoError::InitFailed(e))?;

            let video_subsystem = sdl_context.video().map_err(|e| VideoError::InitFailed(e))?;

            let window = video_subsystem
                .window(title, width, height)
                .position_centered()
                .resizable()
                .build()
                .map_err(|e| VideoError::WindowCreationFailed(e.to_string()))?;

            let event_pump = sdl_context
                .event_pump()
                .map_err(|e| VideoError::InitFailed(e))?;

            Ok(Self {
                sdl_context,
                video_subsystem,
                window,
                event_pump,
                cursor_visible: true,
            })
        }

        /// Poll for next event
        pub fn poll_event(&mut self) -> Option<Event> {
            let sdl_event = self.event_pump.poll_event()?;
            self.convert_event(sdl_event)
        }

        /// Wait for next event with timeout
        pub fn wait_event_timeout(&mut self, timeout: Duration) -> Option<Event> {
            let sdl_event = self
                .event_pump
                .wait_event_timeout(timeout.as_millis() as u32)?;
            self.convert_event(sdl_event)
        }

        /// Convert SDL2 event to OpenTTD event
        fn convert_event(&self, event: SdlEvent) -> Option<Event> {
            match event {
                SdlEvent::Quit { .. } => {
                    debug!("Quit event received");
                    Some(Event::Quit)
                }

                SdlEvent::MouseMotion {
                    x, y, xrel, yrel, ..
                } => Some(Event::MouseMotion { x, y, xrel, yrel }),

                SdlEvent::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    let button = self.convert_mouse_button(mouse_btn)?;
                    Some(Event::MouseButtonDown { button, x, y })
                }

                SdlEvent::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    let button = self.convert_mouse_button(mouse_btn)?;
                    Some(Event::MouseButtonUp { button, x, y })
                }

                SdlEvent::MouseWheel { x, y, .. } => Some(Event::MouseWheel { x, y }),

                SdlEvent::KeyDown {
                    keycode,
                    scancode,
                    keymod,
                    ..
                } => {
                    let keycode = keycode.map(|k| k as u32).unwrap_or(0);
                    let scancode = scancode.map(|s| s as u32).unwrap_or(0);
                    let modifiers = self.convert_key_modifiers(keymod);

                    Some(Event::KeyDown {
                        keycode,
                        scancode,
                        modifiers,
                    })
                }

                SdlEvent::KeyUp {
                    keycode,
                    scancode,
                    keymod,
                    ..
                } => {
                    let keycode = keycode.map(|k| k as u32).unwrap_or(0);
                    let scancode = scancode.map(|s| s as u32).unwrap_or(0);
                    let modifiers = self.convert_key_modifiers(keymod);

                    Some(Event::KeyUp {
                        keycode,
                        scancode,
                        modifiers,
                    })
                }

                SdlEvent::TextInput { text, .. } => Some(Event::TextInput { text }),

                SdlEvent::Window { win_event, .. } => self.convert_window_event(win_event),

                _ => None,
            }
        }

        /// Convert SDL2 mouse button to internal representation
        fn convert_mouse_button(&self, button: SdlMouseButton) -> Option<MouseButton> {
            match button {
                SdlMouseButton::Left => Some(MouseButton::Left),
                SdlMouseButton::Right => Some(MouseButton::Right),
                SdlMouseButton::Middle => Some(MouseButton::Middle),
                SdlMouseButton::X1 => Some(MouseButton::X1),
                SdlMouseButton::X2 => Some(MouseButton::X2),
                _ => None,
            }
        }

        /// Convert SDL2 key modifiers to internal representation
        fn convert_key_modifiers(&self, keymod: sdl2::keyboard::Mod) -> KeyModifiers {
            use sdl2::keyboard::Mod;

            KeyModifiers {
                shift: keymod.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD),
                ctrl: keymod.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD),
                alt: keymod.intersects(Mod::LALTMOD | Mod::RALTMOD),
                gui: keymod.intersects(Mod::LGUIMOD | Mod::RGUIMOD),
            }
        }

        /// Convert SDL2 window event to internal representation
        fn convert_window_event(&self, event: SdlWindowEvent) -> Option<Event> {
            let window_event = match event {
                SdlWindowEvent::Exposed => WindowEvent::Exposed,
                SdlWindowEvent::SizeChanged(width, height) => WindowEvent::SizeChanged {
                    width: width as u32,
                    height: height as u32,
                },
                SdlWindowEvent::Enter => WindowEvent::MouseEnter,
                SdlWindowEvent::Leave => WindowEvent::MouseLeave,
                SdlWindowEvent::FocusGained => WindowEvent::FocusGained,
                SdlWindowEvent::FocusLost => WindowEvent::FocusLost,
                _ => return None,
            };

            Some(Event::Window(window_event))
        }

        /// Show or hide cursor
        pub fn set_cursor_visible(&mut self, visible: bool) {
            self.cursor_visible = visible;
            self.sdl_context.mouse().show_cursor(visible);
        }

        /// Toggle fullscreen mode
        pub fn toggle_fullscreen(&mut self) -> Result<()> {
            use sdl2::video::FullscreenType;

            let current = self.window.fullscreen_state();
            let new_state = if current == FullscreenType::Off {
                FullscreenType::Desktop
            } else {
                FullscreenType::Off
            };

            self.window
                .set_fullscreen(new_state)
                .map_err(|e| VideoError::EventError(e))?;

            info!("Toggled fullscreen: {:?} -> {:?}", current, new_state);
            Ok(())
        }

        /// Get window size
        pub fn window_size(&self) -> (u32, u32) {
            self.window.size()
        }

        /// Set window title
        pub fn set_title(&mut self, title: &str) {
            self.window
                .set_title(title)
                .unwrap_or_else(|e| warn!("Failed to set window title: {}", e));
        }
    }
}

#[cfg(feature = "sdl2-backend")]
pub use sdl2_impl::Sdl2Driver;
