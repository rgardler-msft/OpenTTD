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

    /// Available screen resolution
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Resolution {
        pub width: u32,
        pub height: u32,
    }

    impl Resolution {
        pub fn new(width: u32, height: u32) -> Self {
            Self { width, height }
        }
    }

    /// Window mode state
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum WindowMode {
        Windowed,
        Fullscreen,
        FullscreenDesktop,
    }

    /// SDL2 video driver
    pub struct Sdl2Driver {
        sdl_context: sdl2::Sdl,
        video_subsystem: sdl2::VideoSubsystem,
        window: Window,
        event_pump: EventPump,
        cursor_visible: bool,
        current_mode: WindowMode,
        windowed_size: Resolution,
        available_resolutions: Vec<Resolution>,
    }

    impl Sdl2Driver {
        /// Initialize SDL2 video driver
        pub fn new(title: &str, width: u32, height: u32) -> Result<Self> {
            info!("Initializing SDL2 video driver");

            let sdl_context = sdl2::init().map_err(|e| VideoError::InitFailed(e))?;

            let video_subsystem = sdl_context.video().map_err(|e| VideoError::InitFailed(e))?;

            // Find available resolutions
            let available_resolutions = Self::find_resolutions(&video_subsystem);
            info!("Found {} display resolutions", available_resolutions.len());

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
                current_mode: WindowMode::Windowed,
                windowed_size: Resolution::new(width, height),
                available_resolutions,
            })
        }

        /// Find available display resolutions
        fn find_resolutions(video: &sdl2::VideoSubsystem) -> Vec<Resolution> {
            let mut resolutions = Vec::new();

            // Get resolutions from all displays
            let num_displays = video.num_video_displays().unwrap_or(0);

            for display_index in 0..num_displays {
                if let Ok(num_modes) = video.num_display_modes(display_index) {
                    for mode_index in 0..num_modes {
                        if let Ok(mode) = video.display_mode(display_index, mode_index) {
                            // Filter out resolutions smaller than 640x480
                            if mode.w >= 640 && mode.h >= 480 {
                                let res = Resolution::new(mode.w as u32, mode.h as u32);
                                // Avoid duplicates
                                if !resolutions.contains(&res) {
                                    resolutions.push(res);
                                }
                            }
                        }
                    }
                }
            }

            // Sort by width, then height
            resolutions.sort_by(|a, b| a.width.cmp(&b.width).then(a.height.cmp(&b.height)));

            // If no resolutions found, add default ones
            if resolutions.is_empty() {
                resolutions = vec![
                    Resolution::new(640, 480),
                    Resolution::new(800, 600),
                    Resolution::new(1024, 768),
                    Resolution::new(1280, 960),
                    Resolution::new(1280, 1024),
                    Resolution::new(1366, 768),
                    Resolution::new(1440, 900),
                    Resolution::new(1600, 900),
                    Resolution::new(1680, 1050),
                    Resolution::new(1920, 1080),
                    Resolution::new(1920, 1200),
                ];
            }

            resolutions
        }

        /// Get available resolutions
        pub fn get_available_resolutions(&self) -> &[Resolution] {
            &self.available_resolutions
        }

        /// Change window resolution (windowed mode)
        pub fn change_resolution(&mut self, width: u32, height: u32) -> Result<()> {
            debug!("Changing resolution to {}x{}", width, height);

            if self.current_mode == WindowMode::Windowed {
                self.window.set_size(width, height).map_err(|e| {
                    VideoError::WindowCreationFailed(format!("Failed to resize: {}", e))
                })?;
                self.windowed_size = Resolution::new(width, height);
            }

            Ok(())
        }

        /// Set window mode (windowed, fullscreen, fullscreen desktop)
        pub fn set_window_mode(&mut self, mode: WindowMode) -> Result<()> {
            use sdl2::video::FullscreenType;

            info!("Setting window mode to {:?}", mode);

            // Save current windowed size before going fullscreen
            if self.current_mode == WindowMode::Windowed && mode != WindowMode::Windowed {
                let (w, h) = self.window.size();
                self.windowed_size = Resolution::new(w, h);
            }

            let fullscreen_type = match mode {
                WindowMode::Windowed => {
                    // Restore windowed size when exiting fullscreen
                    if self.current_mode != WindowMode::Windowed {
                        self.window
                            .set_size(self.windowed_size.width, self.windowed_size.height)
                            .map_err(|e| {
                                VideoError::WindowCreationFailed(format!("Failed to resize: {}", e))
                            })?;
                    }
                    FullscreenType::Off
                }
                WindowMode::Fullscreen => FullscreenType::True,
                WindowMode::FullscreenDesktop => FullscreenType::Desktop,
            };

            self.window.set_fullscreen(fullscreen_type).map_err(|e| {
                VideoError::WindowCreationFailed(format!("Failed to set fullscreen: {}", e))
            })?;

            self.current_mode = mode;

            // In fullscreen, cursor is always considered to be in window
            if mode != WindowMode::Windowed {
                // Note: In C++ this sets _cursor.in_window = true
                // We don't have that state here, but it could be added if needed
            }

            Ok(())
        }

        /// Toggle between windowed and fullscreen desktop mode
        pub fn toggle_fullscreen(&mut self) -> Result<()> {
            let new_mode = if self.current_mode == WindowMode::Windowed {
                WindowMode::FullscreenDesktop
            } else {
                WindowMode::Windowed
            };

            self.set_window_mode(new_mode)
        }

        /// Get current window mode
        pub fn get_window_mode(&self) -> WindowMode {
            self.current_mode
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
                SdlWindowEvent::SizeChanged(width, height) => {
                    // Ensure minimum window size (64x64 as per C++ code)
                    let width = (width as u32).max(64);
                    let height = (height as u32).max(64);

                    WindowEvent::SizeChanged { width, height }
                }
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

        /// Get display dimensions for current display
        pub fn get_display_size(&self) -> Result<Resolution> {
            let display_index = self.window.display_index().map_err(|e| {
                VideoError::WindowCreationFailed(format!("Failed to get display index: {}", e))
            })?;

            let mode = self
                .video_subsystem
                .current_display_mode(display_index)
                .map_err(|e| {
                    VideoError::WindowCreationFailed(format!("Failed to get display mode: {}", e))
                })?;

            Ok(Resolution::new(mode.w as u32, mode.h as u32))
        }

        /// Find the best matching resolution for fullscreen
        pub fn find_best_fullscreen_resolution(
            &self,
            desired_width: u32,
            desired_height: u32,
        ) -> Resolution {
            // If we have the exact resolution, use it
            for res in &self.available_resolutions {
                if res.width == desired_width && res.height == desired_height {
                    return *res;
                }
            }

            // Find the closest resolution
            let mut best = self
                .available_resolutions
                .first()
                .copied()
                .unwrap_or(Resolution::new(640, 480));
            let mut best_delta = ((best.width as i32 - desired_width as i32).abs() as u32)
                * ((best.height as i32 - desired_height as i32).abs() as u32);

            for res in &self.available_resolutions {
                let delta = ((res.width as i32 - desired_width as i32).abs() as u32)
                    * ((res.height as i32 - desired_height as i32).abs() as u32);
                if delta < best_delta {
                    best = *res;
                    best_delta = delta;
                }
            }

            best
        }

        /// Handle window resize (called from resize event)
        pub fn handle_resize(&mut self, width: u32, height: u32) -> Result<()> {
            // Update our stored windowed size if in windowed mode
            if self.current_mode == WindowMode::Windowed {
                self.windowed_size = Resolution::new(width, height);
            }

            debug!("Window resized to {}x{}", width, height);
            Ok(())
        }
    }
}

#[cfg(feature = "sdl2-backend")]
pub use sdl2_impl::{Resolution, Sdl2Driver, WindowMode};
