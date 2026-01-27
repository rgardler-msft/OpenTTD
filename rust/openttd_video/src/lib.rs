//! OpenTTD video subsystem
//!
//! This crate provides the video driver abstraction for OpenTTD,
//! supporting multiple backends (SDL2, etc.)

pub mod event;

#[cfg(feature = "sdl2-backend")]
pub mod sdl2_driver;

#[cfg(test)]
mod tests;

pub use event::{Event, KeyModifiers, MouseButton, Result, VideoError, WindowEvent};

#[cfg(feature = "sdl2-backend")]
pub use sdl2_driver::Sdl2Driver;

/// Video driver trait that all backends must implement
pub trait VideoDriver {
    /// Poll for the next event
    fn poll_event(&mut self) -> Option<Event>;

    /// Get window size
    fn window_size(&self) -> (u32, u32);

    /// Set window title
    fn set_title(&mut self, title: &str);
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
}
