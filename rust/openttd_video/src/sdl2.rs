use thiserror::Error;

#[cfg(feature = "sdl2")]
use std::collections::HashSet;

#[cfg(feature = "sdl2")]
use sdl2::{
    self, event::Event, render::Canvas, video::Window as Sdl2Window, video::WindowBuildError,
};

#[derive(Debug, Error)]
pub enum Sdl2VideoError {
    #[error("video subsystem already initialized")]
    AlreadyInitialized,
    #[error("video subsystem not initialized")]
    NotInitialized,
    #[error("sdl2 init failed: {0}")]
    InitFailed(String),
    #[error("event pump failed: {0}")]
    EventPumpFailed(String),
    #[error("window creation failed: {0}")]
    WindowCreateFailed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VideoMode {
    pub width: u32,
    pub height: u32,
}

impl VideoMode {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoEvent {
    Quit,
    WindowResized {
        width: u32,
        height: u32,
    },
    WindowFocusChanged {
        focused: bool,
    },
    KeyDown {
        key: KeyCode,
        modifiers: KeyModifiers,
        char: Option<char>,
    },
    KeyUp {
        key: KeyCode,
        modifiers: KeyModifiers,
    },
    TextInput {
        text: String,
    },
    MouseMotion {
        x: i32,
        y: i32,
    },
    MouseButtonDown {
        button: MouseButton,
        x: i32,
        y: i32,
    },
    MouseButtonUp {
        button: MouseButton,
        x: i32,
        y: i32,
    },
    MouseWheel {
        x: i32,
        y: i32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    X1,
    X2,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool, // Command key on macOS
}

impl KeyModifiers {
    pub fn none() -> Self {
        Self {
            shift: false,
            ctrl: false,
            alt: false,
            meta: false,
        }
    }
}

// Key codes matching OpenTTD's WKC_* constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
    // Special keys
    Backspace,
    Tab,
    Return,
    Escape,
    Space,
    Delete,

    // Arrow keys
    Up,
    Down,
    Left,
    Right,

    // Navigation
    Home,
    End,
    PageUp,
    PageDown,
    Insert,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Numpad
    NumpadDivide,
    NumpadMultiply,
    NumpadMinus,
    NumpadPlus,
    NumpadEnter,
    NumpadDecimal,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    // Punctuation
    Slash,
    Backslash,
    Semicolon,
    Quote,
    Comma,
    Period,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backquote,

    // Misc
    Pause,

    // Unknown key
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowOptions {
    pub mode: WindowMode,
    pub resizable: bool,
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            mode: WindowMode::Windowed,
            resizable: true,
        }
    }
}

pub struct Window {
    title: String,
    mode: WindowMode,
    resizable: bool,
    size: VideoMode,
    #[cfg(feature = "sdl2")]
    canvas: Canvas<Sdl2Window>,
}

impl std::fmt::Debug for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Window")
            .field("title", &self.title)
            .field("mode", &self.mode)
            .field("resizable", &self.resizable)
            .field("size", &self.size)
            .finish()
    }
}

pub struct VideoSubsystem {
    initialized: bool,
    text_input_active: bool,
    #[cfg(feature = "sdl2")]
    _context: sdl2::Sdl,
    #[cfg(feature = "sdl2")]
    video: sdl2::VideoSubsystem,
    #[cfg(feature = "sdl2")]
    event_pump: sdl2::EventPump,
}

impl std::fmt::Debug for VideoSubsystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VideoSubsystem")
            .field("initialized", &self.initialized)
            .field("text_input_active", &self.text_input_active)
            .finish()
    }
}

