//! OpenTTD GUI System - Window and Widget Framework
//!
//! This module provides the window and widget system for OpenTTD's graphical user interface.
//! It handles window management, widget layouts, event routing, and rendering.

mod date_selector;
mod highscore;
mod league;
mod main_menu;
mod main_menu_window;
mod settings_audio;
mod settings_gameplay;
mod settings_graphics;
mod toolbar;
mod world_gen;

pub use date_selector::{
    show_date_selector, DateSelectorWindow, GameDate, DATE_SELECTOR_WINDOW_ID,
};
pub use highscore::{
    draw_highscore_window, show_highscore_table, DifficultyLevel, HighScore, HIGHSCORE_WINDOW_ID,
};
pub use league::{
    show_league_table, CompanyInfo, LeagueWindow, PerformanceDetailWindow, PerformanceTitle,
    LEAGUE_WINDOW_ID, PERFORMANCE_DETAIL_WINDOW_ID,
};
pub use main_menu::{create_main_menu_window, handle_main_menu_click, MainMenuWidgets};
pub use main_menu_window::{show_main_menu, MainMenuWindow};
pub use settings_audio::{
    show_audio_settings, AudioSettingsAction, AudioSettingsWindow, AUDIO_SETTINGS_WINDOW_ID,
};
pub use settings_gameplay::{
    show_gameplay_settings, GameplaySettingsAction, GameplaySettingsWindow,
    GAMEPLAY_SETTINGS_WINDOW_ID,
};
pub use settings_graphics::{
    show_graphics_settings, GraphicsSettingsAction, GraphicsSettingsWindow,
    GRAPHICS_SETTINGS_WINDOW_ID,
};
pub use toolbar::{show_toolbar, ToolbarWindow, TOOLBAR_WINDOW_ID};
pub use world_gen::{show_world_gen, WorldGenWindow, WORLD_GEN_WINDOW_ID};

use openttd_gfx::{ButtonState, Colour, GfxContext, Rect};
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GuiError {
    #[error("Widget not found: {0}")]
    WidgetNotFound(WidgetID),

    #[error("Window not found: {0}")]
    WindowNotFound(WindowID),

    #[error("Invalid layout")]
    InvalidLayout,
}

pub type Result<T> = std::result::Result<T, GuiError>;

// Widget and Window ID types
pub type WidgetID = u32;
pub type WindowID = u32;

pub const INVALID_WIDGET: WidgetID = 0xFFFFFFFF;

/// Widget type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidgetType {
    Empty,      // Placeholder widget
    Panel,      // Simple panel
    Button,     // Push button
    Label,      // Text label
    Frame,      // Frame with optional title
    Horizontal, // Horizontal container
    Vertical,   // Vertical container
    Spacer,     // Flexible spacer
}

/// Text alignment for labels and buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

/// Size constraints for widgets
#[derive(Debug, Clone, Copy)]
pub struct SizeConstraints {
    pub min_width: u32,
    pub min_height: u32,
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub preferred_width: u32,
    pub preferred_height: u32,
}

impl Default for SizeConstraints {
    fn default() -> Self {
        Self {
            min_width: 0,
            min_height: 0,
            max_width: None,
            max_height: None,
            preferred_width: 100,
            preferred_height: 20,
        }
    }
}

/// Base trait for all widgets
pub trait Widget {
    /// Get the widget's unique ID
    fn get_id(&self) -> WidgetID;

    /// Get the widget type
    fn get_type(&self) -> WidgetType;

    /// Get the widget's position and size
    fn get_rect(&self) -> &Rect;

    /// Set the widget's position and size
    fn set_rect(&mut self, rect: Rect);

    /// Calculate minimum size requirements
    fn calculate_size(&mut self) -> SizeConstraints;

    /// Draw the widget
    fn draw(&self, gfx: &mut GfxContext);

    /// Handle mouse click events
    fn on_click(&mut self, x: i32, y: i32, button: MouseButton) -> bool {
        let _ = (x, y, button); // Default: ignore clicks
        false
    }

    /// Handle mouse movement
    fn on_mouse_move(&mut self, x: i32, y: i32) -> bool {
        let _ = (x, y);
        false
    }

