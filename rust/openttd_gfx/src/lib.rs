// OpenTTD Graphics Library
// Provides basic 2D rendering capabilities using SDL2

use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect as SdlRect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use thiserror::Error;

#[cfg(feature = "ttf")]
use sdl2::render::TextureQuery;
#[cfg(feature = "ttf")]
use sdl2::ttf::{Font, Sdl2TtfContext};

#[derive(Debug, Error)]
pub enum GfxError {
    #[error("SDL2 error: {0}")]
    Sdl2(String),

    #[error("TTF error: {0}")]
    Ttf(String),

    #[error("Font not found: {0}")]
    FontNotFound(String),

    #[error("Invalid color format")]
    InvalidColor,
}

impl From<String> for GfxError {
    fn from(s: String) -> Self {
        GfxError::Sdl2(s)
    }
}

/// RGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Colour {
    /// Create a new color with full opacity
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a new color with specified alpha
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Convert to SDL2 color
    fn to_sdl(&self) -> SdlColor {
        SdlColor::RGBA(self.r, self.g, self.b, self.a)
    }
}

// Common OpenTTD colors
impl Colour {
    pub const BLACK: Colour = Colour {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const WHITE: Colour = Colour {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };
    pub const RED: Colour = Colour {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Colour = Colour {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Colour = Colour {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };
    pub const GREY: Colour = Colour {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };
    pub const LIGHT_GREY: Colour = Colour {
        r: 192,
        g: 192,
        b: 192,
        a: 255,
    };
    pub const DARK_GREY: Colour = Colour {
        r: 64,
        g: 64,
        b: 64,
        a: 255,
    };

    // OpenTTD UI colors
    pub const UI_BACKGROUND: Colour = Colour {
        r: 0x2A,
        g: 0x2A,
        b: 0x2A,
        a: 255,
    };
    pub const UI_BUTTON: Colour = Colour {
        r: 0x40,
        g: 0x40,
        b: 0x40,
        a: 255,
    };
    pub const UI_BUTTON_HOVER: Colour = Colour {
        r: 0x50,
        g: 0x50,
        b: 0x50,
        a: 255,
    };
    pub const UI_BUTTON_PRESSED: Colour = Colour {
        r: 0x30,
        g: 0x30,
        b: 0x30,
        a: 255,
    };
    pub const UI_TEXT: Colour = Colour {
        r: 0xE0,
        g: 0xE0,
        b: 0xE0,
        a: 255,
    };
    pub const UI_TEXT_DISABLED: Colour = Colour {
        r: 0x80,
        g: 0x80,
        b: 0x80,
        a: 255,
    };
}

impl Colour {
    /// Helper method for UI background color
    pub fn ui_background() -> Self {
        Self::UI_BACKGROUND
    }

    /// Helper method for UI border color
    pub fn ui_border() -> Self {
        Self::GREY
    }

    /// Helper method for UI text color
    pub fn ui_text() -> Self {
        Self::UI_TEXT
    }

    /// Helper method for UI window background color
    pub fn ui_window_background() -> Self {
        Self::UI_BACKGROUND
    }

    /// Helper method for UI title bar color
    pub fn ui_title_bar() -> Self {
        Self::UI_BUTTON
    }

    /// Helper method for UI title text color
    pub fn ui_title_text() -> Self {
        Self::rgb(255, 255, 255) // White text
    }

    pub fn ui_highlight() -> Self {
        Self::rgb(100, 150, 255) // Light blue for highlights
    }
}

/// Rectangle for drawing operations
#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Convert to SDL2 rectangle
    fn to_sdl(&self) -> SdlRect {
        SdlRect::new(self.x, self.y, self.width, self.height)
    }

    /// Check if a point is inside this rectangle
    pub fn contains_point(&self, x: i32, y: i32) -> bool {
        x >= self.x
            && x < self.x + self.width as i32
            && y >= self.y
            && y < self.y + self.height as i32
    }

    /// Get the center point of this rectangle
    pub fn center(&self) -> (i32, i32) {
        (
            self.x + (self.width / 2) as i32,
            self.y + (self.height / 2) as i32,
        )
    }
}

/// Text alignment options
#[derive(Debug, Clone, Copy)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

/// Button state for rendering
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonState {
    Normal,
    Hover,
    Pressed,
    Disabled,
}

/// Graphics context for rendering operations
pub struct GfxContext<'a> {
    canvas: Canvas<Window>,
    #[cfg(feature = "ttf")]
    ttf_context: Option<&'a Sdl2TtfContext>,
    #[cfg(feature = "ttf")]
    fonts: HashMap<String, Font<'a, 'a>>,
    #[cfg(feature = "ttf")]
    default_font_path: Option<String>,
    #[allow(dead_code)]
    _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a> GfxContext<'a> {
    /// Create a new graphics context from an existing SDL2 window
    pub fn new(window: Window) -> Result<Self, GfxError> {
        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| GfxError::Sdl2(e.to_string()))?;

        Ok(Self {
            canvas,
            #[cfg(feature = "ttf")]
            ttf_context: None,
            #[cfg(feature = "ttf")]
            fonts: HashMap::new(),
            #[cfg(feature = "ttf")]
            default_font_path: None,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Create a new graphics context with TTF support
    #[cfg(feature = "ttf")]
    pub fn new_with_ttf(window: Window, ttf_context: &'a Sdl2TtfContext) -> Result<Self, GfxError> {
        let canvas = window
            .into_canvas()
            .accelerated()
            .present_vsync()
            .build()
            .map_err(|e| GfxError::Sdl2(e.to_string()))?;

        Ok(Self {
            canvas,
            ttf_context: Some(ttf_context),
            fonts: HashMap::new(),
            default_font_path: None,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Load a font from file
    #[cfg(feature = "ttf")]
    pub fn load_font(&mut self, name: &str, path: &Path, point_size: u16) -> Result<(), GfxError> {
        let ttf_context = self
            .ttf_context
            .ok_or_else(|| GfxError::Ttf("TTF context not initialized".to_string()))?;

        let font = ttf_context
            .load_font(path, point_size)
            .map_err(|e| GfxError::Ttf(e.to_string()))?;

        // Set as default if it's the first font
        if self.default_font_path.is_none() {
            self.default_font_path = Some(name.to_string());
        }

        self.fonts.insert(name.to_string(), font);
        Ok(())
    }

    /// Clear the screen with a color
    pub fn clear(&mut self, color: Colour) {
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas.clear();
    }

    /// Present the rendered frame
    pub fn present(&mut self) {
        self.canvas.present();
    }

    /// Fill a rectangle with color
    pub fn fill_rect(&mut self, rect: Rect, color: Colour) -> Result<(), GfxError> {
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas
            .fill_rect(rect.to_sdl())
            .map_err(|e| GfxError::Sdl2(e.to_string()))?;
        Ok(())
    }

    /// Draw a rectangle outline
    pub fn draw_rect(&mut self, rect: Rect, color: Colour) -> Result<(), GfxError> {
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas
            .draw_rect(rect.to_sdl())
            .map_err(|e| GfxError::Sdl2(e.to_string()))?;
        Ok(())
    }

    /// Draw a line
    pub fn draw_line(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        color: Colour,
    ) -> Result<(), GfxError> {
        self.canvas.set_draw_color(color.to_sdl());
        self.canvas
            .draw_line((x1, y1), (x2, y2))
            .map_err(|e| GfxError::Sdl2(e.to_string()))?;
        Ok(())
    }

    /// Draw text at position
    #[cfg(feature = "ttf")]
    pub fn draw_text(
        &mut self,
        text: &str,
        x: i32,
        y: i32,
        color: Colour,
        font_name: Option<&str>,
    ) -> Result<(), GfxError> {
        let font_key = font_name
            .map(|s| s.to_string())
            .or_else(|| self.default_font_path.clone())
            .ok_or_else(|| GfxError::FontNotFound("No font loaded".to_string()))?;

        let font = self
            .fonts
            .get(&font_key)
            .ok_or_else(|| GfxError::FontNotFound(font_key.clone()))?;

        // Render text to surface
        let surface = font
            .render(text)
            .blended(color.to_sdl())
            .map_err(|e| GfxError::Ttf(e.to_string()))?;

        // Convert surface to texture
        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| GfxError::Sdl2(e.to_string()))?;

        // Query texture size
        let TextureQuery { width, height, .. } = texture.query();

        // Copy texture to canvas
        let dest = SdlRect::new(x, y, width, height);
        self.canvas
            .copy(&texture, None, dest)
            .map_err(|e| GfxError::Sdl2(e.to_string()))?;

        Ok(())
    }

    /// Draw text aligned within a rectangle
    #[cfg(feature = "ttf")]
    pub fn draw_text_aligned(
        &mut self,
        text: &str,
        rect: Rect,
        color: Colour,
        align: TextAlign,
        font_name: Option<&str>,
    ) -> Result<(), GfxError> {
        let font_key = font_name
            .map(|s| s.to_string())
            .or_else(|| self.default_font_path.clone())
            .ok_or_else(|| GfxError::FontNotFound("No font loaded".to_string()))?;

        let font = self
            .fonts
            .get(&font_key)
            .ok_or_else(|| GfxError::FontNotFound(font_key.clone()))?;

        // Measure text
        let (text_width, text_height) = font
            .size_of(text)
            .map_err(|e| GfxError::Ttf(e.to_string()))?;

        // Calculate position based on alignment
        let x = match align {
            TextAlign::Left => rect.x,
            TextAlign::Center => rect.x + (rect.width as i32 - text_width as i32) / 2,
            TextAlign::Right => rect.x + rect.width as i32 - text_width as i32,
        };

        // Center vertically
        let y = rect.y + (rect.height as i32 - text_height as i32) / 2;

        self.draw_text(text, x, y, color, font_name)
    }

    /// Draw text at position (stub for non-TTF builds)
    #[cfg(not(feature = "ttf"))]
    pub fn draw_text(
        &mut self,
        _text: &str,
        _x: i32,
        _y: i32,
        _color: Colour,
        _font_name: Option<&str>,
    ) -> Result<(), GfxError> {
        // Text rendering not available without TTF feature
        Ok(())
    }

    /// Draw text aligned within a rectangle (stub for non-TTF builds)
    #[cfg(not(feature = "ttf"))]
    pub fn draw_text_aligned(
        &mut self,
        _text: &str,
        _rect: Rect,
        _color: Colour,
        _align: TextAlign,
        _font_name: Option<&str>,
    ) -> Result<(), GfxError> {
        // Text rendering not available without TTF feature
        Ok(())
    }

    /// Draw a button with text
    pub fn draw_button(
        &mut self,
        rect: Rect,
        text: &str,
        state: ButtonState,
        #[allow(unused_variables)] font_name: Option<&str>,
    ) -> Result<(), GfxError> {
        // Determine colors based on state
        let (bg_color, text_color, border_color) = match state {
            ButtonState::Normal => (Colour::UI_BUTTON, Colour::UI_TEXT, Colour::GREY),
            ButtonState::Hover => (Colour::UI_BUTTON_HOVER, Colour::WHITE, Colour::LIGHT_GREY),
            ButtonState::Pressed => (Colour::UI_BUTTON_PRESSED, Colour::WHITE, Colour::DARK_GREY),
            ButtonState::Disabled => (Colour::DARK_GREY, Colour::UI_TEXT_DISABLED, Colour::GREY),
        };

        // Draw button background
        self.fill_rect(rect, bg_color)?;

        // Draw button border
        self.draw_rect(rect, border_color)?;

        // Draw highlight/shadow for 3D effect
        if state != ButtonState::Disabled {
            let highlight = if state == ButtonState::Pressed {
                Colour::DARK_GREY
            } else {
                Colour::LIGHT_GREY
            };

            let shadow = if state == ButtonState::Pressed {
                Colour::LIGHT_GREY
            } else {
                Colour::DARK_GREY
            };

            // Top and left edges (highlight)
            self.draw_line(
                rect.x,
                rect.y,
                rect.x + rect.width as i32 - 1,
                rect.y,
                highlight,
            )?;
            self.draw_line(
                rect.x,
                rect.y,
                rect.x,
                rect.y + rect.height as i32 - 1,
                highlight,
            )?;

            // Bottom and right edges (shadow)
            self.draw_line(
                rect.x + 1,
                rect.y + rect.height as i32 - 1,
                rect.x + rect.width as i32 - 1,
                rect.y + rect.height as i32 - 1,
                shadow,
            )?;
            self.draw_line(
                rect.x + rect.width as i32 - 1,
                rect.y + 1,
                rect.x + rect.width as i32 - 1,
                rect.y + rect.height as i32 - 1,
                shadow,
            )?;
        }

        // Draw button text
        #[cfg(feature = "ttf")]
        self.draw_text_aligned(text, rect, text_color, TextAlign::Center, font_name)?;

        #[cfg(not(feature = "ttf"))]
        let _ = (text, text_color);

        Ok(())
    }

    /// Get the current window size
    pub fn window_size(&self) -> (u32, u32) {
        self.canvas.output_size().unwrap_or((800, 600))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colour_creation() {
        let color = Colour::rgb(255, 128, 0);
        assert_eq!(color.r, 255);
        assert_eq!(color.g, 128);
        assert_eq!(color.b, 0);
        assert_eq!(color.a, 255);

        let color_alpha = Colour::rgba(255, 128, 0, 128);
        assert_eq!(color_alpha.a, 128);
    }

    #[test]
    fn test_rect_contains_point() {
        let rect = Rect::new(10, 10, 100, 50);

        // Inside
        assert!(rect.contains_point(50, 30));
        assert!(rect.contains_point(10, 10));
        assert!(rect.contains_point(109, 59));

        // Outside
        assert!(!rect.contains_point(9, 10));
        assert!(!rect.contains_point(10, 9));
        assert!(!rect.contains_point(110, 30));
        assert!(!rect.contains_point(50, 60));
    }

    #[test]
    fn test_predefined_colors() {
        assert_eq!(Colour::BLACK, Colour::rgb(0, 0, 0));
        assert_eq!(Colour::WHITE, Colour::rgb(255, 255, 255));
        assert_eq!(Colour::RED, Colour::rgb(255, 0, 0));
        assert_eq!(Colour::GREEN, Colour::rgb(0, 255, 0));
        assert_eq!(Colour::BLUE, Colour::rgb(0, 0, 255));
    }
}