impl VideoSubsystem {
    pub fn init() -> Result<Self, Sdl2VideoError> {
        #[cfg(feature = "sdl2")]
        {
            let context = sdl2::init().map_err(|err| Sdl2VideoError::InitFailed(err))?;
            let video = context
                .video()
                .map_err(|err| Sdl2VideoError::InitFailed(err))?;
            let event_pump = context
                .event_pump()
                .map_err(|err| Sdl2VideoError::EventPumpFailed(err))?;
            return Ok(Self {
                initialized: true,
                text_input_active: false,
                _context: context,
                video,
                event_pump,
            });
        }

        #[cfg(not(feature = "sdl2"))]
        {
            Ok(Self {
                initialized: true,
                text_input_active: false,
            })
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn supports_events(&self) -> bool {
        #[cfg(feature = "sdl2")]
        {
            return true;
        }

        #[cfg(not(feature = "sdl2"))]
        {
            return false;
        }
    }

    pub fn current_driver(&self) -> Option<String> {
        #[cfg(feature = "sdl2")]
        {
            return Some(self.video.current_video_driver().to_string());
        }

        #[cfg(not(feature = "sdl2"))]
        {
            None
        }
    }

    /// Get available display resolutions, similar to C++ FindResolutions()
    pub fn find_resolutions(&self) -> Vec<VideoMode> {
        #[cfg(feature = "sdl2")]
        {
            let mut resolutions = HashSet::new();

            // Get resolutions from all displays
            let num_displays = self.video.num_video_displays().unwrap_or(0);
            for display_index in 0..num_displays {
                if let Ok(num_modes) = self.video.num_display_modes(display_index) {
                    for mode_index in 0..num_modes {
                        if let Ok(mode) = self.video.display_mode(display_index, mode_index) {
                            // Skip resolutions smaller than 640x480 (matching C++)
                            if mode.w >= 640 && mode.h >= 480 {
                                resolutions.insert(VideoMode::new(mode.w as u32, mode.h as u32));
                            }
                        }
                    }
                }
            }

            // If no resolutions found, use default list (matching C++)
            if resolutions.is_empty() {
                vec![
                    VideoMode::new(640, 480),
                    VideoMode::new(800, 600),
                    VideoMode::new(1024, 768),
                    VideoMode::new(1152, 864),
                    VideoMode::new(1280, 800),
                    VideoMode::new(1280, 960),
                    VideoMode::new(1280, 1024),
                    VideoMode::new(1400, 1050),
                    VideoMode::new(1600, 1200),
                    VideoMode::new(1680, 1050),
                    VideoMode::new(1920, 1200),
                ]
            } else {
                let mut sorted: Vec<_> = resolutions.into_iter().collect();
                sorted.sort();
                sorted
            }
        }

        #[cfg(not(feature = "sdl2"))]
        {
            // Return default resolutions for stub
            vec![
                VideoMode::new(640, 480),
                VideoMode::new(800, 600),
                VideoMode::new(1024, 768),
            ]
        }
    }

    /// Get the best available video mode for requested dimensions
    pub fn get_available_video_mode(&self, width: u32, height: u32, fullscreen: bool) -> VideoMode {
        if !fullscreen {
            return VideoMode::new(width, height);
        }

        let resolutions = self.find_resolutions();
        if resolutions.is_empty() {
            return VideoMode::new(width, height);
        }

        // Check if requested mode is available
        let requested = VideoMode::new(width, height);
        if resolutions.contains(&requested) {
            return requested;
        }

        // Find closest resolution (matching C++ logic)
        let mut best = resolutions[0];
        let mut best_delta = ((best.width as i32 - width as i32).abs() as u32)
            * ((best.height as i32 - height as i32).abs() as u32);

        for resolution in &resolutions[1..] {
            let delta = ((resolution.width as i32 - width as i32).abs() as u32)
                * ((resolution.height as i32 - height as i32).abs() as u32);
            if delta < best_delta {
                best = *resolution;
                best_delta = delta;
            }
        }

        best
    }

    /// Get refresh rates for all monitors
    pub fn get_monitor_refresh_rates(&self) -> Vec<i32> {
        #[cfg(feature = "sdl2")]
        {
            let mut rates = Vec::new();
            let num_displays = self.video.num_video_displays().unwrap_or(0);

            for i in 0..num_displays {
                if let Ok(mode) = self.video.desktop_display_mode(i) {
                    if mode.refresh_rate > 0 {
                        rates.push(mode.refresh_rate);
                    }
                }
            }

            rates
        }

        #[cfg(not(feature = "sdl2"))]
        {
            vec![60] // Default refresh rate
        }
    }

    pub fn poll_event(&mut self) -> Result<Option<VideoEvent>, Sdl2VideoError> {
        if !self.initialized {
            return Err(Sdl2VideoError::NotInitialized);
        }

        #[cfg(feature = "sdl2")]
        {
            if let Some(event) = self.event_pump.poll_event() {
                return Ok(map_event(event));
            }
            return Ok(None);
        }

        #[cfg(not(feature = "sdl2"))]
        {
            Ok(None)
        }
    }

    /// Enable text input mode (for edit boxes)
    pub fn start_text_input(&mut self) {
        if !self.text_input_active {
            #[cfg(feature = "sdl2")]
            {
                self.video.text_input().start();
            }
            self.text_input_active = true;
        }
    }

    /// Disable text input mode
    pub fn stop_text_input(&mut self) {
        if self.text_input_active {
            #[cfg(feature = "sdl2")]
            {
                self.video.text_input().stop();
            }
            self.text_input_active = false;
        }
    }

    /// Check if text input is active
    pub fn is_text_input_active(&self) -> bool {
        self.text_input_active
    }

    pub fn create_window(
        &self,
        title: impl Into<String>,
        mode: VideoMode,
        options: WindowOptions,
    ) -> Result<Window, Sdl2VideoError> {
        if !self.initialized {
            return Err(Sdl2VideoError::NotInitialized);
        }

        let title = title.into();

        #[cfg(feature = "sdl2")]
        {
            let mut builder = self.video.window(&title, mode.width, mode.height);
            if options.resizable {
                builder.resizable();
            }
            if options.mode == WindowMode::Fullscreen {
                builder.fullscreen();
            }
            builder.position_centered();
            let window = builder.build().map_err(map_window_error)?;
            let mut canvas = window
                .into_canvas()
                .accelerated()
                .build()
                .map_err(|err| Sdl2VideoError::WindowCreateFailed(err.to_string()))?;
            canvas.window_mut().show();
            canvas.window_mut().raise();
            canvas.set_draw_color(sdl2::pixels::Color::RGB(24, 40, 72));
            canvas.clear();
            canvas.present();

            return Ok(Window {
                title,
                mode: options.mode,
                resizable: options.resizable,
                size: mode,
                canvas,
            });
        }

        #[cfg(not(feature = "sdl2"))]
        {
            Ok(Window {
                title,
                mode: options.mode,
                resizable: options.resizable,
                size: mode,
            })
        }
    }
}

impl Window {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn size(&self) -> VideoMode {
        self.size
    }

    pub fn mode(&self) -> WindowMode {
        self.mode
    }

    pub fn is_resizable(&self) -> bool {
        self.resizable
    }

    pub fn set_mode(&mut self, mode: WindowMode) {
        self.mode = mode;
    }

    #[cfg(feature = "sdl2")]
    pub fn show(&mut self) {
        self.canvas.window_mut().show();
        self.canvas.window_mut().raise();
    }

    #[cfg(feature = "sdl2")]
    pub fn set_fullscreen(&mut self, fullscreen: bool) -> Result<(), Sdl2VideoError> {
        let target = if fullscreen {
            sdl2::video::FullscreenType::Desktop
        } else {
            sdl2::video::FullscreenType::Off
        };
        self.canvas
            .window_mut()
            .set_fullscreen(target)
            .map_err(|err| Sdl2VideoError::WindowCreateFailed(err))?;
        self.mode = if fullscreen {
            WindowMode::Fullscreen
        } else {
            WindowMode::Windowed
        };
        Ok(())
    }

    #[cfg(feature = "sdl2")]
    pub fn change_resolution(&mut self, width: u32, height: u32) -> Result<(), Sdl2VideoError> {
        self.canvas
            .window_mut()
            .set_size(width, height)
            .map_err(|err| Sdl2VideoError::WindowCreateFailed(err.to_string()))?;
        self.size = VideoMode::new(width, height);
        Ok(())
    }

    #[cfg(not(feature = "sdl2"))]
    pub fn change_resolution(&mut self, width: u32, height: u32) -> Result<(), Sdl2VideoError> {
        self.size = VideoMode::new(width, height);
        Ok(())
    }

    #[cfg(feature = "sdl2")]
    pub fn render_solid_color(&mut self, r: u8, g: u8, b: u8) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(r, g, b));
        self.canvas.clear();
        self.canvas.present();
    }