    /// Check if point is inside widget
    fn contains_point(&self, x: i32, y: i32) -> bool {
        self.get_rect().contains_point(x, y)
    }

    /// Set widget visibility
    fn set_visible(&mut self, visible: bool);

    /// Check if widget is visible
    fn is_visible(&self) -> bool;

    /// Set widget enabled state
    fn set_enabled(&mut self, enabled: bool);

    /// Check if widget is enabled
    fn is_enabled(&self) -> bool;
}

/// Base widget data shared by all widgets
pub struct WidgetBase {
    pub id: WidgetID,
    pub widget_type: WidgetType,
    pub rect: Rect,
    pub visible: bool,
    pub enabled: bool,
}

impl WidgetBase {
    pub fn new(id: WidgetID, widget_type: WidgetType) -> Self {
        Self {
            id,
            widget_type,
            rect: Rect {
                x: 0,
                y: 0,
                width: 100,
                height: 20,
            },
            visible: true,
            enabled: true,
        }
    }
}

// Panel Widget - Simple container with background
pub struct PanelWidget {
    base: WidgetBase,
    background_colour: Colour,
    border: bool,
}

impl PanelWidget {
    pub fn new(id: WidgetID) -> Self {
        Self {
            base: WidgetBase::new(id, WidgetType::Panel),
            background_colour: Colour::ui_background(),
            border: true,
        }
    }

    pub fn with_colour(mut self, colour: Colour) -> Self {
        self.background_colour = colour;
        self
    }

    pub fn with_border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
}

impl Widget for PanelWidget {
    fn get_id(&self) -> WidgetID {
        self.base.id
    }
    fn get_type(&self) -> WidgetType {
        self.base.widget_type
    }
    fn get_rect(&self) -> &Rect {
        &self.base.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.base.rect = rect;
    }

    fn calculate_size(&mut self) -> SizeConstraints {
        SizeConstraints {
            min_width: 20,
            min_height: 20,
            ..Default::default()
        }
    }

    fn draw(&self, gfx: &mut GfxContext) {
        if !self.base.visible {
            return;
        }

        // Fill background
        let _ = gfx.fill_rect(self.base.rect, self.background_colour);

        // Draw border if enabled
        if self.border {
            let _ = gfx.draw_rect(self.base.rect, Colour::ui_border());
        }
    }

    fn set_visible(&mut self, visible: bool) {
        self.base.visible = visible;
    }
    fn is_visible(&self) -> bool {
        self.base.visible
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.base.enabled = enabled;
    }
    fn is_enabled(&self) -> bool {
        self.base.enabled
    }
}

// Button Widget
pub struct ButtonWidget {
    base: WidgetBase,
    text: String,
    state: ButtonState,
    on_click: Option<Box<dyn Fn()>>,
}

impl ButtonWidget {
    pub fn new(id: WidgetID, text: impl Into<String>) -> Self {
        Self {
            base: WidgetBase::new(id, WidgetType::Button),
            text: text.into(),
            state: ButtonState::Normal,
            on_click: None,
        }
    }

    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl Widget for ButtonWidget {
    fn get_id(&self) -> WidgetID {
        self.base.id
    }
    fn get_type(&self) -> WidgetType {
        self.base.widget_type
    }
    fn get_rect(&self) -> &Rect {
        &self.base.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.base.rect = rect;
    }

    fn calculate_size(&mut self) -> SizeConstraints {
        // Base button size on text length
        let text_width = self.text.len() as u32 * 8 + 20;
        SizeConstraints {
            min_width: text_width.max(60),
            min_height: 24,
            preferred_width: text_width.max(80),
            preferred_height: 28,
            ..Default::default()
        }
    }

    fn draw(&self, gfx: &mut GfxContext) {
        if !self.base.visible {
            return;
        }

        let state = if !self.base.enabled {
            ButtonState::Disabled
        } else {
            self.state
        };

        let _ = gfx.draw_button(self.base.rect, &self.text, state, None);
    }

    fn on_click(&mut self, x: i32, y: i32, _button: MouseButton) -> bool {
        if !self.base.enabled || !self.base.visible {
            return false;
        }
        if !self.base.rect.contains_point(x, y) {
            return false;
        }

        // Execute callback if set
        if let Some(ref callback) = self.on_click {
            callback();
        }

        // Briefly show pressed state
        self.state = ButtonState::Pressed;
        true
    }

    fn on_mouse_move(&mut self, x: i32, y: i32) -> bool {
        if !self.base.enabled || !self.base.visible {
            return false;
        }

        let was_hover = self.state == ButtonState::Hover;
        let is_hover = self.contains_point(x, y);

        if is_hover && !was_hover {
            self.state = ButtonState::Hover;
            return true;
        } else if !is_hover && was_hover {
            self.state = ButtonState::Normal;
            return true;
        }

        false
    }

    fn set_visible(&mut self, visible: bool) {
        self.base.visible = visible;
    }
    fn is_visible(&self) -> bool {
        self.base.visible
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.base.enabled = enabled;
    }
    fn is_enabled(&self) -> bool {
        self.base.enabled
    }
}

// Label Widget
pub struct LabelWidget {
    base: WidgetBase,
    text: String,
    colour: Colour,
    alignment: Alignment,
}

impl LabelWidget {
    pub fn new(id: WidgetID, text: impl Into<String>) -> Self {
        Self {
            base: WidgetBase::new(id, WidgetType::Label),
            text: text.into(),
            colour: Colour::ui_text(),
            alignment: Alignment::Left,
        }
    }

