//! Graphics settings window

use crate::{
    ButtonWidget, ContainerWidget, LabelWidget, PanelWidget, Rect, Widget, WidgetID, Window,
    WindowID, WindowManager,
};
use openttd_gfx::{Colour, GfxContext};

/// Window ID for the graphics settings window
pub const GRAPHICS_SETTINGS_WINDOW_ID: WindowID = 7000;

/// Widget IDs for graphics settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsSettingsWidgets {
    Window = 7000,
    Panel = 7001,

    DisplaySectionLabel = 7010,
    ResolutionDropdown = 7011,
    FullscreenToggle = 7012,
    VsyncToggle = 7013,

    InterfaceSectionLabel = 7020,
    GuiScaleDropdown = 7021,
    FontDropdown = 7022,

    GraphicsSectionLabel = 7030,
    BaseGraphicsDropdown = 7031,

    ApplyButton = 7040,
    CancelButton = 7041,
}

impl From<GraphicsSettingsWidgets> for WidgetID {
    fn from(value: GraphicsSettingsWidgets) -> Self {
        value as WidgetID
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GraphicsDropdown {
    Resolution,
    GuiScale,
    Font,
    BaseGraphics,
}

#[derive(Debug, Clone)]
pub struct GraphicsSettingsState {
    resolutions: Vec<&'static str>,
    gui_scales: Vec<&'static str>,
    fonts: Vec<&'static str>,
    base_graphics: Vec<&'static str>,
    selected_resolution: usize,
    selected_gui_scale: usize,
    selected_font: usize,
    selected_base_graphics: usize,
    fullscreen: bool,
    vsync: bool,
    dropdown_open: Option<GraphicsDropdown>,
}

impl Default for GraphicsSettingsState {
    fn default() -> Self {
        Self {
            resolutions: vec!["640x480", "800x600", "1024x768", "1280x720"],
            gui_scales: vec!["100%", "125%", "150%", "200%"],
            fonts: vec!["Default", "Large", "Mono"],
            base_graphics: vec!["OpenGFX", "Original", "ZBase"],
            selected_resolution: 1,
            selected_gui_scale: 0,
            selected_font: 0,
            selected_base_graphics: 0,
            fullscreen: false,
            vsync: true,
            dropdown_open: None,
        }
    }
}

/// Graphics settings window controller
pub struct GraphicsSettingsWindow {
    state: GraphicsSettingsState,
}

impl GraphicsSettingsWindow {
    pub fn new() -> Self {
        Self {
            state: GraphicsSettingsState::default(),
        }
    }

    pub fn fullscreen_enabled(&self) -> bool {
        self.state.fullscreen
    }

    pub fn dropdown_open(&self) -> bool {
        self.state.dropdown_open.is_some()
    }

    #[cfg(test)]
    pub fn is_dropdown_open(&self) -> bool {
        self.state.dropdown_open.is_some()
    }

    /// Convert to a Window for the window manager
    pub fn into_window(self) -> Window {
        let mut window = Window::new(
            GRAPHICS_SETTINGS_WINDOW_ID,
            "Graphics Settings",
            Rect::new(140, 80, 520, 420),
        );

        let panel = PanelWidget::new(GraphicsSettingsWidgets::Panel as WidgetID)
            .with_colour(Colour::ui_background());

        let mut main_container = ContainerWidget::new_vertical(7100)
            .with_padding(12)
            .with_spacing(10);

        main_container.add_child(Box::new(panel));

        // Display section
        let mut display_section = ContainerWidget::new_vertical(7101).with_spacing(6);
        display_section.add_child(Box::new(
            LabelWidget::new(
                GraphicsSettingsWidgets::DisplaySectionLabel as WidgetID,
                "Display Settings",
            )
            .with_colour(Colour::ui_highlight()),
        ));

        display_section.add_child(Box::new(Self::dropdown_stub(
            GraphicsSettingsWidgets::ResolutionDropdown,
            "Resolution",
        )));
        display_section.add_child(Box::new(Self::toggle_stub(
            GraphicsSettingsWidgets::FullscreenToggle,
            "Fullscreen",
        )));
        display_section.add_child(Box::new(Self::toggle_stub(
            GraphicsSettingsWidgets::VsyncToggle,
            "VSync",
        )));

        // Interface section
        let mut interface_section = ContainerWidget::new_vertical(7102).with_spacing(6);
        interface_section.add_child(Box::new(
            LabelWidget::new(
                GraphicsSettingsWidgets::InterfaceSectionLabel as WidgetID,
                "Interface Settings",
            )
            .with_colour(Colour::ui_highlight()),
        ));
        interface_section.add_child(Box::new(Self::dropdown_stub(
            GraphicsSettingsWidgets::GuiScaleDropdown,
            "GUI Scale",
        )));
        interface_section.add_child(Box::new(Self::dropdown_stub(
            GraphicsSettingsWidgets::FontDropdown,
            "Font",
        )));

        // Base graphics section
        let mut graphics_section = ContainerWidget::new_vertical(7103).with_spacing(6);
        graphics_section.add_child(Box::new(
            LabelWidget::new(
                GraphicsSettingsWidgets::GraphicsSectionLabel as WidgetID,
                "Graphics Set",
            )
            .with_colour(Colour::ui_highlight()),
        ));
        graphics_section.add_child(Box::new(Self::dropdown_stub(
            GraphicsSettingsWidgets::BaseGraphicsDropdown,
            "Base Graphics",
        )));

        // Action buttons
        let mut actions = ContainerWidget::new_horizontal(7104).with_spacing(10);
        actions.add_child(Box::new(ButtonWidget::new(
            GraphicsSettingsWidgets::ApplyButton as WidgetID,
            "Apply",
        )));
        actions.add_child(Box::new(ButtonWidget::new(
            GraphicsSettingsWidgets::CancelButton as WidgetID,
            "Cancel",
        )));

        main_container.add_child(Box::new(display_section));
        main_container.add_child(Box::new(interface_section));
        main_container.add_child(Box::new(graphics_section));
        main_container.add_child(Box::new(actions));

        window.set_root_widget(Box::new(main_container));
        window
    }

    pub fn draw(&self, gfx: &mut GfxContext, rect: Rect) {
        gfx.fill_rect(rect, Colour::ui_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();

        let title_rect = Rect::new(rect.x, rect.y, rect.width, 30);
        gfx.fill_rect(title_rect, Colour::ui_window_background())
            .ok();
        gfx.draw_text(
            "GRAPHICS SETTINGS",
            rect.x + 10,
            rect.y + 10,
            Colour::ui_text(),
            None,
        )
        .ok();

        let content_x = rect.x + 20;
        let mut y = rect.y + 50;
        let content_width = rect.width - 40;

        y = self.draw_dropdown_row(
            gfx,
            content_x,
            y,
            content_width,
            "Resolution",
            self.state.resolutions[self.state.selected_resolution],
            self.state.dropdown_open == Some(GraphicsDropdown::Resolution),
        );

        y = self.draw_toggle_row(
            gfx,
            content_x,
            y,
            content_width,
            "Fullscreen",
            self.state.fullscreen,
        );
        y = self.draw_toggle_row(gfx, content_x, y, content_width, "VSync", self.state.vsync);

        y += 10;
        y = self.draw_dropdown_row(
            gfx,
            content_x,
            y,
            content_width,
            "GUI Scale",
            self.state.gui_scales[self.state.selected_gui_scale],
            self.state.dropdown_open == Some(GraphicsDropdown::GuiScale),
        );
        y = self.draw_dropdown_row(
            gfx,
            content_x,
            y,
            content_width,
            "Font",
            self.state.fonts[self.state.selected_font],
            self.state.dropdown_open == Some(GraphicsDropdown::Font),
        );

        y += 10;
        y = self.draw_dropdown_row(
            gfx,
            content_x,
            y,
            content_width,
            "Base Graphics",
            self.state.base_graphics[self.state.selected_base_graphics],
            self.state.dropdown_open == Some(GraphicsDropdown::BaseGraphics),
        );

        // Draw dropdown list if open
        if let Some(open) = self.state.dropdown_open {
            let options = match open {
                GraphicsDropdown::Resolution => &self.state.resolutions,
                GraphicsDropdown::GuiScale => &self.state.gui_scales,
                GraphicsDropdown::Font => &self.state.fonts,
                GraphicsDropdown::BaseGraphics => &self.state.base_graphics,
            };
            self.draw_dropdown_list(
                gfx,
                Rect::new(content_x, y + 4, content_width, 120),
                options,
            );
        }
    }

    pub fn handle_click(&mut self, x: i32, y: i32, rect: Rect) -> bool {
        if !rect.contains_point(x, y) {
            return false;
        }

        let content_x = rect.x + 20;
        let content_width = rect.width - 40;
        let mut cursor_y = rect.y + 50;

        let resolution_rect = Rect::new(content_x, cursor_y, content_width, 28);
        cursor_y += 28;
        let fullscreen_rect = Rect::new(content_x, cursor_y, content_width, 24);
        cursor_y += 24;
        let vsync_rect = Rect::new(content_x, cursor_y, content_width, 24);
        cursor_y += 24 + 10;
        let gui_scale_rect = Rect::new(content_x, cursor_y, content_width, 28);
        cursor_y += 28;
        let font_rect = Rect::new(content_x, cursor_y, content_width, 28);
        cursor_y += 28 + 10;
        let base_graphics_rect = Rect::new(content_x, cursor_y, content_width, 28);

        if resolution_rect.contains_point(x, y) {
            self.toggle_dropdown(GraphicsDropdown::Resolution);
            return true;
        }
        if gui_scale_rect.contains_point(x, y) {
            self.toggle_dropdown(GraphicsDropdown::GuiScale);
            return true;
        }
        if font_rect.contains_point(x, y) {
            self.toggle_dropdown(GraphicsDropdown::Font);
            return true;
        }
        if base_graphics_rect.contains_point(x, y) {
            self.toggle_dropdown(GraphicsDropdown::BaseGraphics);
            return true;
        }
        if fullscreen_rect.contains_point(x, y) {
            self.state.fullscreen = !self.state.fullscreen;
            return true;
        }
        if vsync_rect.contains_point(x, y) {
            self.state.vsync = !self.state.vsync;
            return true;
        }

        if let Some(open) = self.state.dropdown_open {
            if self.handle_dropdown_selection(open, x, y, content_x, cursor_y + 32, content_width) {
                return true;
            }
            self.state.dropdown_open = None;
            return true;
        }

        false
    }

    fn handle_dropdown_selection(
        &mut self,
        open: GraphicsDropdown,
        x: i32,
        y: i32,
        list_x: i32,
        list_y: i32,
        list_width: u32,
    ) -> bool {
        let list_height = 120;
        let list_rect = Rect::new(list_x, list_y, list_width, list_height);
        if !list_rect.contains_point(x, y) {
            return false;
        }
        let index = ((y - list_y) / 20).clamp(0, 3) as usize;
        match open {
            GraphicsDropdown::Resolution => {
                self.state.selected_resolution = index.min(self.state.resolutions.len() - 1);
            }
            GraphicsDropdown::GuiScale => {
                self.state.selected_gui_scale = index.min(self.state.gui_scales.len() - 1);
            }
            GraphicsDropdown::Font => {
                self.state.selected_font = index.min(self.state.fonts.len() - 1);
            }
            GraphicsDropdown::BaseGraphics => {
                self.state.selected_base_graphics = index.min(self.state.base_graphics.len() - 1);
            }
        }
        self.state.dropdown_open = None;
        true
    }

    fn toggle_dropdown(&mut self, target: GraphicsDropdown) {
        self.state.dropdown_open = if self.state.dropdown_open == Some(target) {
            None
        } else {
            Some(target)
        };
    }

    fn draw_dropdown_row(
        &self,
        gfx: &mut GfxContext,
        x: i32,
        y: i32,
        width: u32,
        label: &str,
        value: &str,
        open: bool,
    ) -> i32 {
        let label_rect = Rect::new(x, y, width, 16);
        gfx.draw_text(label, label_rect.x, label_rect.y, Colour::ui_text(), None)
            .ok();
        let dropdown_rect = Rect::new(x, y + 16, width, 24);
        gfx.draw_rect(dropdown_rect, Colour::ui_border()).ok();
        gfx.draw_text(
            value,
            dropdown_rect.x + 8,
            dropdown_rect.y + 6,
            Colour::ui_text(),
            None,
        )
        .ok();
        if open {
            gfx.draw_rect(
                Rect::new(
                    dropdown_rect.x + dropdown_rect.width as i32 - 20,
                    dropdown_rect.y,
                    20,
                    24,
                ),
                Colour::ui_highlight(),
            )
            .ok();
        }
        y + 40
    }

    fn draw_toggle_row(
        &self,
        gfx: &mut GfxContext,
        x: i32,
        y: i32,
        width: u32,
        label: &str,
        value: bool,
    ) -> i32 {
        let rect = Rect::new(x, y, width, 24);
        gfx.draw_text(label, rect.x, rect.y + 4, Colour::ui_text(), None)
            .ok();
        let toggle_rect = Rect::new(rect.x + rect.width as i32 - 60, rect.y, 50, 22);
        gfx.draw_rect(toggle_rect, Colour::ui_border()).ok();
        gfx.draw_text(
            if value { "ON" } else { "OFF" },
            toggle_rect.x + 12,
            toggle_rect.y + 4,
            Colour::ui_text(),
            None,
        )
        .ok();
        y + 24
    }

    fn draw_dropdown_list(&self, gfx: &mut GfxContext, rect: Rect, options: &[&str]) {
        gfx.fill_rect(rect, Colour::ui_window_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();
        let mut y = rect.y + 4;
        for option in options.iter().take(4) {
            gfx.draw_text(option, rect.x + 6, y, Colour::ui_text(), None)
                .ok();
            y += 20;
        }
    }

    fn dropdown_stub(id: GraphicsSettingsWidgets, label: &str) -> LabelWidget {
        LabelWidget::new(id as WidgetID, label).with_colour(Colour::ui_text())
    }

    fn toggle_stub(id: GraphicsSettingsWidgets, label: &str) -> LabelWidget {
        LabelWidget::new(id as WidgetID, label).with_colour(Colour::ui_text())
    }
}

/// Create and show the graphics settings window
pub fn show_graphics_settings(window_manager: &mut WindowManager) -> WindowID {
    let window = GraphicsSettingsWindow::new().into_window();
    window_manager.add_window(window)
}
