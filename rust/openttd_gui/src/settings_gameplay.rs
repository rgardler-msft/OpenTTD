//! Gameplay settings window

use crate::{
    ButtonWidget, ContainerWidget, LabelWidget, Rect, WidgetID, Window, WindowID, WindowManager,
};

/// Window ID for the gameplay settings window
pub const GAMEPLAY_SETTINGS_WINDOW_ID: WindowID = 7400;

/// Widget IDs for gameplay settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameplaySettingsWidgets {
    Window = 7400,
    TabsContainer = 7401,
    AudioTab = 7402,
    VideoTab = 7403,
    GameplayTab = 7404,
    AutosaveSectionLabel = 7410,
    AutosaveToggle = 7411,
    AutosaveFrequency = 7412,
    DifficultySectionLabel = 7420,
    DifficultyPreset = 7421,
    BreakdownToggle = 7422,
    ApplyButton = 7430,
    CancelButton = 7431,
}

impl From<GameplaySettingsWidgets> for WidgetID {
    fn from(value: GameplaySettingsWidgets) -> Self {
        value as WidgetID
    }
}

#[derive(Debug, Clone)]
struct GameplaySettingsState {
    autosave_enabled: bool,
    autosave_frequency: u8,
    difficulty_index: usize,
    breakdowns_enabled: bool,
}

impl Default for GameplaySettingsState {
    fn default() -> Self {
        Self {
            autosave_enabled: true,
            autosave_frequency: 10,
            difficulty_index: 1,
            breakdowns_enabled: true,
        }
    }
}

pub enum GameplaySettingsAction {
    None,
    Close,
    OpenAudio,
    OpenVideo,
}

const WINDOW_X: i32 = 140;
const WINDOW_Y: i32 = 80;
const WINDOW_WIDTH: u32 = 520;
const WINDOW_HEIGHT: u32 = 360;
const TITLE_BAR_HEIGHT: i32 = 20;
const TAB_BAR_HEIGHT: i32 = 30;
const ROW_SPACING: i32 = 10;
const SECTION_PADDING: i32 = 0;
const SECTION_SPACING: i32 = 6;

/// Gameplay settings window controller
pub struct GameplaySettingsWindow {
    state: GameplaySettingsState,
}

impl GameplaySettingsWindow {
    pub fn new() -> Self {
        Self {
            state: GameplaySettingsState::default(),
        }
    }

    pub fn autosave_enabled(&self) -> bool {
        self.state.autosave_enabled
    }

    pub fn autosave_frequency(&self) -> u8 {
        self.state.autosave_frequency
    }

    pub fn difficulty_index(&self) -> usize {
        self.state.difficulty_index
    }

    pub fn breakdowns_enabled(&self) -> bool {
        self.state.breakdowns_enabled
    }

