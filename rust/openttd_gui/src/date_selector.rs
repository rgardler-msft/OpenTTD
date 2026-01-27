//! Date selection window for setting game dates

use crate::{WidgetID, Window, WindowID, WindowManager};
use openttd_gfx::{Colour, GfxContext, Rect};

/// Window ID for the date selector
pub const DATE_SELECTOR_WINDOW_ID: WindowID = 3000;

/// Widget IDs for date selector
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DateSelectorWidgets {
    DayDropdown = 3001,
    MonthDropdown = 3002,
    YearDropdown = 3003,
    SetDateButton = 3004,
    CancelButton = 3005,
}

impl Into<WidgetID> for DateSelectorWidgets {
    fn into(self) -> WidgetID {
        self as WidgetID
    }
}

/// Date structure
#[derive(Debug, Clone, Copy)]
pub struct GameDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

impl Default for GameDate {
    fn default() -> Self {
        GameDate {
            day: 1,
            month: 1,
            year: 1950,
        }
    }
}

impl GameDate {
    /// Create a new date
    pub fn new(day: u8, month: u8, year: u16) -> Self {
        GameDate {
            day: day.clamp(1, 31),
            month: month.clamp(1, 12),
            year: year.clamp(1900, 2100),
        }
    }

    /// Get month name
    pub fn month_name(&self) -> &'static str {
        match self.month {
            1 => "JAN",
            2 => "FEB",
            3 => "MAR",
            4 => "APR",
            5 => "MAY",
            6 => "JUN",
            7 => "JUL",
            8 => "AUG",
            9 => "SEP",
            10 => "OCT",
            11 => "NOV",
            12 => "DEC",
            _ => "???",
        }
    }

    /// Format as string
    pub fn to_string(&self) -> String {
        format!("{:02} {} {}", self.day, self.month_name(), self.year)
    }
}

/// Date selector window
pub struct DateSelectorWindow {
    selected_date: GameDate,
    min_year: u16,
    max_year: u16,
    dropdown_open: Option<DateSelectorWidgets>,
}

impl DateSelectorWindow {
    /// Create a new date selector window
    pub fn new(initial_date: GameDate, min_year: u16, max_year: u16) -> Self {
        DateSelectorWindow {
            selected_date: initial_date,
            min_year: min_year.max(1900),
            max_year: max_year.min(2100),
            dropdown_open: None,
        }
    }