    pub fn with_colour(mut self, colour: Colour) -> Self {
        self.colour = colour;
        self
    }

    pub fn with_alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }
}

impl Widget for LabelWidget {
    fn get_id(&self) -> WidgetID {
        self.base.id
    }
    fn get_type(&self) -> WidgetType {
        self.base.widget_type
    }
    fn get_rect(&self) -> &Rect {
        &self.base.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.base.rect = rect;
    }

    fn calculate_size(&mut self) -> SizeConstraints {
        let text_width = self.text.len() as u32 * 8;
        SizeConstraints {
            min_width: text_width,
            min_height: 14,
            preferred_width: text_width,
            preferred_height: 16,
            ..Default::default()
        }
    }

    fn draw(&self, gfx: &mut GfxContext) {
        if !self.base.visible {
            return;
        }

        // Calculate text position based on alignment
        let x = match self.alignment {
            Alignment::Left => self.base.rect.x + 2,
            Alignment::Center => {
                self.base.rect.x + (self.base.rect.width as i32 - self.text.len() as i32 * 8) / 2
            }
            Alignment::Right => {
                self.base.rect.x + self.base.rect.width as i32 - self.text.len() as i32 * 8 - 2
            }
        };

        let y = self.base.rect.y + (self.base.rect.height as i32 - 14) / 2;

        // Draw text
        let _ = gfx.draw_text(&self.text, x, y, self.colour, None);
    }

    fn set_visible(&mut self, visible: bool) {
        self.base.visible = visible;
    }
    fn is_visible(&self) -> bool {
        self.base.visible
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.base.enabled = enabled;
    }
    fn is_enabled(&self) -> bool {
        self.base.enabled
    }
}

// Container widgets for layout
pub struct ContainerWidget {
    base: WidgetBase,
    children: Vec<Box<dyn Widget>>,
    padding: u32,
    spacing: u32,
}

impl ContainerWidget {
    pub fn new_horizontal(id: WidgetID) -> Self {
        Self {
            base: WidgetBase::new(id, WidgetType::Horizontal),
            children: Vec::new(),
            padding: 4,
            spacing: 4,
        }
    }

    pub fn new_vertical(id: WidgetID) -> Self {
        Self {
            base: WidgetBase::new(id, WidgetType::Vertical),
            children: Vec::new(),
            padding: 4,
            spacing: 4,
        }
    }

    pub fn add_child(&mut self, widget: Box<dyn Widget>) {
        self.children.push(widget);
    }