    pub fn build_window(&self) -> Window {
        let mut window = Window::new(
            GAMEPLAY_SETTINGS_WINDOW_ID,
            "Gameplay Settings",
            Rect::new(WINDOW_X, WINDOW_Y, WINDOW_WIDTH, WINDOW_HEIGHT),
        );

        let mut main_container = ContainerWidget::new_vertical(7500)
            .with_padding(12)
            .with_spacing(10);

        let mut tabs =
            ContainerWidget::new_horizontal(GameplaySettingsWidgets::TabsContainer as u32)
                .with_spacing(6)
                .with_padding(0);
        tabs.add_child(Box::new(ButtonWidget::new(
            GameplaySettingsWidgets::AudioTab as WidgetID,
            "Audio",
        )));
        tabs.add_child(Box::new(ButtonWidget::new(
            GameplaySettingsWidgets::VideoTab as WidgetID,
            "Video",
        )));
        tabs.add_child(Box::new(
            LabelWidget::new(GameplaySettingsWidgets::GameplayTab as WidgetID, "Gameplay")
                .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        main_container.add_child(Box::new(tabs));

        let mut autosave_section = ContainerWidget::new_vertical(7501).with_spacing(6);
        autosave_section.add_child(Box::new(
            LabelWidget::new(
                GameplaySettingsWidgets::AutosaveSectionLabel as WidgetID,
                "Autosave",
            )
            .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        autosave_section.add_child(Box::new(Self::value_button(
            GameplaySettingsWidgets::AutosaveToggle,
            &format!(
                "Autosave: {}",
                if self.state.autosave_enabled {
                    "On"
                } else {
                    "Off"
                }
            ),
        )));
        autosave_section.add_child(Box::new(Self::value_button(
            GameplaySettingsWidgets::AutosaveFrequency,
            &format!("Frequency: every {} months", self.state.autosave_frequency),
        )));

        let mut difficulty_section = ContainerWidget::new_vertical(7502).with_spacing(6);
        difficulty_section.add_child(Box::new(
            LabelWidget::new(
                GameplaySettingsWidgets::DifficultySectionLabel as WidgetID,
                "Difficulty",
            )
            .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        difficulty_section.add_child(Box::new(Self::value_button(
            GameplaySettingsWidgets::DifficultyPreset,
            &format!("Preset: {}", difficulty_label(self.state.difficulty_index)),
        )));
        difficulty_section.add_child(Box::new(Self::value_button(
            GameplaySettingsWidgets::BreakdownToggle,
            &format!(
                "Vehicle breakdowns: {}",
                if self.state.breakdowns_enabled {
                    "On"
                } else {
                    "Off"
                }
            ),
        )));

        let mut actions = ContainerWidget::new_horizontal(7503)
            .with_spacing(10)
            .with_padding(0);
        actions.add_child(Box::new(ButtonWidget::new(
            GameplaySettingsWidgets::ApplyButton as WidgetID,
            "Apply",
        )));
        actions.add_child(Box::new(ButtonWidget::new(
            GameplaySettingsWidgets::CancelButton as WidgetID,
            "Cancel",
        )));

        main_container.add_child(Box::new(autosave_section));
        main_container.add_child(Box::new(difficulty_section));
        main_container.add_child(Box::new(actions));

        window.set_root_widget(Box::new(main_container));
        window
    }

    pub fn handle_click(&mut self, x: i32, y: i32, rect: Rect) -> Option<GameplaySettingsAction> {
        if !rect.contains_point(x, y) {
            return None;
        }

        let close_rect = Rect::new(rect.x + rect.width as i32 - 18, rect.y + 2, 16, 16);
        if close_rect.contains_point(x, y) {
            println!("Gameplay settings closed (not yet applied)");
            return Some(GameplaySettingsAction::Close);
        }

        let tab_rects = tab_button_rects();
        if tab_rects[0].contains_point(x, y) {
            println!("Switching to audio tab");
            return Some(GameplaySettingsAction::OpenAudio);
        }
        if tab_rects[1].contains_point(x, y) {
            println!("Switching to video tab");
            return Some(GameplaySettingsAction::OpenVideo);
        }

        if autosave_toggle_rect().contains_point(x, y) {
            self.state.autosave_enabled = !self.state.autosave_enabled;
            println!(
                "Autosave toggled {} (not yet applied)",
                if self.state.autosave_enabled {
                    "on"
                } else {
                    "off"
                }
            );
            return Some(GameplaySettingsAction::None);
        }

        if autosave_frequency_rect().contains_point(x, y) {
            self.state.autosave_frequency = next_frequency(self.state.autosave_frequency);
            println!(
                "Autosave frequency set to {} months (not yet applied)",
                self.state.autosave_frequency
            );
            return Some(GameplaySettingsAction::None);
        }

        if difficulty_preset_rect().contains_point(x, y) {
            self.state.difficulty_index =
                (self.state.difficulty_index + 1) % difficulty_labels().len();
            println!(
                "Difficulty preset set to {} (not yet applied)",
                difficulty_label(self.state.difficulty_index)
            );
            return Some(GameplaySettingsAction::None);
        }

        if breakdown_toggle_rect().contains_point(x, y) {
            self.state.breakdowns_enabled = !self.state.breakdowns_enabled;
            println!(
                "Breakdowns toggled {} (not yet applied)",
                if self.state.breakdowns_enabled {
                    "on"
                } else {
                    "off"
                }
            );
            return Some(GameplaySettingsAction::None);
        }

        match action_button_at(x, y) {
            Some(GameplaySettingsWidgets::ApplyButton) => {
                println!("Apply clicked (not yet applied)");
                Some(GameplaySettingsAction::Close)
            }
            Some(GameplaySettingsWidgets::CancelButton) => {
                println!("Cancel clicked (no changes applied)");
                Some(GameplaySettingsAction::Close)
            }
            _ => Some(GameplaySettingsAction::None),
        }
    }

    fn value_button(id: GameplaySettingsWidgets, label: &str) -> ButtonWidget {
        ButtonWidget::new(id as WidgetID, label)
    }
}

pub fn show_gameplay_settings(
    window_manager: &mut WindowManager,
    settings: &GameplaySettingsWindow,
) -> WindowID {
    let window = settings.build_window();
    window_manager.add_window(window)
}

fn layout_sections() -> [Rect; 3] {
    let content_rect = Rect::new(
        WINDOW_X,
        WINDOW_Y + TITLE_BAR_HEIGHT + TAB_BAR_HEIGHT,
        WINDOW_WIDTH,
        WINDOW_HEIGHT - (TITLE_BAR_HEIGHT + TAB_BAR_HEIGHT) as u32,
    );

    let padding = 12;
    let spacing = ROW_SPACING;
    let count = 3;
    let available_height = content_rect.height as i32 - 2 * padding;
    let child_height = (available_height - (count - 1) * spacing) / count;
    let mut rects = [Rect::new(0, 0, 0, 0); 3];
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

fn tab_button_rects() -> [Rect; 3] {
    let bar_rect = Rect::new(
        WINDOW_X + 12,
        WINDOW_Y + TITLE_BAR_HEIGHT,
        WINDOW_WIDTH - 24,
        TAB_BAR_HEIGHT as u32,
    );
    let spacing = 6;
    let button_width = (bar_rect.width as i32 - 2 * spacing) / 3;
    let audio_rect = Rect::new(bar_rect.x, bar_rect.y, button_width as u32, bar_rect.height);
    let video_rect = Rect::new(
        bar_rect.x + button_width + spacing,
        bar_rect.y,
        button_width as u32,
        bar_rect.height,
    );
    let gameplay_rect = Rect::new(
        bar_rect.x + 2 * (button_width + spacing),
        bar_rect.y,
        button_width as u32,
        bar_rect.height,
    );
    [audio_rect, video_rect, gameplay_rect]
}

fn autosave_toggle_rect() -> Rect {
    let sections = layout_sections();
    let autosave_children = layout_vertical_children(sections[0], 3);
    autosave_children[1]
}

fn autosave_frequency_rect() -> Rect {
    let sections = layout_sections();
    let autosave_children = layout_vertical_children(sections[0], 3);
    autosave_children[2]
}

fn difficulty_preset_rect() -> Rect {
    let sections = layout_sections();
    let difficulty_children = layout_vertical_children(sections[1], 3);
    difficulty_children[1]
}

fn breakdown_toggle_rect() -> Rect {
    let sections = layout_sections();
    let difficulty_children = layout_vertical_children(sections[1], 3);
    difficulty_children[2]
}

fn action_button_at(x: i32, y: i32) -> Option<GameplaySettingsWidgets> {
    let sections = layout_sections();
    let actions_rect = sections[2];
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
        Some(GameplaySettingsWidgets::ApplyButton)
    } else if right_rect.contains_point(x, y) {
        Some(GameplaySettingsWidgets::CancelButton)
    } else {
        None
    }
}

fn next_frequency(value: u8) -> u8 {
    match value {
        1 => 3,
        3 => 6,
        6 => 12,
        12 => 24,
        _ => 1,
    }
}

fn difficulty_labels() -> [&'static str; 4] {
    ["Easy", "Medium", "Hard", "Custom"]
}

fn difficulty_label(index: usize) -> &'static str {
    let labels = difficulty_labels();
    labels.get(index).copied().unwrap_or("Custom")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gameplay_settings_toggle_and_frequency() {
        let mut window = GameplaySettingsWindow::new();
        let rect = Rect::new(WINDOW_X, WINDOW_Y, WINDOW_WIDTH, WINDOW_HEIGHT);

        let toggle_rect = autosave_toggle_rect();
        assert!(window
            .handle_click(toggle_rect.x + 1, toggle_rect.y + 1, rect)
            .is_some());
        assert!(!window.autosave_enabled());

        let frequency_rect = autosave_frequency_rect();
        assert!(window
            .handle_click(frequency_rect.x + 1, frequency_rect.y + 1, rect)
            .is_some());
        assert_ne!(window.autosave_frequency(), 10);
    }

    #[test]
    fn test_gameplay_settings_switch_tabs() {
        let mut window = GameplaySettingsWindow::new();
        let rect = Rect::new(WINDOW_X, WINDOW_Y, WINDOW_WIDTH, WINDOW_HEIGHT);
        let tab_rects = tab_button_rects();
        let audio_action = window.handle_click(tab_rects[0].x + 1, tab_rects[0].y + 1, rect);
        assert!(matches!(
            audio_action,
            Some(GameplaySettingsAction::OpenAudio)
        ));
        let video_action = window.handle_click(tab_rects[1].x + 1, tab_rects[1].y + 1, rect);
        assert!(matches!(
            video_action,
            Some(GameplaySettingsAction::OpenVideo)
        ));
    }

    #[test]
    fn test_gameplay_settings_window_contains_widgets() {
        let window = GameplaySettingsWindow::new().build_window();
        assert_eq!(window.id, GameplaySettingsWidgets::Window as WidgetID);
        assert_eq!(window.title, "Gameplay Settings");
    }
}
