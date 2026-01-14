use crate::domain::PremiumResult;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// Renders the summary view for a premium result
pub fn render_summary(result: &PremiumResult) -> Vec<Line<'_>> {
    vec![
        Line::from(vec![
            Span::styled("Name: ", Style::default().fg(Color::Cyan)),
            Span::raw(&result.applicant.name),
        ]),
        Line::from(vec![
            Span::styled("Age: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{} | ", result.applicant.age)),
            Span::styled("Gender: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!("{} | ", result.applicant.gender)),
            Span::styled("Smoker: ", Style::default().fg(Color::Cyan)),
            Span::raw(if result.applicant.smoker { "Yes" } else { "No" }),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Coverage: ", Style::default().fg(Color::Cyan)),
            Span::raw(format!(
                "${:.0} for {} years",
                result.applicant.coverage_amount, result.applicant.coverage_years
            )),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Monthly Premium: ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("${:.2}", result.final_premium),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Calculation Time: ", Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:.3}ms", result.calculation_time_ms),
                Style::default().fg(Color::Magenta),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press Enter/Space to see detailed breakdown",
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::ITALIC),
        )),
    ]
}

/// Renders the expanded/detailed view for a premium result
pub fn render_detailed(result: &PremiumResult) -> Vec<Line<'_>> {
    vec![
        Line::from(vec![Span::styled(
            "━━━ APPLICANT INFO ━━━",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![
            Span::raw("Name: "),
            Span::styled(&result.applicant.name, Style::default().fg(Color::White)),
        ]),
        Line::from(format!(
            "Age: {} | Gender: {} | Occupation: {}",
            result.applicant.age, result.applicant.gender, result.applicant.occupation
        )),
        Line::from(format!(
            "Income: ${:.0} | Coverage: ${:.0} for {} years",
            result.applicant.annual_income,
            result.applicant.coverage_amount,
            result.applicant.coverage_years
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "━━━ HEALTH METRICS ━━━",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!(
            "BMI: {:.1} | Blood Pressure: {}/{}",
            result.applicant.bmi,
            result.applicant.blood_pressure_sys,
            result.applicant.blood_pressure_dia
        )),
        Line::from(format!("Cholesterol: {} mg/dL", result.applicant.cholesterol)),
        Line::from(format!(
            "Existing Conditions: {}",
            if result.applicant.existing_conditions == "none" {
                "None"
            } else {
                &result.applicant.existing_conditions
            }
        )),
        Line::from(format!(
            "Smoker: {} | Family History Score: {}/6",
            if result.applicant.smoker { "Yes" } else { "No" },
            result.applicant.family_history_score
        )),
        Line::from(""),
        Line::from(vec![Span::styled(
            "━━━ PREMIUM CALCULATION ━━━",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(format!(
            "Base Premium:          ${:.2}",
            result.base_premium
        )),
        Line::from(format!("Age Factor:            x{:.2}", result.age_factor)),
        Line::from(format!(
            "Health Risk Score:     x{:.3}",
            result.health_risk_score
        )),
        Line::from(format!(
            "Lifestyle Multiplier:  x{:.2}",
            result.lifestyle_multiplier
        )),
        Line::from(format!(
            "Occupation Factor:     x{:.2}",
            result.occupation_factor
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "MONTHLY PREMIUM:       ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("${:.2}", result.final_premium),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Calculation Time: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.3}ms", result.calculation_time_ms),
                Style::default().fg(Color::Magenta),
            ),
        ]),
    ]
}
