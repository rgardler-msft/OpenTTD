//! League table windows for displaying company performance rankings

use crate::{Window, WindowID, WindowManager};
use openttd_gfx::{Colour, GfxContext, Rect};

/// Window ID for the performance league table
pub const LEAGUE_WINDOW_ID: WindowID = 4000;

/// Window ID for the detailed performance rating window
pub const PERFORMANCE_DETAIL_WINDOW_ID: WindowID = 4001;

/// Performance titles based on rating
#[derive(Debug, Clone, Copy)]
pub enum PerformanceTitle {
    Engineer,             // 0-127
    TrafficManager,       // 128-255
    TransportCoordinator, // 256-383
    RouteSupervisor,      // 384-511
    Director,             // 512-639
    ChiefExecutive,       // 640-767
    Chairman,             // 768-895
    President,            // 896-959
    Tycoon,               // 960-1000
}

impl PerformanceTitle {
    /// Get title for a given performance rating (0-1000)
    pub fn from_rating(rating: u32) -> Self {
        match rating {
            0..=127 => Self::Engineer,
            128..=255 => Self::TrafficManager,
            256..=383 => Self::TransportCoordinator,
            384..=511 => Self::RouteSupervisor,
            512..=639 => Self::Director,
            640..=767 => Self::ChiefExecutive,
            768..=895 => Self::Chairman,
            896..=959 => Self::President,
            _ => Self::Tycoon,
        }
    }

    /// Get the display string for the title
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Engineer => "Engineer",
            Self::TrafficManager => "Traffic Manager",
            Self::TransportCoordinator => "Transport Coordinator",
            Self::RouteSupervisor => "Route Supervisor",
            Self::Director => "Director",
            Self::ChiefExecutive => "Chief Executive",
            Self::Chairman => "Chairman",
            Self::President => "President",
            Self::Tycoon => "Tycoon of the Century",
        }
    }
}

/// Company color enumeration
#[derive(Debug, Clone, Copy)]
pub enum CompanyColor {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Brown,
    Grey,
}

impl CompanyColor {
    /// Convert to actual color for rendering
    pub fn to_colour(&self) -> Colour {
        match self {
            Self::Red => Colour::rgb(200, 0, 0),
            Self::Blue => Colour::rgb(0, 0, 200),
            Self::Green => Colour::rgb(0, 160, 0),
            Self::Yellow => Colour::rgb(200, 200, 0),
            Self::Orange => Colour::rgb(255, 128, 0),
            Self::Purple => Colour::rgb(128, 0, 200),
            Self::Brown => Colour::rgb(139, 69, 19),
            Self::Grey => Colour::rgb(128, 128, 128),
        }
    }
}

/// Company information for league table
#[derive(Debug, Clone)]
pub struct CompanyInfo {
    pub id: u8,
    pub name: String,
    pub manager: String,
    pub color: CompanyColor,
    pub rating: u32,   // Performance rating (0-1000)
    pub value: i64,    // Company value
    pub vehicles: u32, // Number of vehicles
}

impl CompanyInfo {
    /// Get the performance title for this company
    pub fn get_title(&self) -> PerformanceTitle {
        PerformanceTitle::from_rating(self.rating)
    }
}

/// League table window
pub struct LeagueWindow {
    companies: Vec<CompanyInfo>,
    selected_company: Option<u8>,
}