    pub fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }

    fn layout_children(&mut self) {
        if self.children.is_empty() {
            return;
        }

        let mut x = self.base.rect.x + self.padding as i32;
        let mut y = self.base.rect.y + self.padding as i32;

        let available_width = self.base.rect.width - 2 * self.padding;
        let available_height = self.base.rect.height - 2 * self.padding;

        if self.base.widget_type == WidgetType::Horizontal {
            // Horizontal layout
            let child_width = (available_width - (self.children.len() - 1) as u32 * self.spacing)
                / self.children.len() as u32;

            for child in &mut self.children {
                child.set_rect(Rect {
                    x,
                    y,
                    width: child_width,
                    height: available_height,
                });
                x += child_width as i32 + self.spacing as i32;
            }
        } else {
            // Vertical layout
            let child_height = (available_height - (self.children.len() - 1) as u32 * self.spacing)
                / self.children.len() as u32;

            for child in &mut self.children {
                child.set_rect(Rect {
                    x,
                    y,
                    width: available_width,
                    height: child_height,
                });
                y += child_height as i32 + self.spacing as i32;
            }
        }
    }
}

impl Widget for ContainerWidget {
    fn get_id(&self) -> WidgetID {
        self.base.id
    }
    fn get_type(&self) -> WidgetType {
        self.base.widget_type
    }
    fn get_rect(&self) -> &Rect {
        &self.base.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.base.rect = rect;
        self.layout_children();
    }

    fn calculate_size(&mut self) -> SizeConstraints {
        let mut total_width = 2 * self.padding;
        let mut total_height = 2 * self.padding;
        let mut max_width = 0u32;
        let mut max_height = 0u32;

        for child in &mut self.children {
            let child_size = child.calculate_size();

            if self.base.widget_type == WidgetType::Horizontal {
                total_width += child_size.preferred_width + self.spacing;
                max_height = max_height.max(child_size.preferred_height);
            } else {
                total_height += child_size.preferred_height + self.spacing;
                max_width = max_width.max(child_size.preferred_width);
            }
        }

        // Remove last spacing
        if !self.children.is_empty() {
            if self.base.widget_type == WidgetType::Horizontal {
                total_width -= self.spacing;
                total_height += max_height;
            } else {
                total_height -= self.spacing;
                total_width += max_width;
            }
        }

        SizeConstraints {
            min_width: total_width,
            min_height: total_height,
            preferred_width: total_width,
            preferred_height: total_height,
            ..Default::default()
        }
    }

    fn draw(&self, gfx: &mut GfxContext) {
        if !self.base.visible {
            return;
        }

        // Draw all children
        for child in &self.children {
            child.draw(gfx);
        }
    }

    fn on_click(&mut self, x: i32, y: i32, button: MouseButton) -> bool {
        if !self.base.enabled || !self.base.visible {
            return false;
        }

        // Forward to children
        for child in &mut self.children {
            if child.on_click(x, y, button) {
                return true;
            }
        }
        false
    }

    fn on_mouse_move(&mut self, x: i32, y: i32) -> bool {
        if !self.base.enabled || !self.base.visible {
            return false;
        }

        let mut handled = false;
        for child in &mut self.children {
            if child.on_mouse_move(x, y) {
                handled = true;
            }
        }
        handled
    }

    fn set_visible(&mut self, visible: bool) {
        self.base.visible = visible;
    }
    fn is_visible(&self) -> bool {
        self.base.visible
    }
    fn set_enabled(&mut self, enabled: bool) {
        self.base.enabled = enabled;
    }
    fn is_enabled(&self) -> bool {
        self.base.enabled
    }
}

/// Window class - Top-level container for widgets
pub struct Window {
    pub id: WindowID,
    pub title: String,
    pub rect: Rect,
    pub visible: bool,
    pub closeable: bool,
    pub resizable: bool,
    pub modal: bool,

    root_widget: Option<Box<dyn Widget>>,
    widgets: HashMap<WidgetID, Rc<RefCell<dyn Widget>>>,
    dirty: bool,
    last_clicked_widget: Option<WidgetID>,
}

impl Window {
    pub fn new(id: WindowID, title: impl Into<String>, rect: Rect) -> Self {
        Self {
            id,
            title: title.into(),
            rect,
            visible: true,
            closeable: true,
            resizable: false,
            modal: false,
            root_widget: None,
            widgets: HashMap::new(),
            dirty: true,
            last_clicked_widget: None,
        }
    }

