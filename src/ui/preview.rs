use crate::app::App;
use crate::i18n::*;
use crate::ui::widgets;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Row, Table};
use ratatui::Frame;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(4), // 3-segment gauge (label + bar + legend)
            Constraint::Min(0),    // Summary table
            Constraint::Length(3), // Action bar
        ])
        .split(area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(translate_preview_title())
        .title_style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(block, area);

    if let Some(ref result) = app.scan_result {
        let expiry = app.settings.expiry_days;
        let will_clean = result.reclaimable_size(expiry);
        let matchable = result.matchable_size(expiry);
        let skipped = matchable.saturating_sub(will_clean);
        let total = result.total_size;
        let _kept = total.saturating_sub(matchable);

        // Ratios for the 3-segment bar
        let r_clean = if total > 0 {
            will_clean as f64 / total as f64
        } else {
            0.0
        };
        let r_skip = if total > 0 {
            skipped as f64 / total as f64
        } else {
            0.0
        };

        let bar_width = chunks[0].width.saturating_sub(2) as usize;
        let green_w = ((r_clean * bar_width as f64).round() as usize)
            .max(if will_clean > 0 { 1 } else { 0 })
            .min(bar_width);
        let yellow_w = ((r_skip * bar_width as f64).round() as usize)
            .max(if skipped > 0 { 1 } else { 0 })
            .min(bar_width.saturating_sub(green_w));
        let red_w = bar_width.saturating_sub(green_w + yellow_w);

        // Line 1: sizes
        let label_line = Line::from(vec![
            Span::styled(
                format!("{}: ", translate_preview_clean_label()),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                widgets::format_size(will_clean),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                format!("  {}: ", translate_preview_skipped_label()),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                widgets::format_size(skipped),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled(
                format!("  {}: ", translate_preview_total_label()),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled(
                widgets::format_size(total),
                Style::default().fg(Color::White),
            ),
        ]);

        // Line 2: bar
        let bar_line = Line::from(vec![
            Span::styled(
                "\u{2588}".repeat(green_w),
                Style::default().fg(Color::Green),
            ),
            Span::styled(
                "\u{2588}".repeat(yellow_w),
                Style::default().fg(Color::Yellow),
            ),
            Span::styled("\u{2588}".repeat(red_w), Style::default().fg(Color::Red)),
        ]);

        // Line 3: legend
        let legend_line = Line::from(vec![
            Span::styled(" \u{25A0}", Style::default().fg(Color::Green)),
            Span::styled(translate_preview_legend_clean(), Style::default().fg(Color::DarkGray)),
            Span::styled("\u{25A0}", Style::default().fg(Color::Yellow)),
            Span::styled(
                translate_preview_legend_unselected(),
                Style::default().fg(Color::DarkGray),
            ),
            Span::styled("\u{25A0}", Style::default().fg(Color::Red)),
            Span::styled(translate_preview_legend_kept(), Style::default().fg(Color::DarkGray)),
        ]);

        let bar = Paragraph::new(vec![label_line, bar_line, legend_line]).block(Block::default());
        f.render_widget(bar, chunks[0]);

        // Summary table
        let header = Row::new(vec![
            translate_preview_table_category(),
            translate_preview_table_files(),
            translate_preview_table_size(),
            translate_preview_table_action(),
        ])
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .bottom_margin(1);

        let mut rows: Vec<Row> = Vec::new();

        for cat in &result.categories {
            if !cat.selected {
                continue;
            }
            let action = if cat.category.is_trim_only() {
                translate_preview_action_trim().to_string()
            } else {
                format!("{}（大于{}天）", translate_preview_action_delete(), expiry)
            };

            let exp_count = cat.expired_count(expiry);
            let exp_size = cat.expired_size(expiry);

            rows.push(
                Row::new(vec![
                    cat.category.to_string(),
                    format!("{}", exp_count),
                    widgets::format_size(exp_size),
                    action,
                ])
                .style(Style::default().fg(Color::Yellow)),
            );
        }

        // Selected projects (if not covered by Projects category)
        let proj_cat_selected = result
            .categories
            .iter()
            .any(|c| c.category == crate::model::Category::Projects && c.selected);

        if !proj_cat_selected {
            let selected_projects: Vec<_> = result.projects.iter().filter(|p| p.selected).collect();
            if !selected_projects.is_empty() {
                let total_proj_size: u64 = selected_projects
                    .iter()
                    .map(|p| p.expired_size(expiry))
                    .sum();
                let total_proj_files: usize = selected_projects
                    .iter()
                    .map(|p| p.expired_count(expiry))
                    .sum();
                let orphan_count = selected_projects.iter().filter(|p| p.is_orphan).count();
                let active_count = selected_projects.len() - orphan_count;
                let label = if orphan_count > 0 && active_count > 0 {
                    format!(
                        "项目{}",
                        translate_format_orphan_active(orphan_count, active_count)
                    )
                } else if orphan_count > 0 {
                    format!(
                        "项目{}",
                        translate_format_orphan(orphan_count)
                    )
                } else {
                    format!(
                        "项目{}",
                        translate_format_active(active_count)
                    )
                };
                rows.push(
                    Row::new(vec![
                        label,
                        format!("{}", total_proj_files),
                        widgets::format_size(total_proj_size),
                        format!("{}（大于{}天/孤立）", translate_preview_action_delete_orphan(), expiry),
                    ])
                    .style(Style::default().fg(Color::Red)),
                );
            }
        }

        // Config JSON cleanup
        let cj = &result.config_json;
        let cj_reclaimable = cj.reclaimable_size();
        if cj_reclaimable > 0 {
            let mut parts = Vec::new();
            if cj.orphan_projects_selected {
                parts.push("orphans");
            }
            if cj.metrics_selected {
                parts.push("metrics");
            }
            if cj.cache_selected {
                parts.push("caches");
            }
            rows.push(
                Row::new(vec![
                    translate_preview_config_json_label().to_string(),
                    "1".to_string(),
                    widgets::format_size(cj_reclaimable),
                    format!("{}（{}）", translate_preview_config_clean_label(), translate_preview_config_parts(&parts.to_vec())),
                ])
                .style(Style::default().fg(Color::Magenta)),
            );
        }

        let table = Table::new(
            rows,
            &[
                Constraint::Length(28),
                Constraint::Length(8),
                Constraint::Length(10),
                Constraint::Min(15),
            ],
        )
        .header(header)
        .block(Block::default());
        f.render_widget(table, chunks[1]);

        // Action bar
        let start_label = if app.settings.dry_run {
            translate_preview_start_dry_run()
        } else {
            translate_preview_start_clean()
        };
        let mut action_spans = vec![
            Span::raw("  "),
            Span::styled(
                "[Enter]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(start_label),
            Span::styled("[Esc]", Style::default().fg(Color::Yellow)),
            Span::raw(" "),
            Span::styled(translate_preview_go_back(), Style::default().fg(Color::White)),
            Span::raw(format!(
                "  总计：{} 个文件，{}",
                result.selected_file_count(expiry),
                widgets::format_size(will_clean),
            )),
        ];
        if app.settings.dry_run {
            action_spans.push(Span::styled(
                format!("  [{}]", translate_preview_dry_run_tag()),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        let action_bar = Paragraph::new(Line::from(action_spans));
        f.render_widget(action_bar, chunks[2]);
    } else {
        let p = Paragraph::new(translate_preview_no_data())
            .style(Style::default().fg(Color::DarkGray));
        f.render_widget(p, chunks[1]);
    }
}
