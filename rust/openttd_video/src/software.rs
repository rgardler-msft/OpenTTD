/// Software rendering backend for SDL2 video driver
///
/// This module implements a software-based rendering backend that uses SDL2's
/// surface blitting capabilities for drawing to the screen.
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::surface::Surface;

/// Represents a dirty rectangle that needs to be redrawn
#[derive(Debug, Clone, Copy, Default)]
pub struct DirtyRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl DirtyRect {
    /// Check if the rectangle is empty
    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    /// Clear the rectangle
    pub fn clear(&mut self) {
        *self = DirtyRect::default();
    }

    /// Expand the rectangle to include the given area
    pub fn expand(&mut self, left: i32, top: i32, right: i32, bottom: i32) {
        if self.is_empty() {
            self.left = left;
            self.top = top;
            self.right = right;
            self.bottom = bottom;
        } else {
            self.left = self.left.min(left);
            self.top = self.top.min(top);
            self.right = self.right.max(right);
            self.bottom = self.bottom.max(bottom);
        }
    }

    /// Convert to SDL rect
    pub fn to_sdl_rect(&self) -> Option<Rect> {
        if self.is_empty() {
            None
        } else {
            Some(Rect::new(
                self.left,
                self.top,
                (self.right - self.left) as u32,
                (self.bottom - self.top) as u32,
            ))
        }
    }
}

/// Palette management for 8-bit color modes
pub struct Palette {
    pub colors: Vec<Color>,
    pub first_dirty: usize,
    pub count_dirty: usize,
}

impl Default for Palette {
    fn default() -> Self {
        // Initialize with a basic 256-color palette
        let mut colors = Vec::with_capacity(256);
        for i in 0..256 {
            colors.push(Color::RGB(i as u8, i as u8, i as u8));
        }

        Palette {
            colors,
            first_dirty: 0,
            count_dirty: 0,
        }
    }
}

impl Palette {
    /// Mark a range of palette entries as dirty
    pub fn mark_dirty(&mut self, first: usize, count: usize) {
        if self.count_dirty == 0 {
            self.first_dirty = first;
            self.count_dirty = count;
        } else {
            let last_dirty = self.first_dirty + self.count_dirty;
            let new_last = first + count;

            self.first_dirty = self.first_dirty.min(first);
            self.count_dirty = new_last.max(last_dirty) - self.first_dirty;
        }
    }

    /// Clear the dirty flag
    pub fn clear_dirty(&mut self) {
        self.count_dirty = 0;
    }

    /// Set a color in the palette
    pub fn set_color(&mut self, index: usize, color: Color) {
        if index < self.colors.len() {
            self.colors[index] = color;
            self.mark_dirty(index, 1);
        }
    }
}

/// Software rendering backend
pub struct SoftwareRenderer {
    rgb_surface: Option<Surface<'static>>,
    dirty_rect: DirtyRect,
    palette: Palette,
    screen_width: u32,
    screen_height: u32,
    bits_per_pixel: u8,
}

impl SoftwareRenderer {
    /// Create a new software renderer
    pub fn new(width: u32, height: u32) -> Result<Self, String> {
        Ok(SoftwareRenderer {
            rgb_surface: None,
            dirty_rect: DirtyRect::default(),
            palette: Palette::default(),
            screen_width: width,
            screen_height: height,
            bits_per_pixel: 32, // Default to 32-bit color
        })
    }

    /// Allocate backing store for the given dimensions
    pub fn allocate_backing_store(
        &mut self,
        width: u32,
        height: u32,
        bpp: u8,
    ) -> Result<bool, String> {
        // Check if we actually need to reallocate
        if self.screen_width == width && self.screen_height == height && self.bits_per_pixel == bpp
        {
            return Ok(false);
        }

        self.screen_width = width;
        self.screen_height = height;
        self.bits_per_pixel = bpp;

        // Free any existing RGB surface
        self.rgb_surface = None;

        // For 8-bit mode, create a shadow surface
        if bpp == 8 {
            let surface = Surface::new(width, height, PixelFormatEnum::Index8)?;

            self.rgb_surface = Some(surface);
        }

        // Clear dirty rect as dimensions have changed
        self.dirty_rect.clear();

        Ok(true)
    }

    /// Get a pointer to the video buffer
    pub fn get_video_pointer(&self) -> *mut u8 {
        if let Some(ref surface) = self.rgb_surface {
            surface.raw() as *mut u8
        } else {
            // Return null pointer if no surface
            std::ptr::null_mut()
        }
    }

