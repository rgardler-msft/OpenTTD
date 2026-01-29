//! Graphics settings window

use crate::{
    ButtonWidget, ContainerWidget, LabelWidget, Rect, WidgetID, Window, WindowID, WindowManager,
};

/// Window ID for the graphics settings window
pub const GRAPHICS_SETTINGS_WINDOW_ID: WindowID = 7000;

/// Widget IDs for graphics settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsSettingsWidgets {
    Window = 7000,
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

#[derive(Debug, Clone)]
struct GraphicsSettingsState {
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
        }
    }
}

pub enum GraphicsSettingsAction {
    None,
    Close,
}

const WINDOW_X: i32 = 140;
const WINDOW_Y: i32 = 80;
const WINDOW_WIDTH: u32 = 520;
const WINDOW_HEIGHT: u32 = 420;
const TITLE_BAR_HEIGHT: i32 = 20;
const ROW_SPACING: i32 = 10;
const SECTION_PADDING: i32 = 0;
const SECTION_SPACING: i32 = 6;

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

    #[cfg(test)]
    pub fn is_dropdown_open(&self) -> bool {
        false
    }

    #[cfg(test)]
    pub fn resolution_rect() -> Rect {
        resolution_rect()
    }

    #[cfg(test)]
    pub fn fullscreen_rect() -> Rect {
        fullscreen_rect()
    }

    /// Convert to a Window for the window manager
    pub fn build_window(&self) -> Window {
        let mut window = Window::new(
            GRAPHICS_SETTINGS_WINDOW_ID,
            "Graphics Settings",
            Rect::new(WINDOW_X, WINDOW_Y, WINDOW_WIDTH, WINDOW_HEIGHT),
        );

        let mut main_container = ContainerWidget::new_vertical(7100)
            .with_padding(12)
            .with_spacing(10);

        // Display section
        let mut display_section = ContainerWidget::new_vertical(7101).with_spacing(6);
        display_section.add_child(Box::new(
            LabelWidget::new(
                GraphicsSettingsWidgets::DisplaySectionLabel as WidgetID,
                "Display Settings",
            )
            .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        display_section.add_child(Box::new(Self::value_button(
            GraphicsSettingsWidgets::ResolutionDropdown,
            &format!(
                "Resolution: {}",
                self.state.resolutions[self.state.selected_resolution]
            ),
        )));
        display_section.add_child(Box::new(Self::value_button(
            GraphicsSettingsWidgets::FullscreenToggle,
            &format!(
                "Fullscreen: {}",
                if self.state.fullscreen { "On" } else { "Off" }
            ),
        )));
        display_section.add_child(Box::new(Self::value_button(
            GraphicsSettingsWidgets::VsyncToggle,
            &format!("VSync: {}", if self.state.vsync { "On" } else { "Off" }),
        )));

        // Interface section
        let mut interface_section = ContainerWidget::new_vertical(7102).with_spacing(6);
        interface_section.add_child(Box::new(
            LabelWidget::new(
                GraphicsSettingsWidgets::InterfaceSectionLabel as WidgetID,
                "Interface Settings",
            )
            .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        interface_section.add_child(Box::new(Self::value_button(
            GraphicsSettingsWidgets::GuiScaleDropdown,
            &format!(
                "GUI Scale: {}",
                self.state.gui_scales[self.state.selected_gui_scale]
            ),
        )));
        interface_section.add_child(Box::new(Self::value_button(
            GraphicsSettingsWidgets::FontDropdown,
            &format!("Font: {}", self.state.fonts[self.state.selected_font]),
        )));

        // Base graphics section
        let mut graphics_section = ContainerWidget::new_vertical(7103).with_spacing(6);
        graphics_section.add_child(Box::new(
            LabelWidget::new(
                GraphicsSettingsWidgets::GraphicsSectionLabel as WidgetID,
                "Graphics Set",
            )
            .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        graphics_section.add_child(Box::new(Self::value_button(
            GraphicsSettingsWidgets::BaseGraphicsDropdown,
            &format!(
                "Base Graphics: {}",
                self.state.base_graphics[self.state.selected_base_graphics]
            ),
        )));

        // Action buttons
        let mut actions = ContainerWidget::new_horizontal(7104)
            .with_spacing(10)
            .with_padding(0);
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

    pub fn handle_click(&mut self, x: i32, y: i32, rect: Rect) -> Option<GraphicsSettingsAction> {
        if !rect.contains_point(x, y) {
            return None;
        }

        let close_rect = Rect::new(rect.x + rect.width as i32 - 18, rect.y + 2, 16, 16);
        if close_rect.contains_point(x, y) {
            println!("Graphics settings closed (not yet applied)");
            return Some(GraphicsSettingsAction::Close);
        }

        if resolution_rect().contains_point(x, y) {
            self.state.selected_resolution =
                (self.state.selected_resolution + 1) % self.state.resolutions.len();
            println!(
                "Resolution set to {} (not yet applied)",
                self.state.resolutions[self.state.selected_resolution]
            );
            return Some(GraphicsSettingsAction::None);
        }

        if fullscreen_rect().contains_point(x, y) {
            self.state.fullscreen = !self.state.fullscreen;
            println!(
                "Fullscreen toggled {} (not yet applied)",
                if self.state.fullscreen { "on" } else { "off" }
            );
            return Some(GraphicsSettingsAction::None);
        }

        if vsync_rect().contains_point(x, y) {
            self.state.vsync = !self.state.vsync;
            println!(
                "VSync toggled {} (not yet applied)",
                if self.state.vsync { "on" } else { "off" }
            );
            return Some(GraphicsSettingsAction::None);
        }

        if gui_scale_rect().contains_point(x, y) {
            self.state.selected_gui_scale =
                (self.state.selected_gui_scale + 1) % self.state.gui_scales.len();
            println!(
                "GUI scale set to {} (not yet applied)",
                self.state.gui_scales[self.state.selected_gui_scale]
            );
            return Some(GraphicsSettingsAction::None);
        }

        if font_rect().contains_point(x, y) {
            self.state.selected_font = (self.state.selected_font + 1) % self.state.fonts.len();
            println!(
                "Font set to {} (not yet applied)",
                self.state.fonts[self.state.selected_font]
            );
            return Some(GraphicsSettingsAction::None);
        }

        if base_graphics_rect().contains_point(x, y) {
            self.state.selected_base_graphics =
                (self.state.selected_base_graphics + 1) % self.state.base_graphics.len();
            println!(
                "Base graphics set to {} (not yet applied)",
                self.state.base_graphics[self.state.selected_base_graphics]
            );
            return Some(GraphicsSettingsAction::None);
        }

        match action_button_at(x, y) {
            Some(GraphicsSettingsWidgets::ApplyButton) => {
                println!("Apply clicked (not yet applied)");
                Some(GraphicsSettingsAction::Close)
            }
            Some(GraphicsSettingsWidgets::CancelButton) => {
                println!("Cancel clicked (no changes applied)");
                Some(GraphicsSettingsAction::Close)
            }
            _ => Some(GraphicsSettingsAction::None),
        }
    }

    fn value_button(id: GraphicsSettingsWidgets, label: &str) -> ButtonWidget {
        ButtonWidget::new(id as WidgetID, label)
    }
}

/// Create and show the graphics settings window
pub fn show_graphics_settings(
    window_manager: &mut WindowManager,
    settings: &GraphicsSettingsWindow,
) -> WindowID {
    let window = settings.build_window();
    window_manager.add_window(window)
}

fn layout_sections() -> [Rect; 4] {
    let content_rect = Rect::new(
        WINDOW_X,
        WINDOW_Y + TITLE_BAR_HEIGHT,
        WINDOW_WIDTH,
        WINDOW_HEIGHT - TITLE_BAR_HEIGHT as u32,
    );

    let padding = 12;
    let spacing = ROW_SPACING;
    let count = 4;
    let available_height = content_rect.height as i32 - 2 * padding;
    let child_height = (available_height - (count - 1) * spacing) / count;
    let mut rects = [Rect::new(0, 0, 0, 0); 4];
    for i in 0..count {
        let y = content_rect.y + padding + i * (child_height + spacing);
        rects[i as usize] = Rect::new(
            content_rect.x + padding,
            y,
            content_rect.width - (2 * padding) as u32,
            child_height as u32,
        );
    }
    rects
}

fn layout_vertical_children(parent: Rect, count: i32) -> Vec<Rect> {
    let available_height = parent.height as i32 - 2 * SECTION_PADDING;
    let child_height = (available_height - (count - 1) * SECTION_SPACING) / count;
    let mut rects = Vec::new();
    for i in 0..count {
        let y = parent.y + SECTION_PADDING + i * (child_height + SECTION_SPACING);
        rects.push(Rect::new(
            parent.x + SECTION_PADDING,
            y,
            parent.width - (2 * SECTION_PADDING) as u32,
            child_height as u32,
        ));
    }
    rects
}

fn resolution_rect() -> Rect {
    let sections = layout_sections();
    let display_children = layout_vertical_children(sections[0], 4);
    display_children[1]
}

fn fullscreen_rect() -> Rect {
    let sections = layout_sections();
    let display_children = layout_vertical_children(sections[0], 4);
    display_children[2]
}

fn vsync_rect() -> Rect {
    let sections = layout_sections();
    let display_children = layout_vertical_children(sections[0], 4);
    display_children[3]
}

fn gui_scale_rect() -> Rect {
    let sections = layout_sections();
    let interface_children = layout_vertical_children(sections[1], 3);
    interface_children[1]
}

fn font_rect() -> Rect {
    let sections = layout_sections();
    let interface_children = layout_vertical_children(sections[1], 3);
    interface_children[2]
}

fn base_graphics_rect() -> Rect {
    let sections = layout_sections();
    let graphics_children = layout_vertical_children(sections[2], 2);
    graphics_children[1]
}

fn action_button_at(x: i32, y: i32) -> Option<GraphicsSettingsWidgets> {
    let sections = layout_sections();
    let actions_rect = sections[3];
    if !actions_rect.contains_point(x, y) {
        return None;
    }

    let spacing = 10;
    let available_width = actions_rect.width as i32;
    let button_width = (available_width - spacing) / 2;
    let left_rect = Rect::new(
        actions_rect.x,
        actions_rect.y,
        button_width as u32,
        actions_rect.height,
    );
    let right_rect = Rect::new(
        actions_rect.x + button_width + spacing,
        actions_rect.y,
        button_width as u32,
        actions_rect.height,
    );

    if left_rect.contains_point(x, y) {
        Some(GraphicsSettingsWidgets::ApplyButton)
    } else if right_rect.contains_point(x, y) {
        Some(GraphicsSettingsWidgets::CancelButton)
    } else {
        None
    }
}