    /// Draw the window
    pub fn draw(&self, gfx: &mut GfxContext, rect: Rect) {
        // Draw window background
        gfx.fill_rect(rect, Colour::ui_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();

        // Draw title bar
        let title_rect = Rect::new(rect.x, rect.y, rect.width, 30);
        gfx.fill_rect(title_rect, Colour::ui_window_background())
            .ok();
        gfx.draw_text(
            "SELECT DATE",
            rect.x + 10,
            rect.y + 10,
            Colour::ui_text(),
            None,
        )
        .ok();

        // Draw close button
        let close_rect = Rect::new(rect.x + rect.width as i32 - 25, rect.y + 5, 20, 20);
        gfx.fill_rect(close_rect, Colour::rgb(200, 50, 50)).ok();
        gfx.draw_rect(close_rect, Colour::BLACK).ok();
        gfx.draw_text("X", close_rect.x + 6, close_rect.y + 6, Colour::WHITE, None)
            .ok();

        // Content area
        let content_y = rect.y + 40;

        // Draw date display
        let date_str = self.selected_date.to_string();
        let display_rect = Rect::new(rect.x + 20, content_y, rect.width - 40, 30);
        gfx.fill_rect(display_rect, Colour::ui_window_background())
            .ok();
        gfx.draw_rect(display_rect, Colour::ui_border()).ok();
        gfx.draw_text(
            &date_str,
            display_rect.x + 10,
            display_rect.y + 10,
            Colour::ui_text(),
            None,
        )
        .ok();

        // Draw dropdown buttons
        let dropdown_y = content_y + 40;
        let dropdown_width = (rect.width - 60) / 3;

        // Day dropdown
        let day_rect = Rect::new(rect.x + 20, dropdown_y, dropdown_width, 25);
        self.draw_dropdown(
            gfx,
            day_rect,
            &format!("{:02}", self.selected_date.day),
            self.dropdown_open == Some(DateSelectorWidgets::DayDropdown),
        );

        // Month dropdown
        let month_rect = Rect::new(
            rect.x + 30 + dropdown_width as i32,
            dropdown_y,
            dropdown_width,
            25,
        );
        self.draw_dropdown(
            gfx,
            month_rect,
            self.selected_date.month_name(),
            self.dropdown_open == Some(DateSelectorWidgets::MonthDropdown),
        );

        // Year dropdown
        let year_rect = Rect::new(
            rect.x + 40 + (dropdown_width * 2) as i32,
            dropdown_y,
            dropdown_width,
            25,
        );
        self.draw_dropdown(
            gfx,
            year_rect,
            &format!("{}", self.selected_date.year),
            self.dropdown_open == Some(DateSelectorWidgets::YearDropdown),
        );

        // Draw dropdown lists if open
        if let Some(widget) = self.dropdown_open {
            let dropdown_list_y = dropdown_y + 30;
            match widget {
                DateSelectorWidgets::DayDropdown => {
                    self.draw_day_list(
                        gfx,
                        Rect::new(rect.x + 20, dropdown_list_y, dropdown_width, 200),
                    );
                }
                DateSelectorWidgets::MonthDropdown => {
                    self.draw_month_list(
                        gfx,
                        Rect::new(
                            rect.x + 30 + dropdown_width as i32,
                            dropdown_list_y,
                            dropdown_width,
                            200,
                        ),
                    );
                }
                DateSelectorWidgets::YearDropdown => {
                    self.draw_year_list(
                        gfx,
                        Rect::new(
                            rect.x + 40 + (dropdown_width * 2) as i32,
                            dropdown_list_y,
                            dropdown_width,
                            200,
                        ),
                    );
                }
                _ => {}
            }
        }

        // Draw action buttons
        let button_y = rect.y + rect.height as i32 - 40;

        // Set Date button
        let set_rect = Rect::new(rect.x + 20, button_y, 100, 25);
        gfx.draw_button(set_rect, "SET DATE", openttd_gfx::ButtonState::Normal, None)
            .ok();

        // Cancel button
        let cancel_rect = Rect::new(rect.x + rect.width as i32 - 120, button_y, 100, 25);
        gfx.draw_button(
            cancel_rect,
            "CANCEL",
            openttd_gfx::ButtonState::Normal,
            None,
        )
        .ok();
    }

    /// Draw a dropdown button
    fn draw_dropdown(&self, gfx: &mut GfxContext, rect: Rect, text: &str, is_open: bool) {
        let bg_color = if is_open {
            Colour::ui_highlight()
        } else {
            Colour::ui_window_background()
        };
        gfx.fill_rect(rect, bg_color).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();
        gfx.draw_text(text, rect.x + 5, rect.y + 8, Colour::ui_text(), None)
            .ok();

        // Draw dropdown arrow
        let arrow = if is_open { "^" } else { "v" };
        gfx.draw_text(
            arrow,
            rect.x + rect.width as i32 - 15,
            rect.y + 8,
            Colour::ui_text(),
            None,
        )
        .ok();
    }

    /// Draw day selection list
    fn draw_day_list(&self, gfx: &mut GfxContext, rect: Rect) {
        gfx.fill_rect(rect, Colour::ui_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();

        let mut y = rect.y + 5;
        for day in 1..=31 {
            if y + 20 > rect.y + rect.height as i32 {
                break;
            }

            let text = format!("{:02}", day);
            let is_selected = day == self.selected_date.day;
            let color = if is_selected {
                Colour::ui_highlight()
            } else {
                Colour::ui_text()
            };

            if is_selected {
                let highlight_rect = Rect::new(rect.x + 2, y - 2, rect.width - 4, 20);
                gfx.fill_rect(highlight_rect, Colour::ui_highlight()).ok();
            }

            gfx.draw_text(&text, rect.x + 10, y, color, None).ok();
            y += 20;
        }
    }

    /// Draw month selection list
    fn draw_month_list(&self, gfx: &mut GfxContext, rect: Rect) {
        gfx.fill_rect(rect, Colour::ui_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();

        let months = [
            "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
        ];

        let mut y = rect.y + 5;
        for (idx, month) in months.iter().enumerate() {
            let month_num = (idx + 1) as u8;
            let is_selected = month_num == self.selected_date.month;
            let color = if is_selected {
                Colour::ui_highlight()
            } else {
                Colour::ui_text()
            };

            if is_selected {
                let highlight_rect = Rect::new(rect.x + 2, y - 2, rect.width - 4, 20);
                gfx.fill_rect(highlight_rect, Colour::ui_highlight()).ok();
            }

            gfx.draw_text(month, rect.x + 10, y, color, None).ok();
            y += 20;
        }
    }

    /// Draw year selection list
    fn draw_year_list(&self, gfx: &mut GfxContext, rect: Rect) {
        gfx.fill_rect(rect, Colour::ui_background()).ok();
        gfx.draw_rect(rect, Colour::ui_border()).ok();

        // Show a range of years around the selected year
        let start_year = self.selected_date.year.saturating_sub(5).max(self.min_year);
        let end_year = (start_year + 10).min(self.max_year);

        let mut y = rect.y + 5;
        for year in start_year..=end_year {
            let text = format!("{}", year);
            let is_selected = year == self.selected_date.year;
            let color = if is_selected {
                Colour::ui_highlight()
            } else {
                Colour::ui_text()
            };

            if is_selected {
                let highlight_rect = Rect::new(rect.x + 2, y - 2, rect.width - 4, 20);
                gfx.fill_rect(highlight_rect, Colour::ui_highlight()).ok();
            }

            gfx.draw_text(&text, rect.x + 10, y, color, None).ok();
            y += 20;
        }
    }

    /// Handle click events
    pub fn handle_click(&mut self, x: i32, y: i32, window_rect: Rect) -> Option<GameDate> {
        // Check close button
        let close_rect = Rect::new(
            window_rect.x + window_rect.width as i32 - 25,
            window_rect.y + 5,
            20,
            20,
        );
        if rect_contains_point(&close_rect, x, y) {
            return None; // Signal to close window
        }

        let content_y = window_rect.y + 40;
        let dropdown_y = content_y + 40;
        let dropdown_width = (window_rect.width - 60) / 3;

        // Check dropdown buttons
        let day_rect = Rect::new(window_rect.x + 20, dropdown_y, dropdown_width, 25);
        let month_rect = Rect::new(
            window_rect.x + 30 + dropdown_width as i32,
            dropdown_y,
            dropdown_width,
            25,
        );
        let year_rect = Rect::new(
            window_rect.x + 40 + (dropdown_width * 2) as i32,
            dropdown_y,
            dropdown_width,
            25,
        );

        if rect_contains_point(&day_rect, x, y) {
            self.dropdown_open = if self.dropdown_open == Some(DateSelectorWidgets::DayDropdown) {
                None
            } else {
                Some(DateSelectorWidgets::DayDropdown)
            };
            return None;
        }

        if rect_contains_point(&month_rect, x, y) {
            self.dropdown_open = if self.dropdown_open == Some(DateSelectorWidgets::MonthDropdown) {
                None
            } else {
                Some(DateSelectorWidgets::MonthDropdown)
            };
            return None;
        }

        if rect_contains_point(&year_rect, x, y) {
            self.dropdown_open = if self.dropdown_open == Some(DateSelectorWidgets::YearDropdown) {
                None
            } else {
                Some(DateSelectorWidgets::YearDropdown)
            };
            return None;
        }

        // Check dropdown list selections
        if let Some(widget) = self.dropdown_open {
            let dropdown_list_y = dropdown_y + 30;
            match widget {
                DateSelectorWidgets::DayDropdown => {
                    let list_rect =
                        Rect::new(window_rect.x + 20, dropdown_list_y, dropdown_width, 200);
                    if rect_contains_point(&list_rect, x, y) {
                        let relative_y = y - list_rect.y - 5;
                        let day = (relative_y / 20) + 1;
                        if day >= 1 && day <= 31 {
                            self.selected_date.day = day as u8;
                            self.dropdown_open = None;
                        }
                    }
                }
                DateSelectorWidgets::MonthDropdown => {
                    let list_rect = Rect::new(
                        window_rect.x + 30 + dropdown_width as i32,
                        dropdown_list_y,
                        dropdown_width,
                        240,
                    );
                    if rect_contains_point(&list_rect, x, y) {
                        let relative_y = y - list_rect.y - 5;
                        let month = (relative_y / 20) + 1;
                        if month >= 1 && month <= 12 {
                            self.selected_date.month = month as u8;
                            self.dropdown_open = None;
                        }
                    }
                }
                DateSelectorWidgets::YearDropdown => {
                    let list_rect = Rect::new(
                        window_rect.x + 40 + (dropdown_width * 2) as i32,
                        dropdown_list_y,
                        dropdown_width,
                        200,
                    );
                    if rect_contains_point(&list_rect, x, y) {
                        let relative_y = y - list_rect.y - 5;
                        let index = relative_y / 20;
                        let start_year =
                            self.selected_date.year.saturating_sub(5).max(self.min_year);
                        let year = start_year + index as u16;
                        if year >= self.min_year && year <= self.max_year {
                            self.selected_date.year = year;
                            self.dropdown_open = None;
                        }
                    }
                }
                _ => {}
            }
        }

        // Check action buttons
        let button_y = window_rect.y + window_rect.height as i32 - 40;
        let set_rect = Rect::new(window_rect.x + 20, button_y, 100, 25);
        let cancel_rect = Rect::new(
            window_rect.x + window_rect.width as i32 - 120,
            button_y,
            100,
            25,
        );

        if rect_contains_point(&set_rect, x, y) {
            return Some(self.selected_date);
        }

        if rect_contains_point(&cancel_rect, x, y) {
            return None; // Signal to close window
        }

        None
    }
}

/// Helper function to check if a point is within a rectangle
fn rect_contains_point(rect: &Rect, x: i32, y: i32) -> bool {
    x >= rect.x && x < rect.x + rect.width as i32 && y >= rect.y && y < rect.y + rect.height as i32
}

/// Create and show the date selector window
pub fn show_date_selector(
    wm: &mut WindowManager,
    initial_date: GameDate,
    min_year: u16,
    max_year: u16,
) -> WindowID {
    // Set window size and position (centered)
    let window_width = 400;
    let window_height = 300;
    let screen_width = 800;
    let screen_height = 600;

    let window_rect = Rect::new(
        (screen_width - window_width) as i32 / 2,
        (screen_height - window_height) as i32 / 2,
        window_width,
        window_height,
    );

    // Create window with rect
    let window = Window::new(DATE_SELECTOR_WINDOW_ID, "Date Selector", window_rect);

    // Add the window to the manager
    wm.add_window(window);

    DATE_SELECTOR_WINDOW_ID
}
