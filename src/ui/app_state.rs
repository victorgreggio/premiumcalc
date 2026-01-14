use crate::domain::PremiumResult;
use ratatui::widgets::ListState;
use std::time::Duration;

/// State management for the TUI application
/// Follows Single Responsibility Principle - manages UI state only
pub struct AppState {
    pub results: Vec<PremiumResult>,
    pub selected_index: Option<usize>,
    pub selected_expanded: bool,
    pub total_calculation_time: Duration,
    pub list_state: ListState,
}

impl AppState {
    pub fn new(results: Vec<PremiumResult>, total_time: Duration) -> Self {
        let selected_index = if results.is_empty() { None } else { Some(0) };
        let mut list_state = ListState::default();
        list_state.select(selected_index);

        Self {
            results,
            selected_index,
            selected_expanded: false,
            total_calculation_time: total_time,
            list_state,
        }
    }

    pub fn select_next(&mut self) {
        if self.results.is_empty() {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(i) if i >= self.results.len() - 1 => 0,
            Some(i) => i + 1,
            None => 0,
        });

        self.list_state.select(self.selected_index);
        self.selected_expanded = false;
    }

    pub fn select_previous(&mut self) {
        if self.results.is_empty() {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(0) => self.results.len() - 1,
            Some(i) => i - 1,
            None => 0,
        });

        self.list_state.select(self.selected_index);
        self.selected_expanded = false;
    }

    pub fn page_down(&mut self, page_size: usize) {
        if self.results.is_empty() {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(i) => {
                let next = i + page_size;
                if next >= self.results.len() {
                    self.results.len() - 1
                } else {
                    next
                }
            }
            None => 0,
        });

        self.list_state.select(self.selected_index);
        self.selected_expanded = false;
    }

    pub fn page_up(&mut self, page_size: usize) {
        if self.results.is_empty() {
            return;
        }

        self.selected_index = Some(match self.selected_index {
            Some(i) => i.saturating_sub(page_size),
            None => 0,
        });

        self.list_state.select(self.selected_index);
        self.selected_expanded = false;
    }

    pub fn toggle_expand(&mut self) {
        self.selected_expanded = !self.selected_expanded;
    }

    pub fn selected_result(&self) -> Option<&PremiumResult> {
        self.selected_index.and_then(|i| self.results.get(i))
    }

    pub fn average_calculation_time_ms(&self) -> f64 {
        if self.results.is_empty() {
            0.0
        } else {
            self.total_calculation_time.as_secs_f64() * 1000.0 / self.results.len() as f64
        }
    }

    pub fn total_calculation_time_ms(&self) -> f64 {
        self.total_calculation_time.as_secs_f64() * 1000.0
    }
}