    /// Set the root widget for this window
    pub fn set_root_widget(&mut self, mut widget: Box<dyn Widget>) {
        widget.set_rect(Rect {
            x: self.rect.x,
            y: self.rect.y + 20, // Leave space for title bar
            width: self.rect.width,
            height: self.rect.height - 20,
        });
        self.root_widget = Some(widget);
        self.dirty = true;
    }

    /// Draw the window and all its widgets
    pub fn draw(&self, gfx: &mut GfxContext) {
        if !self.visible {
            return;
        }

        // Draw window background
        let _ = gfx.fill_rect(self.rect, Colour::ui_window_background());

        // Draw title bar
        let title_rect = Rect {
            x: self.rect.x,
            y: self.rect.y,
            width: self.rect.width,
            height: 20,
        };
        let _ = gfx.fill_rect(title_rect, Colour::ui_title_bar());

        // Draw title text (placeholder)
        let text_rect = Rect {
            x: self.rect.x + 4,
            y: self.rect.y + 3,
            width: (self.title.len() * 8) as u32,
            height: 14,
        };
        let _ = gfx.draw_rect(text_rect, Colour::ui_title_text());

        // Draw close button if closeable
        if self.closeable {
            let close_rect = Rect {
                x: self.rect.x + self.rect.width as i32 - 18,
                y: self.rect.y + 2,
                width: 16,
                height: 16,
            };
            let _ = gfx.draw_button(close_rect, "X", ButtonState::Normal, None);
        }

        // Draw window border
        let _ = gfx.draw_rect(self.rect, Colour::ui_border());

        // Draw root widget if present
        if let Some(ref widget) = self.root_widget {
            widget.draw(gfx);
        }
    }

    /// Handle mouse click events
    pub fn on_click(&mut self, x: i32, y: i32, button: MouseButton) -> bool {
        if !self.visible {
            return false;
        }
        if !self.rect.contains_point(x, y) {
            return false;
        }

        // Check close button
        if self.closeable {
            let close_rect = Rect {
                x: self.rect.x + self.rect.width as i32 - 18,
                y: self.rect.y + 2,
                width: 16,
                height: 16,
            };
            if close_rect.contains_point(x, y) {
                self.visible = false;
                return true;
            }
        }

        // Forward to root widget
        if let Some(ref mut widget) = self.root_widget {
            return widget.on_click(x, y, button);
        }

        false
    }

    /// Handle mouse movement
    pub fn on_mouse_move(&mut self, x: i32, y: i32) -> bool {
        if !self.visible {
            return false;
        }

        if let Some(ref mut widget) = self.root_widget {
            return widget.on_mouse_move(x, y);
        }

        false
    }

