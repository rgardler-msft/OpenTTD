//! Audio settings window

use crate::{
    ButtonWidget, ContainerWidget, LabelWidget, Rect, WidgetID, Window, WindowID, WindowManager,
};

/// Window ID for the audio settings window
pub const AUDIO_SETTINGS_WINDOW_ID: WindowID = 7200;

/// Widget IDs for audio settings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioSettingsWidgets {
    Window = 7200,
    TabsContainer = 7201,
    VideoTab = 7202,
    AudioTab = 7203,
    GameplayTab = 7204,
    MusicSectionLabel = 7210,
    MusicToggle = 7211,
    MusicVolume = 7212,
    EffectsSectionLabel = 7220,
    EffectsToggle = 7221,
    EffectsVolume = 7222,
    ApplyButton = 7230,
    CancelButton = 7231,
}

impl From<AudioSettingsWidgets> for WidgetID {
    fn from(value: AudioSettingsWidgets) -> Self {
        value as WidgetID
    }
}

#[derive(Debug, Clone)]
struct AudioSettingsState {
    music_enabled: bool,
    sound_enabled: bool,
    music_volume: u8,
    sound_volume: u8,
}

impl Default for AudioSettingsState {
    fn default() -> Self {
        Self {
            music_enabled: true,
            sound_enabled: true,
            music_volume: 80,
            sound_volume: 70,
        }
    }
}

pub enum AudioSettingsAction {
    None,
    Close,
    OpenVideo,
    OpenGameplay,
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

/// Audio settings window controller
pub struct AudioSettingsWindow {
    state: AudioSettingsState,
}

impl AudioSettingsWindow {
    pub fn new() -> Self {
        Self {
            state: AudioSettingsState::default(),
        }
    }

    pub fn music_enabled(&self) -> bool {
        self.state.music_enabled
    }

    pub fn sound_enabled(&self) -> bool {
        self.state.sound_enabled
    }

    pub fn music_volume(&self) -> u8 {
        self.state.music_volume
    }

    pub fn sound_volume(&self) -> u8 {
        self.state.sound_volume
    }