    #[cfg(not(feature = "sdl2"))]
    pub fn render_solid_color(&mut self, _r: u8, _g: u8, _b: u8) {}
}

#[cfg(feature = "sdl2")]
fn map_window_error(err: WindowBuildError) -> Sdl2VideoError {
    Sdl2VideoError::WindowCreateFailed(err.to_string())
}

#[cfg(feature = "sdl2")]
fn map_event(event: Event) -> Option<VideoEvent> {
    match event {
        Event::Quit { .. } => Some(VideoEvent::Quit),
        Event::Window { win_event, .. } => match win_event {
            sdl2::event::WindowEvent::Resized(width, height)
            | sdl2::event::WindowEvent::SizeChanged(width, height) => {
                Some(VideoEvent::WindowResized {
                    width: width as u32,
                    height: height as u32,
                })
            }
            sdl2::event::WindowEvent::FocusGained => {
                Some(VideoEvent::WindowFocusChanged { focused: true })
            }
            sdl2::event::WindowEvent::FocusLost => {
                Some(VideoEvent::WindowFocusChanged { focused: false })
            }
            _ => None,
        },
        Event::KeyDown {
            keycode, keymod, ..
        } => {
            if let Some(keycode) = keycode {
                let key = map_keycode(keycode);
                let modifiers = map_modifiers(keymod);
                let char = if !modifiers.ctrl && !modifiers.alt && !modifiers.meta {
                    keycode_to_char(keycode, modifiers.shift)
                } else {
                    None
                };
                Some(VideoEvent::KeyDown {
                    key,
                    modifiers,
                    char,
                })
            } else {
                None
            }
        }
        Event::KeyUp {
            keycode, keymod, ..
        } => {
            if let Some(keycode) = keycode {
                let key = map_keycode(keycode);
                let modifiers = map_modifiers(keymod);
                Some(VideoEvent::KeyUp { key, modifiers })
            } else {
                None
            }
        }
        Event::TextInput { text, .. } => Some(VideoEvent::TextInput { text }),
        Event::MouseMotion { x, y, .. } => Some(VideoEvent::MouseMotion { x, y }),
        Event::MouseButtonDown {
            mouse_btn, x, y, ..
        } => {
            let button = map_mouse_button(mouse_btn);
            Some(VideoEvent::MouseButtonDown { button, x, y })
        }
        Event::MouseButtonUp {
            mouse_btn, x, y, ..
        } => {
            let button = map_mouse_button(mouse_btn);
            Some(VideoEvent::MouseButtonUp { button, x, y })
        }
        Event::MouseWheel { x, y, .. } => Some(VideoEvent::MouseWheel { x, y }),
        _ => None,
    }
}