    /// Mark window as needing redraw
    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    /// Check if window needs redraw
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Clear dirty flag
    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

/// Window Manager - Manages all windows in the application
pub struct WindowManager {
    windows: Vec<Window>,
    focused_window: Option<usize>,
    next_window_id: WindowID,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: Vec::new(),
            focused_window: None,
            next_window_id: 1,
        }
    }

    /// Add a new window
    pub fn add_window(&mut self, mut window: Window) -> WindowID {
        if window.id == 0 {
            window.id = self.next_window_id;
            self.next_window_id += 1;
        }

        let id = window.id;
        self.windows.push(window);
        self.focused_window = Some(self.windows.len() - 1);
        id
    }

    /// Get the number of windows
    pub fn window_count(&self) -> usize {
        self.windows.len()
    }

    /// Remove a window by ID
    pub fn remove_window(&mut self, id: WindowID) -> Result<()> {
        if let Some(pos) = self.windows.iter().position(|w| w.id == id) {
            self.windows.remove(pos);
            if self.focused_window == Some(pos) {
                self.focused_window = if self.windows.is_empty() {
                    None
                } else {
                    Some(self.windows.len() - 1)
                };
            }
            Ok(())
        } else {
            Err(GuiError::WindowNotFound(id))
        }
    }

    /// Get a window by ID
    pub fn get_window(&mut self, id: WindowID) -> Option<&mut Window> {
        self.windows.iter_mut().find(|w| w.id == id)
    }

    /// Draw all windows
    pub fn draw_all(&self, gfx: &mut GfxContext) {
        // Draw non-modal windows first
        for window in &self.windows {
            if !window.modal {
                window.draw(gfx);
            }
        }

        // Draw modal windows on top
        for window in &self.windows {
            if window.modal {
                window.draw(gfx);
            }
        }
    }

    /// Handle mouse click events
    pub fn on_click(&mut self, x: i32, y: i32, button: MouseButton) -> bool {
        // Check windows from top to bottom
        for (i, window) in self.windows.iter_mut().enumerate().rev() {
            if window.on_click(x, y, button) {
                self.focused_window = Some(i);
                return true;
            }
        }
        false
    }

    /// Handle mouse movement
    pub fn on_mouse_move(&mut self, x: i32, y: i32) -> bool {
        // Forward to all windows
        let mut handled = false;
        for window in &mut self.windows {
            if window.on_mouse_move(x, y) {
                handled = true;
            }
        }
        handled
    }

    /// Process SDL events
    pub fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            Event::MouseButtonDown {
                x, y, mouse_btn, ..
            } => self.on_click(*x, *y, *mouse_btn),
            Event::MouseMotion { x, y, .. } => self.on_mouse_move(*x, *y),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings_graphics::{GraphicsSettingsWidgets, GraphicsSettingsWindow};

    #[test]
    fn test_widget_base() {
        let widget = WidgetBase::new(1, WidgetType::Panel);
        assert_eq!(widget.id, 1);
        assert_eq!(widget.widget_type, WidgetType::Panel);
        assert!(widget.visible);
        assert!(widget.enabled);
    }

    #[test]
    fn test_panel_widget() {
        let panel = PanelWidget::new(1)
            .with_colour(Colour::rgba(100, 100, 100, 255))
            .with_border(false);

        assert_eq!(panel.get_id(), 1);
        assert_eq!(panel.get_type(), WidgetType::Panel);
        assert!(panel.is_visible());
        assert!(panel.is_enabled());
    }

    #[test]
    fn test_button_widget() {
        let mut button = ButtonWidget::new(2, "Click Me");
        assert_eq!(button.get_id(), 2);
        assert_eq!(button.get_type(), WidgetType::Button);
        assert_eq!(button.text, "Click Me");

        // Test click handling
        button.set_rect(Rect {
            x: 10,
            y: 10,
            width: 80,
            height: 30,
        });
        assert!(button.on_click(20, 20, MouseButton::Left));
        assert!(!button.on_click(100, 100, MouseButton::Left));
    }

    #[test]
    fn test_window_manager() {
        let mut manager = WindowManager::new();

        let window = Window::new(
            0,
            "Test Window",
            Rect {
                x: 100,
                y: 100,
                width: 400,
                height: 300,
            },
        );
        let id = manager.add_window(window);

        assert!(manager.get_window(id).is_some());
        assert!(manager.remove_window(id).is_ok());
        assert!(manager.get_window(id).is_none());
    }

    #[test]
    fn test_container_layout() {
        let mut container = ContainerWidget::new_horizontal(1);
        container.add_child(Box::new(ButtonWidget::new(2, "Button 1")));
        container.add_child(Box::new(ButtonWidget::new(3, "Button 2")));

        let size = container.calculate_size();
        assert!(size.min_width > 0);
        assert!(size.min_height > 0);
    }

    #[test]
    fn test_graphics_settings_toggle_and_dropdown() {
        let mut window = GraphicsSettingsWindow::new();
        let rect = Rect::new(140, 80, 520, 420);

        let fullscreen_click = GraphicsSettingsWindow::fullscreen_rect();
        assert!(window
            .handle_click(fullscreen_click.x + 1, fullscreen_click.y + 1, rect)
            .is_some());
        assert!(window.fullscreen_enabled());

        let resolution_click = GraphicsSettingsWindow::resolution_rect();
        assert!(window
            .handle_click(resolution_click.x + 1, resolution_click.y + 1, rect)
            .is_some());
    }

    #[test]
    fn test_graphics_settings_window_contains_widgets() {
        let window = GraphicsSettingsWindow::new().build_window();
        assert_eq!(window.id, GraphicsSettingsWidgets::Window as WidgetID);
        assert_eq!(window.title, "Graphics Settings");
    }
}
