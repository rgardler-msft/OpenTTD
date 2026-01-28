//! OpenTTD video subsystem
//!
//! This crate provides the video driver abstraction for OpenTTD,
//! supporting multiple backends (SDL2, etc.)

pub mod event;

#[cfg(feature = "sdl2-backend")]
pub mod sdl2_driver;

// Additional modules from SDL2 windowing branch
pub mod sdl2;
pub mod software;

#[cfg(test)]
mod tests;

pub use event::{Event, KeyModifiers, MouseButton, Result, VideoError, WindowEvent};

#[cfg(feature = "sdl2-backend")]
pub use sdl2_driver::{Resolution, Sdl2Driver, WindowMode};

/// Video driver trait that all backends must implement
pub trait VideoDriver {
    /// Poll for the next event
    fn poll_event(&mut self) -> Option<Event>;

    /// Get window size
    fn window_size(&self) -> (u32, u32);

    /// Set window title
    fn set_title(&mut self, title: &str);

    /// Change resolution
    fn change_resolution(&mut self, width: u32, height: u32) -> Result<()>;

    /// Toggle fullscreen mode
    fn toggle_fullscreen(&mut self) -> Result<()>;

    /// Set window mode
    fn set_window_mode(&mut self, mode: WindowMode) -> Result<()>;
}

#[cfg(feature = "sdl2-backend")]
impl VideoDriver for Sdl2Driver {
    fn poll_event(&mut self) -> Option<Event> {
        self.poll_event()
    }

    fn window_size(&self) -> (u32, u32) {
        self.window_size()
    }

    fn set_title(&mut self, title: &str) {
        self.set_title(title)
    }

    fn change_resolution(&mut self, width: u32, height: u32) -> Result<()> {
        self.change_resolution(width, height)
    }

    fn toggle_fullscreen(&mut self) -> Result<()> {
        self.toggle_fullscreen()
    }

    fn set_window_mode(&mut self, mode: WindowMode) -> Result<()> {
        self.set_window_mode(mode)
    }
}