#[cfg(feature = "sdl2")]
fn map_keycode(keycode: sdl2::keyboard::Keycode) -> KeyCode {
    use sdl2::keyboard::Keycode as SDL;

    match keycode {
        // Special keys
        SDL::Backspace => KeyCode::Backspace,
        SDL::Tab => KeyCode::Tab,
        SDL::Return | SDL::Return2 => KeyCode::Return,
        SDL::Escape => KeyCode::Escape,
        SDL::Space => KeyCode::Space,
        SDL::Delete => KeyCode::Delete,

        // Arrow keys
        SDL::Up => KeyCode::Up,
        SDL::Down => KeyCode::Down,
        SDL::Left => KeyCode::Left,
        SDL::Right => KeyCode::Right,

        // Navigation
        SDL::Home => KeyCode::Home,
        SDL::End => KeyCode::End,
        SDL::PageUp => KeyCode::PageUp,
        SDL::PageDown => KeyCode::PageDown,
        SDL::Insert => KeyCode::Insert,

        // Function keys
        SDL::F1 => KeyCode::F1,
        SDL::F2 => KeyCode::F2,
        SDL::F3 => KeyCode::F3,
        SDL::F4 => KeyCode::F4,
        SDL::F5 => KeyCode::F5,
        SDL::F6 => KeyCode::F6,
        SDL::F7 => KeyCode::F7,
        SDL::F8 => KeyCode::F8,
        SDL::F9 => KeyCode::F9,
        SDL::F10 => KeyCode::F10,
        SDL::F11 => KeyCode::F11,
        SDL::F12 => KeyCode::F12,

        // Letters
        SDL::A => KeyCode::A,
        SDL::B => KeyCode::B,
        SDL::C => KeyCode::C,
        SDL::D => KeyCode::D,
        SDL::E => KeyCode::E,
        SDL::F => KeyCode::F,
        SDL::G => KeyCode::G,
        SDL::H => KeyCode::H,
        SDL::I => KeyCode::I,
        SDL::J => KeyCode::J,
        SDL::K => KeyCode::K,
        SDL::L => KeyCode::L,
        SDL::M => KeyCode::M,
        SDL::N => KeyCode::N,
        SDL::O => KeyCode::O,
        SDL::P => KeyCode::P,
        SDL::Q => KeyCode::Q,
        SDL::R => KeyCode::R,
        SDL::S => KeyCode::S,
        SDL::T => KeyCode::T,
        SDL::U => KeyCode::U,
        SDL::V => KeyCode::V,
        SDL::W => KeyCode::W,
        SDL::X => KeyCode::X,
        SDL::Y => KeyCode::Y,
        SDL::Z => KeyCode::Z,

        // Numbers
        SDL::Num0 => KeyCode::Num0,
        SDL::Num1 => KeyCode::Num1,
        SDL::Num2 => KeyCode::Num2,
        SDL::Num3 => KeyCode::Num3,
        SDL::Num4 => KeyCode::Num4,
        SDL::Num5 => KeyCode::Num5,
        SDL::Num6 => KeyCode::Num6,
        SDL::Num7 => KeyCode::Num7,
        SDL::Num8 => KeyCode::Num8,
        SDL::Num9 => KeyCode::Num9,

        // Numpad
        SDL::KpDivide => KeyCode::NumpadDivide,
        SDL::KpMultiply => KeyCode::NumpadMultiply,
        SDL::KpMinus => KeyCode::NumpadMinus,
        SDL::KpPlus => KeyCode::NumpadPlus,
        SDL::KpEnter => KeyCode::NumpadEnter,
        SDL::KpPeriod => KeyCode::NumpadDecimal,
        SDL::Kp0 => KeyCode::Numpad0,
        SDL::Kp1 => KeyCode::Numpad1,
        SDL::Kp2 => KeyCode::Numpad2,
        SDL::Kp3 => KeyCode::Numpad3,
        SDL::Kp4 => KeyCode::Numpad4,
        SDL::Kp5 => KeyCode::Numpad5,
        SDL::Kp6 => KeyCode::Numpad6,
        SDL::Kp7 => KeyCode::Numpad7,
        SDL::Kp8 => KeyCode::Numpad8,
        SDL::Kp9 => KeyCode::Numpad9,

        // Punctuation
        SDL::Slash => KeyCode::Slash,
        SDL::Backslash => KeyCode::Backslash,
        SDL::Semicolon => KeyCode::Semicolon,
        SDL::Quote => KeyCode::Quote,
        SDL::Comma => KeyCode::Comma,
        SDL::Period => KeyCode::Period,
        SDL::Minus => KeyCode::Minus,
        SDL::Equals => KeyCode::Equals,
        SDL::LeftBracket => KeyCode::LeftBracket,
        SDL::RightBracket => KeyCode::RightBracket,
        SDL::Backquote => KeyCode::Backquote,

        // Misc
        SDL::Pause => KeyCode::Pause,

        _ => KeyCode::Unknown,
    }
}