impl LeagueWindow {
    /// Create a new league window with mock data
    pub fn new() -> Self {
        // Create mock companies for testing
        let companies = vec![
            CompanyInfo {
                id: 0,
                name: "Transport Tycoon Ltd".to_string(),
                manager: "Player 1".to_string(),
                color: CompanyColor::Red,
                rating: 850,
                value: 5_000_000,
                vehicles: 45,
            },
            CompanyInfo {
                id: 1,
                name: "City Connect".to_string(),
                manager: "Player 2".to_string(),
                color: CompanyColor::Blue,
                rating: 720,
                value: 3_500_000,
                vehicles: 32,
            },
            CompanyInfo {
                id: 2,
                name: "Rural Routes Co".to_string(),
                manager: "Player 3".to_string(),
                color: CompanyColor::Green,
                rating: 650,
                value: 2_800_000,
                vehicles: 28,
            },
            CompanyInfo {
                id: 3,
                name: "Express Logistics".to_string(),
                manager: "AI Player".to_string(),
                color: CompanyColor::Yellow,
                rating: 580,
                value: 2_200_000,
                vehicles: 24,
            },
        ];

        Self {
            companies,
            selected_company: None,
        }
    }

    /// Draw the league window
    pub fn draw(&self, gfx: &mut GfxContext, rect: Rect) {
        // Draw window background
        gfx.fill_rect(rect, Colour::ui_window_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();

        // Draw title bar
        let title_rect = Rect::new(rect.x, rect.y, rect.width, 30);
        gfx.fill_rect(title_rect, Colour::ui_title_bar()).ok();
        gfx.draw_text(
            "Company League Table",
            title_rect.x + 10,
            title_rect.y + 8,
            Colour::ui_title_text(),
            None,
        )
        .ok();

        // Draw close button
        let close_rect = Rect::new(rect.x + rect.width as i32 - 25, rect.y + 5, 20, 20);
        gfx.draw_rect(close_rect, Colour::ui_border()).ok();
        gfx.draw_text(
            "X",
            close_rect.x + 6,
            close_rect.y + 4,
            Colour::ui_text(),
            None,
        )
        .ok();

        // Sort companies by rating (descending)
        let mut sorted_companies = self.companies.clone();
        sorted_companies.sort_by(|a, b| b.rating.cmp(&a.rating));

        // Draw column headers
        let headers_y = rect.y + 40;
        gfx.draw_text("Rank", rect.x + 10, headers_y, Colour::ui_text(), None)
            .ok();
        gfx.draw_text("Company", rect.x + 60, headers_y, Colour::ui_text(), None)
            .ok();
        gfx.draw_text("Manager", rect.x + 200, headers_y, Colour::ui_text(), None)
            .ok();
        gfx.draw_text("Title", rect.x + 320, headers_y, Colour::ui_text(), None)
            .ok();
        gfx.draw_text("Rating", rect.x + 480, headers_y, Colour::ui_text(), None)
            .ok();

        // Draw separator line
        let sep_y = headers_y + 20;
        gfx.draw_line(
            rect.x + 10,
            sep_y,
            rect.x + rect.width as i32 - 10,
            sep_y,
            Colour::ui_border(),
        )
        .ok();

        // Draw company entries
        let mut y = sep_y + 10;
        for (rank, company) in sorted_companies.iter().enumerate() {
            let row_rect = Rect::new(rect.x + 5, y - 4, rect.width - 10, 24);

            // Highlight selected company
            if Some(company.id) == self.selected_company {
                gfx.fill_rect(row_rect, Colour::ui_highlight()).ok();
            }

            // Draw rank with medal for top 3
            let rank_str = match rank {
                0 => "🥇 1st".to_string(),
                1 => "🥈 2nd".to_string(),
                2 => "🥉 3rd".to_string(),
                _ => format!("   {}th", rank + 1),
            };
            gfx.draw_text(&rank_str, rect.x + 10, y, Colour::ui_text(), None)
                .ok();

            // Draw company color box
            let color_box = Rect::new(rect.x + 60, y - 2, 16, 16);
            gfx.fill_rect(color_box, company.color.to_colour()).ok();
            gfx.draw_rect(color_box, Colour::ui_border()).ok();

            // Draw company name
            gfx.draw_text(&company.name, rect.x + 80, y, Colour::ui_text(), None)
                .ok();

            // Draw manager
            gfx.draw_text(&company.manager, rect.x + 200, y, Colour::ui_text(), None)
                .ok();

            // Draw title
            let title = company.get_title();
            gfx.draw_text(title.as_str(), rect.x + 320, y, Colour::ui_text(), None)
                .ok();

            // Draw rating with color based on performance
            let rating_color = if company.rating >= 800 {
                Colour::rgb(0, 200, 0) // Green for excellent
            } else if company.rating >= 600 {
                Colour::rgb(200, 200, 0) // Yellow for good
            } else {
                Colour::rgb(200, 0, 0) // Red for poor
            };
            gfx.draw_text(
                &format!("{}/1000", company.rating),
                rect.x + 480,
                y,
                rating_color,
                None,
            )
            .ok();

            y += 28;
        }

        // Draw footer with summary
        let footer_y = rect.y + rect.height as i32 - 30;
        gfx.draw_line(
            rect.x + 10,
            footer_y - 5,
            rect.x + rect.width as i32 - 10,
            footer_y - 5,
            Colour::ui_border(),
        )
        .ok();

        let total_companies = self.companies.len();
        let avg_rating: u32 =
            self.companies.iter().map(|c| c.rating).sum::<u32>() / total_companies as u32;
        gfx.draw_text(
            &format!(
                "Total Companies: {}  |  Average Rating: {}",
                total_companies, avg_rating
            ),
            rect.x + 10,
            footer_y,
            Colour::ui_text(),
            None,
        )
        .ok();
    }

    /// Handle click events
    pub fn handle_click(&mut self, x: i32, y: i32, rect: Rect) -> Option<WindowAction> {
        // Check close button
        let close_rect = Rect::new(rect.x + rect.width as i32 - 25, rect.y + 5, 20, 20);
        if rect_contains_point(&close_rect, x, y) {
            return Some(WindowAction::Close);
        }

        // Check company rows for selection
        let mut sorted_companies = self.companies.clone();
        sorted_companies.sort_by(|a, b| b.rating.cmp(&a.rating));

        let start_y = rect.y + 70;
        for (idx, company) in sorted_companies.iter().enumerate() {
            let row_y = start_y + (idx as i32 * 28);
            let row_rect = Rect::new(rect.x + 5, row_y - 4, rect.width - 10, 24);

            if rect_contains_point(&row_rect, x, y) {
                self.selected_company = Some(company.id);
                return Some(WindowAction::CompanySelected(company.id));
            }
        }

        None
    }
}

/// Window actions that can be triggered
#[derive(Debug, Clone)]
pub enum WindowAction {
    Close,
    CompanySelected(u8),
}

/// Helper function to check if a point is within a rectangle
fn rect_contains_point(rect: &Rect, x: i32, y: i32) -> bool {
    x >= rect.x && x < rect.x + rect.width as i32 && y >= rect.y && y < rect.y + rect.height as i32
}

/// Create and show the league table window
pub fn show_league_table(wm: &mut WindowManager) -> WindowID {
    let window_width = 600;
    let window_height = 400;
    let screen_width = 800;
    let screen_height = 600;

    let window_rect = Rect::new(
        (screen_width - window_width) as i32 / 2,
        (screen_height - window_height) as i32 / 2,
        window_width,
        window_height,
    );

    let window = Window::new(LEAGUE_WINDOW_ID, "Company League Table", window_rect);
    wm.add_window(window);

    LEAGUE_WINDOW_ID
}

/// Detailed performance rating window
pub struct PerformanceDetailWindow {
    company_id: u8,
    company: CompanyInfo,
}

impl PerformanceDetailWindow {
    /// Create a new performance detail window for a specific company
    pub fn new(company: CompanyInfo) -> Self {
        Self {
            company_id: company.id,
            company,
        }
    }

