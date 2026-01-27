//! Highscore window and data structures

use crate::{Window, WindowID, WindowManager};
use openttd_gfx::{Colour, GfxContext, Rect};

// Window IDs for highscore windows
pub const HIGHSCORE_WINDOW_ID: WindowID = 2000;

/// Single highscore entry
#[derive(Debug, Clone, Default)]
pub struct HighScore {
    /// The name of the company and president
    pub name: String,
    /// The title achieved
    pub title: String,
    /// The score for this high score
    pub score: u16,
}

/// Record 5 high scores
pub type HighScores = [HighScore; 5];

/// Difficulty levels for highscores
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyLevel {
    Easy = 0,
    Medium = 1,
    Hard = 2,
    Custom = 3,
    Multiplayer = 4,
}

impl DifficultyLevel {
    pub const COUNT: usize = 5;
}

/// Record high scores for each difficulty level
pub type HighScoresTable = [HighScores; DifficultyLevel::COUNT];

/// Global highscore table (in a real implementation, this would be loaded/saved)
static mut HIGHSCORE_TABLE: Option<HighScoresTable> = None;

/// Get the global highscore table
pub fn get_highscore_table() -> &'static HighScoresTable {
    unsafe {
        HIGHSCORE_TABLE.get_or_insert_with(|| {
            // Initialize with mock data for testing
            let mut table: HighScoresTable = Default::default();

            // Add some sample highscores for testing
            for (difficulty_idx, difficulty_scores) in table.iter_mut().enumerate() {
                for (rank, score_entry) in difficulty_scores.iter_mut().enumerate() {
                    if rank < 3 {
                        // Only fill top 3 for testing
                        *score_entry = HighScore {
                            name: format!("Company {}", rank + 1),
                            title: match rank {
                                0 => "Tycoon of the Century",
                                1 => "Entrepreneur",
                                2 => "Businessman",
                                _ => "Manager",
                            }
                            .to_string(),
                            score: (5000 - (rank as u16 * 1000) - (difficulty_idx as u16 * 100))
                                as u16,
                        };
                    }
                }
            }

            table
        })
    }
}

/// Show the highscore table
pub fn show_highscore_table(
    window_manager: &mut WindowManager,
    _difficulty: DifficultyLevel,
    _rank: Option<i8>,
) -> WindowID {
    // Close any existing highscore window
    let _ = window_manager.remove_window(HIGHSCORE_WINDOW_ID);

    // Create new highscore window
    let window = Window::new(
        HIGHSCORE_WINDOW_ID,
        "Highscore",
        Rect::new(50, 50, 700, 500),
    );

    window_manager.add_window(window)
}

/// Handle drawing for highscore windows
pub fn draw_highscore_window(window: &Window, gfx: &mut GfxContext) {
    let rect = window.rect;

    // Fill with OpenTTD-style dark gray background
    let _ = gfx.fill_rect(rect, Colour::ui_background());

    // Draw border with OpenTTD style
    let _ = gfx.draw_rect(rect, Colour::ui_border());

    // Draw inner border for visual depth
    let inner_rect = Rect::new(rect.x + 2, rect.y + 2, rect.width - 4, rect.height - 4);
    let _ = gfx.draw_rect(inner_rect, Colour::rgb(80, 80, 80));

    // Draw title background panel
    let title_panel = Rect::new(rect.x + 10, rect.y + 10, rect.width - 20, 50);
    let _ = gfx.fill_rect(title_panel, Colour::rgb(40, 40, 40));
    let _ = gfx.draw_rect(title_panel, Colour::ui_border());

    // Draw title text
    let (center_x, center_y) = title_panel.center();
    let _ = gfx.draw_text(
        "HIGH SCORES",
        center_x - 50,
        center_y - 10,
        Colour::ui_text(),
        None,
    );

    // Draw difficulty selector area
    let diff_panel = Rect::new(rect.x + 10, rect.y + 70, rect.width - 20, 30);
    let _ = gfx.fill_rect(diff_panel, Colour::rgb(30, 30, 30));
    let _ = gfx.draw_rect(diff_panel, Colour::rgb(60, 60, 60));

    // Show current difficulty
    let diff_text = "Difficulty: Custom";
    let _ = gfx.draw_text(
        diff_text,
        diff_panel.x + 10,
        diff_panel.y + 8,
        Colour::ui_text(),
        None,
    );

    // Get highscores for custom difficulty (for testing)
    let table = get_highscore_table();
    let scores = &table[DifficultyLevel::Custom as usize];

    // Draw scores panel
    let scores_panel = Rect::new(
        rect.x + 10,
        rect.y + 110,
        rect.width - 20,
        rect.height - 130,
    );
    let _ = gfx.fill_rect(scores_panel, Colour::rgb(25, 25, 25));
    let _ = gfx.draw_rect(scores_panel, Colour::rgb(60, 60, 60));

    // Draw each highscore entry
    let entry_height = 65;
    let start_y = scores_panel.y + 20;

    for (i, score) in scores.iter().enumerate() {
        let y = start_y + (i as i32 * entry_height);

        // Alternate row backgrounds for readability
        if i % 2 == 0 {
            let row_bg = Rect::new(scores_panel.x + 5, y - 5, scores_panel.width - 10, 60);
            let _ = gfx.fill_rect(row_bg, Colour::rgb(35, 35, 35));
        }

        // Highlight color for top 3
        let color = match i {
            0 => Colour::rgb(255, 215, 0),   // Gold
            1 => Colour::rgb(192, 192, 192), // Silver
            2 => Colour::rgb(205, 127, 50),  // Bronze
            _ => Colour::ui_text(),
        };

        // Draw position number with medal icon placeholder
        let pos_text = format!("#{}", i + 1);
        let _ = gfx.draw_text(&pos_text, scores_panel.x + 20, y, color, None);

        // Draw company name and score if not empty
        if !score.name.is_empty() {
            // Company name
            let _ = gfx.draw_text(&score.name, scores_panel.x + 80, y, Colour::ui_text(), None);

            // Title on second line
            let _ = gfx.draw_text(
                &score.title,
                scores_panel.x + 80,
                y + 20,
                Colour::rgb(150, 150, 150),
                None,
            );

            // Score with special formatting
            let score_text = format!("{} points", score.score);
            let score_x = scores_panel.x + scores_panel.width as i32 - 150;
            let _ = gfx.draw_text(&score_text, score_x, y + 10, color, None);
        } else {
            // Empty slot
            let empty_text = "--- Empty ---";
            let _ = gfx.draw_text(
                empty_text,
                scores_panel.x + 80,
                y + 10,
                Colour::rgb(100, 100, 100),
                None,
            );
        }
    }

    // Draw "Press any key to continue" hint
    let hint_y = rect.y + rect.height as i32 - 30;
    let hint_text = "Click anywhere or press any key to close";
    let _ = gfx.draw_text(
        hint_text,
        rect.x + 20,
        hint_y,
        Colour::rgb(150, 150, 150),
        None,
    );
}