#[cfg(feature = "sdl2")]
fn map_modifiers(keymod: sdl2::keyboard::Mod) -> KeyModifiers {
    KeyModifiers {
        shift: keymod.contains(sdl2::keyboard::Mod::LSHIFTMOD)
            || keymod.contains(sdl2::keyboard::Mod::RSHIFTMOD),
        ctrl: keymod.contains(sdl2::keyboard::Mod::LCTRLMOD)
            || keymod.contains(sdl2::keyboard::Mod::RCTRLMOD),
        alt: keymod.contains(sdl2::keyboard::Mod::LALTMOD)
            || keymod.contains(sdl2::keyboard::Mod::RALTMOD),
        meta: keymod.contains(sdl2::keyboard::Mod::LGUIMOD)
            || keymod.contains(sdl2::keyboard::Mod::RGUIMOD),
    }
}

#[cfg(feature = "sdl2")]
fn map_mouse_button(button: sdl2::mouse::MouseButton) -> MouseButton {
    match button {
        sdl2::mouse::MouseButton::Left => MouseButton::Left,
        sdl2::mouse::MouseButton::Middle => MouseButton::Middle,
        sdl2::mouse::MouseButton::Right => MouseButton::Right,
        sdl2::mouse::MouseButton::X1 => MouseButton::X1,
        sdl2::mouse::MouseButton::X2 => MouseButton::X2,
        _ => MouseButton::Unknown,
    }
}

