use crate::ui::{app_state::AppState, renderer};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Renders the complete UI
/// Follows Single Responsibility Principle - only handles UI rendering
pub fn render(f: &mut Frame, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    render_header(f, chunks[0], state);
    render_main_content(f, chunks[1], state);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: ratatui::layout::Rect, state: &AppState) {
    let header_text = vec![Line::from(vec![
        Span::styled(
            "Premium Calculator ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("| {} applicants calculated in ", state.results.len())),
        Span::styled(
            format!("{:.2}ms", state.total_calculation_time_ms()),
            Style::default().fg(Color::Green),
        ),
        Span::raw(format!(
            " | Avg: {:.2}ms",
            state.average_calculation_time_ms()
        )),
    ])];

    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL).title("Insurance Premium Calculator"));
    
    f.render_widget(header, area);
}

fn render_main_content(f: &mut Frame, area: ratatui::layout::Rect, state: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(area);

    render_applicant_list(f, main_chunks[0], state);
    render_detail_panel(f, main_chunks[1], state);
}

fn render_applicant_list(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    state: &AppState,
) {
    let items: Vec<ListItem> = state
        .results
        .iter()
        .enumerate()
        .map(|(i, result)| {
            let content = format!(
                "{:2}. {} - ${:.2}/mo ({:.2}ms)",
                i + 1,
                result.applicant.name,
                result.final_premium,
                result.calculation_time_ms
            );
            
            let style = if state.selected_index == Some(i) {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Applicants"))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(list, area);
}

fn render_detail_panel(
    f: &mut Frame,
    area: ratatui::layout::Rect,
    state: &AppState,
) {
    if let Some(result) = state.selected_result() {
        let detail_text = if state.selected_expanded {
            renderer::render_detailed(result)
        } else {
            renderer::render_summary(result)
        };

        let detail = Paragraph::new(detail_text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Details (Press Enter/Space to expand)"),
            )
            .wrap(Wrap { trim: false });

        f.render_widget(detail, area);
    }
}

fn render_footer(f: &mut Frame, area: ratatui::layout::Rect) {
    let footer = Paragraph::new("↑/↓ or j/k: Navigate | Enter/Space: Expand | q: Quit")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Gray));
    
    f.render_widget(footer, area);
}