    /// Draw the performance detail window
    pub fn draw(&self, gfx: &mut GfxContext, rect: Rect) {
        // Draw window background
        gfx.fill_rect(rect, Colour::ui_window_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();

        // Draw title bar
        let title_rect = Rect::new(rect.x, rect.y, rect.width, 30);
        gfx.fill_rect(title_rect, Colour::ui_title_bar()).ok();
        let title = format!("{} - Performance Details", self.company.name);
        gfx.draw_text(
            &title,
            title_rect.x + 10,
            title_rect.y + 8,
            Colour::ui_title_text(),
            None,
        )
        .ok();

        // Draw close button
        let close_rect = Rect::new(rect.x + rect.width as i32 - 25, rect.y + 5, 20, 20);
        gfx.draw_rect(close_rect, Colour::ui_border()).ok();
        gfx.draw_text(
            "X",
            close_rect.x + 6,
            close_rect.y + 4,
            Colour::ui_text(),
            None,
        )
        .ok();

        // Draw performance breakdown
        let mut y = rect.y + 50;
        let label_x = rect.x + 20;
        let value_x = rect.x + 200;

        // Company info section
        gfx.draw_text("Company Information", label_x, y, Colour::ui_text(), None)
            .ok();
        y += 25;

        gfx.draw_text("Manager:", label_x + 20, y, Colour::ui_text(), None)
            .ok();
        gfx.draw_text(&self.company.manager, value_x, y, Colour::ui_text(), None)
            .ok();
        y += 20;

        gfx.draw_text("Company Value:", label_x + 20, y, Colour::ui_text(), None)
            .ok();
        gfx.draw_text(
            &format!("${}", self.company.value),
            value_x,
            y,
            Colour::ui_text(),
            None,
        )
        .ok();
        y += 20;

        gfx.draw_text("Vehicle Count:", label_x + 20, y, Colour::ui_text(), None)
            .ok();
        gfx.draw_text(
            &format!("{}", self.company.vehicles),
            value_x,
            y,
            Colour::ui_text(),
            None,
        )
        .ok();
        y += 30;

        // Performance section
        gfx.draw_text("Performance Rating", label_x, y, Colour::ui_text(), None)
            .ok();
        y += 25;

        gfx.draw_text("Current Rating:", label_x + 20, y, Colour::ui_text(), None)
            .ok();
        let rating_color = if self.company.rating >= 800 {
            Colour::rgb(0, 200, 0)
        } else if self.company.rating >= 600 {
            Colour::rgb(200, 200, 0)
        } else {
            Colour::rgb(200, 0, 0)
        };
        gfx.draw_text(
            &format!("{}/1000", self.company.rating),
            value_x,
            y,
            rating_color,
            None,
        )
        .ok();
        y += 20;

        gfx.draw_text(
            "Performance Title:",
            label_x + 20,
            y,
            Colour::ui_text(),
            None,
        )
        .ok();
        gfx.draw_text(
            self.company.get_title().as_str(),
            value_x,
            y,
            Colour::ui_text(),
            None,
        )
        .ok();
        y += 30;

        // Draw rating bar
        let bar_width = 300;
        let bar_height = 20;
        let bar_x = label_x + 20;
        let bar_y = y;

        // Background bar
        let bg_bar = Rect::new(bar_x, bar_y, bar_width, bar_height);
        gfx.fill_rect(bg_bar, Colour::rgb(50, 50, 50)).ok();

        // Progress bar
        let progress_width = (self.company.rating * bar_width / 1000) as u32;
        let progress_bar = Rect::new(bar_x, bar_y, progress_width, bar_height);
        gfx.fill_rect(progress_bar, rating_color).ok();

        // Border
        gfx.draw_rect(bg_bar, Colour::ui_border()).ok();

        // Labels
        gfx.draw_text("0", bar_x - 15, bar_y + 4, Colour::ui_text(), None)
            .ok();
        gfx.draw_text(
            "1000",
            bar_x + bar_width as i32 + 5,
            bar_y + 4,
            Colour::ui_text(),
            None,
        )
        .ok();
    }
}