    /// Mark an area as dirty
    pub fn make_dirty(&mut self, left: i32, top: i32, width: i32, height: i32) {
        self.dirty_rect
            .expand(left, top, left + width, top + height);
    }

    /// Update the palette (for 8-bit mode)
    pub fn update_palette(&mut self) -> Result<(), String> {
        if self.palette.count_dirty == 0 {
            return Ok(());
        }

        if let Some(ref mut _surface) = self.rgb_surface {
            // Apply palette colors to the surface
            let _palette_slice = &self.palette.colors
                [self.palette.first_dirty..self.palette.first_dirty + self.palette.count_dirty];

            // In a real implementation, we'd use SDL_SetPaletteColors here
            // For now, we'll just mark it as updated
            self.palette.clear_dirty();
        }

        Ok(())
    }

    /// Paint the dirty areas to the screen
    pub fn paint(&mut self) -> Result<(), String> {
        // Skip if nothing to paint
        if self.dirty_rect.is_empty() && self.palette.count_dirty == 0 {
            return Ok(());
        }

        // Update palette if needed
        if self.palette.count_dirty > 0 {
            self.update_palette()?;
        }

        // Get the SDL rect for the dirty area
        if let Some(_rect) = self.dirty_rect.to_sdl_rect() {
            // If we have a shadow surface, blit from it to the window surface
            if let Some(ref _surface) = self.rgb_surface {
                // In a real implementation, we'd use SDL_BlitSurface here
                // and then call SDL_UpdateWindowSurfaceRects
                // For now, this is a placeholder
            }
        }

        // Clear the dirty rect
        self.dirty_rect.clear();

        Ok(())
    }

    /// Lock the video buffer for direct access
    pub fn lock_buffer(&mut self) -> Result<(), String> {
        // Most surfaces don't need explicit locking in SDL2
        Ok(())
    }

    /// Unlock the video buffer
    pub fn unlock_buffer(&mut self) -> Result<(), String> {
        Ok(())
    }

    /// Handle a resize event
    pub fn handle_resize(&mut self, width: u32, height: u32) -> Result<(), String> {
        self.allocate_backing_store(width, height, self.bits_per_pixel)?;
        // Mark entire screen as dirty after resize
        self.make_dirty(0, 0, width as i32, height as i32);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dirty_rect_empty() {
        let rect = DirtyRect::default();
        assert!(rect.is_empty());
    }

    #[test]
    fn test_dirty_rect_expand() {
        let mut rect = DirtyRect::default();
        rect.expand(10, 20, 100, 200);
        assert!(!rect.is_empty());
        assert_eq!(rect.left, 10);
        assert_eq!(rect.top, 20);
        assert_eq!(rect.right, 100);
        assert_eq!(rect.bottom, 200);

        // Expand it further
        rect.expand(5, 15, 150, 250);
        assert_eq!(rect.left, 5);
        assert_eq!(rect.top, 15);
        assert_eq!(rect.right, 150);
        assert_eq!(rect.bottom, 250);
    }

    #[test]
    fn test_dirty_rect_to_sdl() {
        let mut rect = DirtyRect::default();
        assert!(rect.to_sdl_rect().is_none());

        rect.expand(10, 20, 110, 120);
        let sdl_rect = rect.to_sdl_rect().unwrap();
        assert_eq!(sdl_rect.x(), 10);
        assert_eq!(sdl_rect.y(), 20);
        assert_eq!(sdl_rect.width(), 100);
        assert_eq!(sdl_rect.height(), 100);
    }

    #[test]
    fn test_palette_dirty_tracking() {
        let mut palette = Palette::default();
        assert_eq!(palette.count_dirty, 0);

        palette.mark_dirty(10, 5);
        assert_eq!(palette.first_dirty, 10);
        assert_eq!(palette.count_dirty, 5);

        palette.mark_dirty(5, 3);
        assert_eq!(palette.first_dirty, 5);
        assert_eq!(palette.count_dirty, 10); // Should extend from 5 to 15

        palette.clear_dirty();
        assert_eq!(palette.count_dirty, 0);
    }

    #[test]
    fn test_palette_set_color() {
        let mut palette = Palette::default();
        let red = Color::RGB(255, 0, 0);

        palette.set_color(42, red);
        assert_eq!(palette.colors[42], red);
        assert_eq!(palette.first_dirty, 42);
        assert_eq!(palette.count_dirty, 1);
    }
}