    /// Convert to a Window for the window manager
    pub fn build_window(&self) -> Window {
        let mut window = Window::new(
            AUDIO_SETTINGS_WINDOW_ID,
            "Audio Settings",
            Rect::new(WINDOW_X, WINDOW_Y, WINDOW_WIDTH, WINDOW_HEIGHT),
        );

        let mut main_container = ContainerWidget::new_vertical(7300)
            .with_padding(12)
            .with_spacing(10);

        let mut tabs = ContainerWidget::new_horizontal(AudioSettingsWidgets::TabsContainer as u32)
            .with_spacing(6)
            .with_padding(0);
        tabs.add_child(Box::new(
            LabelWidget::new(AudioSettingsWidgets::AudioTab as WidgetID, "Audio")
                .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        tabs.add_child(Box::new(ButtonWidget::new(
            AudioSettingsWidgets::VideoTab as WidgetID,
            "Video",
        )));
        tabs.add_child(Box::new(ButtonWidget::new(
            AudioSettingsWidgets::GameplayTab as WidgetID,
            "Gameplay",
        )));
        main_container.add_child(Box::new(tabs));

        let mut music_section = ContainerWidget::new_vertical(7301).with_spacing(6);
        music_section.add_child(Box::new(
            LabelWidget::new(AudioSettingsWidgets::MusicSectionLabel as WidgetID, "Music")
                .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        music_section.add_child(Box::new(Self::value_button(
            AudioSettingsWidgets::MusicToggle,
            &format!(
                "Music: {}",
                if self.state.music_enabled {
                    "On"
                } else {
                    "Off"
                }
            ),
        )));
        music_section.add_child(Box::new(Self::value_button(
            AudioSettingsWidgets::MusicVolume,
            &format!("Music Volume: {}%", self.state.music_volume),
        )));

        let mut effects_section = ContainerWidget::new_vertical(7302).with_spacing(6);
        effects_section.add_child(Box::new(
            LabelWidget::new(
                AudioSettingsWidgets::EffectsSectionLabel as WidgetID,
                "Sound Effects",
            )
            .with_colour(openttd_gfx::Colour::ui_highlight()),
        ));
        effects_section.add_child(Box::new(Self::value_button(
            AudioSettingsWidgets::EffectsToggle,
            &format!(
                "Sound Effects: {}",
                if self.state.sound_enabled {
                    "On"
                } else {
                    "Off"
                }
            ),
        )));
        effects_section.add_child(Box::new(Self::value_button(
            AudioSettingsWidgets::EffectsVolume,
            &format!("Effects Volume: {}%", self.state.sound_volume),
        )));

        let mut actions = ContainerWidget::new_horizontal(7303)
            .with_spacing(10)
            .with_padding(0);
        actions.add_child(Box::new(ButtonWidget::new(
            AudioSettingsWidgets::ApplyButton as WidgetID,
            "Apply",
        )));
        actions.add_child(Box::new(ButtonWidget::new(
            AudioSettingsWidgets::CancelButton as WidgetID,
            "Cancel",
        )));

        main_container.add_child(Box::new(music_section));
        main_container.add_child(Box::new(effects_section));
        main_container.add_child(Box::new(actions));

        window.set_root_widget(Box::new(main_container));
        window
    }

    pub fn handle_click(&mut self, x: i32, y: i32, rect: Rect) -> Option<AudioSettingsAction> {
        if !rect.contains_point(x, y) {
            return None;
        }

        let close_rect = Rect::new(rect.x + rect.width as i32 - 18, rect.y + 2, 16, 16);
        if close_rect.contains_point(x, y) {
            println!("Audio settings closed (not yet applied)");
            return Some(AudioSettingsAction::Close);
        }

        let tab_rects = tab_button_rects();
        if tab_rects[1].contains_point(x, y) {
            println!("Switching to video tab");
            return Some(AudioSettingsAction::OpenVideo);
        }
        if tab_rects[2].contains_point(x, y) {
            println!("Switching to gameplay tab");
            return Some(AudioSettingsAction::OpenGameplay);
        }

        if music_toggle_rect().contains_point(x, y) {
            self.state.music_enabled = !self.state.music_enabled;
            println!(
                "Music toggled {} (not yet applied)",
                if self.state.music_enabled {
                    "on"
                } else {
                    "off"
                }
            );
            return Some(AudioSettingsAction::None);
        }

        if music_volume_rect().contains_point(x, y) {
            self.state.music_volume = next_volume(self.state.music_volume);
            println!(
                "Music volume set to {}% (not yet applied)",
                self.state.music_volume
            );
            return Some(AudioSettingsAction::None);
        }

        if effects_toggle_rect().contains_point(x, y) {
            self.state.sound_enabled = !self.state.sound_enabled;
            println!(
                "Sound effects toggled {} (not yet applied)",
                if self.state.sound_enabled {
                    "on"
                } else {
                    "off"
                }
            );
            return Some(AudioSettingsAction::None);
        }

        if effects_volume_rect().contains_point(x, y) {
            self.state.sound_volume = next_volume(self.state.sound_volume);
            println!(
                "Effects volume set to {}% (not yet applied)",
                self.state.sound_volume
            );
            return Some(AudioSettingsAction::None);
        }

        match action_button_at(x, y) {
            Some(AudioSettingsWidgets::ApplyButton) => {
                println!("Apply clicked (not yet applied)");
                Some(AudioSettingsAction::Close)
            }
            Some(AudioSettingsWidgets::CancelButton) => {
                println!("Cancel clicked (no changes applied)");
                Some(AudioSettingsAction::Close)
            }
            _ => Some(AudioSettingsAction::None),
        }
    }

    fn value_button(id: AudioSettingsWidgets, label: &str) -> ButtonWidget {
        ButtonWidget::new(id as WidgetID, label)
    }
}

/// Create and show the audio settings window
pub fn show_audio_settings(
    window_manager: &mut WindowManager,
    settings: &AudioSettingsWindow,
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

fn music_toggle_rect() -> Rect {
    let sections = layout_sections();
    let music_children = layout_vertical_children(sections[0], 3);
    music_children[1]
}

fn music_volume_rect() -> Rect {
    let sections = layout_sections();
    let music_children = layout_vertical_children(sections[0], 3);
    music_children[2]
}

fn effects_toggle_rect() -> Rect {
    let sections = layout_sections();
    let effect_children = layout_vertical_children(sections[1], 3);
    effect_children[1]
}

fn effects_volume_rect() -> Rect {
    let sections = layout_sections();
    let effect_children = layout_vertical_children(sections[1], 3);
    effect_children[2]
}

fn action_button_at(x: i32, y: i32) -> Option<AudioSettingsWidgets> {
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
        Some(AudioSettingsWidgets::ApplyButton)
    } else if right_rect.contains_point(x, y) {
        Some(AudioSettingsWidgets::CancelButton)
    } else {
        None
    }
}

fn next_volume(value: u8) -> u8 {
    match value {
        0 => 25,
        25 => 50,
        50 => 75,
        75 => 100,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_settings_toggle_and_volume() {
        let mut window = AudioSettingsWindow::new();
        let rect = Rect::new(WINDOW_X, WINDOW_Y, WINDOW_WIDTH, WINDOW_HEIGHT);

        let music_toggle = music_toggle_rect();
        assert!(window
            .handle_click(music_toggle.x + 1, music_toggle.y + 1, rect)
            .is_some());
        assert!(!window.music_enabled());

        let volume_rect = music_volume_rect();
        assert!(window
            .handle_click(volume_rect.x + 1, volume_rect.y + 1, rect)
            .is_some());
        assert_ne!(window.music_volume(), 80);
    }

    #[test]
    fn test_audio_settings_window_contains_widgets() {
        let window = AudioSettingsWindow::new().build_window();
        assert_eq!(window.id, AudioSettingsWidgets::Window as WidgetID);
        assert_eq!(window.title, "Audio Settings");
    }

    #[test]
    fn test_audio_settings_switch_to_gameplay() {
        let mut window = AudioSettingsWindow::new();
        let rect = Rect::new(WINDOW_X, WINDOW_Y, WINDOW_WIDTH, WINDOW_HEIGHT);
        let tab_rects = tab_button_rects();
        let action = window.handle_click(tab_rects[2].x + 1, tab_rects[2].y + 1, rect);
        assert!(matches!(action, Some(AudioSettingsAction::OpenGameplay)));
    }
}
