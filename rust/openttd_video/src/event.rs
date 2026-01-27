//! Event types for OpenTTD video system
//! Maps SDL2 events to OpenTTD internal events

use thiserror::Error;

#[derive(Debug, Error)]
pub enum VideoError {
    #[error("SDL2 initialization failed: {0}")]
    InitFailed(String),
    #[error("Window creation failed: {0}")]
    WindowCreationFailed(String),
    #[error("Event processing error: {0}")]
    EventError(String),
}

pub type Result<T> = std::result::Result<T, VideoError>;

/// Mouse button types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    X1,
    X2,
}

/// Keyboard modifier state
#[derive(Debug, Clone, Copy, Default)]
pub struct KeyModifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub gui: bool,
}

/// Window events
#[derive(Debug, Clone)]
pub enum WindowEvent {
    /// Window needs redraw
    Exposed,
    /// Window was resized
    SizeChanged { width: u32, height: u32 },
    /// Mouse entered window
    MouseEnter,
    /// Mouse left window
    MouseLeave,
    /// Window gained focus
    FocusGained,
    /// Window lost focus
    FocusLost,
}

/// Input events matching OpenTTD's internal representation
#[derive(Debug, Clone)]
pub enum Event {
    /// Application quit requested
    Quit,

    /// Mouse moved
    MouseMotion {
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    },

    /// Mouse button pressed
    MouseButtonDown { button: MouseButton, x: i32, y: i32 },

    /// Mouse button released
    MouseButtonUp { button: MouseButton, x: i32, y: i32 },

    /// Mouse wheel scrolled
    MouseWheel { x: i32, y: i32 },

    /// Key pressed
    KeyDown {
        keycode: u32,
        scancode: u32,
        modifiers: KeyModifiers,
    },

    /// Key released
    KeyUp {
        keycode: u32,
        scancode: u32,
        modifiers: KeyModifiers,
    },

    /// Text input
    TextInput { text: String },

    /// Window event
    Window(WindowEvent),
}

impl Event {
    /// Check if this is a quit event
    pub fn is_quit(&self) -> bool {
        matches!(self, Event::Quit)
    }

    /// Check if Alt+Enter was pressed (fullscreen toggle)
    pub fn is_fullscreen_toggle(&self) -> bool {
        if let Event::KeyDown {
            keycode, modifiers, ..
        } = self
        {
            // SDL_SCANCODE_RETURN = 40, SDL_SCANCODE_KP_ENTER = 88
            modifiers.alt && (*keycode == 13 || *keycode == 271) // Return or KP_Enter
        } else {
            false
        }
    }
}