#[cfg(feature = "sdl2")]
fn keycode_to_char(keycode: sdl2::keyboard::Keycode, shift: bool) -> Option<char> {
    use sdl2::keyboard::Keycode as SDL;

    match keycode {
        SDL::A => Some(if shift { 'A' } else { 'a' }),
        SDL::B => Some(if shift { 'B' } else { 'b' }),
        SDL::C => Some(if shift { 'C' } else { 'c' }),
        SDL::D => Some(if shift { 'D' } else { 'd' }),
        SDL::E => Some(if shift { 'E' } else { 'e' }),
        SDL::F => Some(if shift { 'F' } else { 'f' }),
        SDL::G => Some(if shift { 'G' } else { 'g' }),
        SDL::H => Some(if shift { 'H' } else { 'h' }),
        SDL::I => Some(if shift { 'I' } else { 'i' }),
        SDL::J => Some(if shift { 'J' } else { 'j' }),
        SDL::K => Some(if shift { 'K' } else { 'k' }),
        SDL::L => Some(if shift { 'L' } else { 'l' }),
        SDL::M => Some(if shift { 'M' } else { 'm' }),
        SDL::N => Some(if shift { 'N' } else { 'n' }),
        SDL::O => Some(if shift { 'O' } else { 'o' }),
        SDL::P => Some(if shift { 'P' } else { 'p' }),
        SDL::Q => Some(if shift { 'Q' } else { 'q' }),
        SDL::R => Some(if shift { 'R' } else { 'r' }),
        SDL::S => Some(if shift { 'S' } else { 's' }),
        SDL::T => Some(if shift { 'T' } else { 't' }),
        SDL::U => Some(if shift { 'U' } else { 'u' }),
        SDL::V => Some(if shift { 'V' } else { 'v' }),
        SDL::W => Some(if shift { 'W' } else { 'w' }),
        SDL::X => Some(if shift { 'X' } else { 'x' }),
        SDL::Y => Some(if shift { 'Y' } else { 'y' }),
        SDL::Z => Some(if shift { 'Z' } else { 'z' }),

        SDL::Num0 => Some(if shift { ')' } else { '0' }),
        SDL::Num1 => Some(if shift { '!' } else { '1' }),
        SDL::Num2 => Some(if shift { '@' } else { '2' }),
        SDL::Num3 => Some(if shift { '#' } else { '3' }),
        SDL::Num4 => Some(if shift { '$' } else { '4' }),
        SDL::Num5 => Some(if shift { '%' } else { '5' }),
        SDL::Num6 => Some(if shift { '^' } else { '6' }),
        SDL::Num7 => Some(if shift { '&' } else { '7' }),
        SDL::Num8 => Some(if shift { '*' } else { '8' }),
        SDL::Num9 => Some(if shift { '(' } else { '9' }),

        SDL::Space => Some(' '),
        SDL::Comma => Some(if shift { '<' } else { ',' }),
        SDL::Period => Some(if shift { '>' } else { '.' }),
        SDL::Slash => Some(if shift { '?' } else { '/' }),
        SDL::Semicolon => Some(if shift { ':' } else { ';' }),
        SDL::Quote => Some(if shift { '"' } else { '\'' }),
        SDL::LeftBracket => Some(if shift { '{' } else { '[' }),
        SDL::RightBracket => Some(if shift { '}' } else { ']' }),
        SDL::Backslash => Some(if shift { '|' } else { '\\' }),
        SDL::Minus => Some(if shift { '_' } else { '-' }),
        SDL::Equals => Some(if shift { '+' } else { '=' }),
        SDL::Backquote => Some(if shift { '~' } else { '`' }),

        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_window_with_default_options() {
        let video = VideoSubsystem::init().expect("init ok");
        let window = video
            .create_window(
                "OpenTTD",
                VideoMode {
                    width: 640,
                    height: 480,
                },
                WindowOptions::default(),
            )
            .expect("window ok");

        assert_eq!(window.title(), "OpenTTD");
        assert_eq!(window.size().width, 640);
        assert_eq!(window.size().height, 480);
        assert_eq!(window.mode(), WindowMode::Windowed);
        assert!(window.is_resizable());
    }

    #[test]
    fn toggles_window_mode() {
        let video = VideoSubsystem::init().expect("init ok");
        let mut window = video
            .create_window(
                "OpenTTD",
                VideoMode {
                    width: 800,
                    height: 600,
                },
                WindowOptions::default(),
            )
            .expect("window ok");

        assert_eq!(window.mode(), WindowMode::Windowed);
        window.set_mode(WindowMode::Fullscreen);
        assert_eq!(window.mode(), WindowMode::Fullscreen);
    }

    #[test]
    fn finds_resolutions() {
        let video = VideoSubsystem::init().expect("init ok");
        let resolutions = video.find_resolutions();

        // Should have at least some resolutions (default list if nothing else)
        assert!(!resolutions.is_empty());

        // All resolutions should be at least 640x480
        for res in &resolutions {
            assert!(res.width >= 640);
            assert!(res.height >= 480);
        }

        // Should be sorted
        for window in resolutions.windows(2) {
            assert!(window[0] <= window[1]);
        }
    }

    #[test]
    fn gets_best_video_mode() {
        let video = VideoSubsystem::init().expect("init ok");

        // In windowed mode, should return exact requested size
        let mode = video.get_available_video_mode(1234, 567, false);
        assert_eq!(mode.width, 1234);
        assert_eq!(mode.height, 567);

        // In fullscreen, should find closest available
        let mode = video.get_available_video_mode(1000, 700, true);
        assert!(mode.width >= 640);
        assert!(mode.height >= 480);
    }

    #[test]
    fn changes_resolution() {
        let video = VideoSubsystem::init().expect("init ok");
        let mut window = video
            .create_window(
                "OpenTTD",
                VideoMode::new(800, 600),
                WindowOptions::default(),
            )
            .expect("window ok");

        assert_eq!(window.size(), VideoMode::new(800, 600));

        window.change_resolution(1024, 768).expect("resize ok");
        assert_eq!(window.size(), VideoMode::new(1024, 768));
    }

    #[test]
    fn manages_text_input() {
        let mut video = VideoSubsystem::init().expect("init ok");

        assert!(!video.is_text_input_active());

        video.start_text_input();
        assert!(video.is_text_input_active());

        video.stop_text_input();
        assert!(!video.is_text_input_active());
    }

    #[test]
    fn stub_event_poll_returns_none() {
        let mut video = VideoSubsystem::init().expect("init ok");

        // Stub version doesn't support events
        #[cfg(not(feature = "sdl2"))]
        assert!(!video.supports_events());

        let event = video.poll_event().expect("poll ok");
        assert!(event.is_none());
    }
}
