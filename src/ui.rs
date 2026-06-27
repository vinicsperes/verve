use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Padding, Paragraph};
use ratatui::Frame;

use crate::app::App;
use crate::confetti::Particle;

const ACCENT: Color = Color::Rgb(215, 119, 87);
const TEXT: Color = Color::Rgb(225, 225, 225);
const DIM: Color = Color::Rgb(110, 110, 110);
const FAINT: Color = Color::Rgb(80, 80, 80);
const BORDER: Color = Color::Rgb(64, 64, 64);
const ERROR: Color = Color::Rgb(224, 108, 117);

const CONTENT_WIDTH: u16 = 64;

pub fn render(frame: &mut Frame, app: &App) {
    let width = CONTENT_WIDTH.min(frame.area().width.saturating_sub(2));
    let area = centered(frame.area(), width, 16);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(5),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    frame.render_widget(title(app), chunks[0]);

    let block = panel();
    let inner = block.inner(chunks[2]);
    frame.render_widget(block, chunks[2]);

    if app.is_finished() {
        frame.render_widget(results(app), inner);
    } else {
        let (lines, cursor) = typing_layout(app, inner.width);
        frame.render_widget(Paragraph::new(lines), inner);
        frame.set_cursor_position((inner.x + cursor.0, inner.y + cursor.1));
    }

    frame.render_widget(status(app), chunks[4]);

    if let Some(t) = app.celebration_elapsed() {
        draw_confetti(frame, &app.confetti, t);
    }
}

fn draw_confetti(frame: &mut Frame, particles: &[Particle], t: f32) {
    let area = frame.area();
    if area.width == 0 || area.height == 0 {
        return;
    }

    let buffer = frame.buffer_mut();
    for particle in particles {
        let (x, y) = particle.position(t);
        if !(0.0..1.0).contains(&x) || !(0.0..1.0).contains(&y) {
            continue;
        }

        let col = area.left() + (x * area.width as f32) as u16;
        let row = area.top() + (y * area.height as f32) as u16;
        if col >= area.right() || row >= area.bottom() {
            continue;
        }

        let (r, g, b) = particle.color;
        let cell = &mut buffer[(col, row)];
        cell.set_char(particle.glyph);
        cell.set_fg(Color::Rgb(r, g, b));
    }
}

fn title(app: &App) -> Paragraph<'static> {
    let typed_words = if app.cursor() == 0 {
        0
    } else {
        app.target[..app.cursor()]
            .iter()
            .filter(|&&c| c == ' ')
            .count()
            + 1
    };

    Paragraph::new(Line::from(vec![
        Span::styled("\u{25cf} ", Style::default().fg(ACCENT)),
        Span::styled(
            "verve",
            Style::default().fg(TEXT).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!(
                "   {} / {}",
                typed_words.min(app.word_count),
                app.word_count
            ),
            Style::default().fg(FAINT),
        ),
    ]))
}

fn panel() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(BORDER))
        .padding(Padding::new(2, 2, 1, 1))
}

fn typing_layout(app: &App, inner_width: u16) -> (Vec<Line<'static>>, (u16, u16)) {
    let width = inner_width.max(1) as usize;
    let target = &app.target;
    let n = target.len();
    let cursor = app.cursor();

    let mut pos = vec![(0u16, 0u16); n + 1];
    let mut drawn = vec![true; n];
    let mut row = 0usize;
    let mut col = 0usize;

    let mut i = 0;
    while i < n {
        let start = i;
        while i < n && target[i] != ' ' {
            i += 1;
        }
        let word_len = i - start;

        if col > 0 {
            if col + 1 + word_len > width {
                pos[start - 1] = (0, (row + 1) as u16);
                drawn[start - 1] = false;
                row += 1;
                col = 0;
            } else {
                pos[start - 1] = (col as u16, row as u16);
                col += 1;
            }
        }

        for cell in pos.iter_mut().take(i).skip(start) {
            if col >= width {
                row += 1;
                col = 0;
            }
            *cell = (col as u16, row as u16);
            col += 1;
        }

        if i < n {
            i += 1;
        }
    }
    pos[n] = (col as u16, row as u16);

    let mut rows: Vec<Vec<Span<'static>>> = vec![Vec::new(); row + 1];
    for (idx, &target_ch) in target.iter().enumerate() {
        if !drawn[idx] {
            continue;
        }
        let style = if idx < cursor {
            if app.typed[idx] == target_ch {
                Style::default().fg(TEXT)
            } else {
                Style::default()
                    .fg(ERROR)
                    .add_modifier(Modifier::UNDERLINED)
            }
        } else {
            Style::default().fg(DIM)
        };
        let (_, r) = pos[idx];
        rows[r as usize].push(Span::styled(target_ch.to_string(), style));
    }

    let lines = rows.into_iter().map(Line::from).collect();
    (lines, pos[cursor])
}

fn results(app: &App) -> Paragraph<'static> {
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{:.0}", app.wpm()),
                Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
            ),
            Span::styled(" wpm", Style::default().fg(DIM)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(format!("{:.0}%", app.accuracy()), Style::default().fg(TEXT)),
            Span::styled("  accuracy", Style::default().fg(DIM)),
        ]),
        Line::from(Span::styled(
            format!("{:.1}s", app.elapsed_secs()),
            Style::default().fg(FAINT),
        )),
        Line::from(""),
    ];

    if app.is_record() {
        lines.push(Line::from(Span::styled(
            "\u{2726}  new personal best  \u{2726}",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            format!("best  {:.0} wpm", app.best_wpm()),
            Style::default().fg(FAINT),
        )));
    }

    Paragraph::new(lines).alignment(Alignment::Center)
}

fn status(app: &App) -> Paragraph<'static> {
    let sep = Span::styled("   \u{2022}   ", Style::default().fg(FAINT));

    let line = if app.is_finished() {
        Line::from(vec![hint("tab", "restart"), sep, hint("esc", "quit")])
    } else if app.is_started() {
        Line::from(vec![
            Span::styled("\u{25cf} ", Style::default().fg(ACCENT)),
            Span::styled(format!("{:.0} wpm", app.wpm()), Style::default().fg(TEXT)),
            sep,
            Span::styled(
                format!("{:.0}s", app.elapsed_secs()),
                Style::default().fg(DIM),
            ),
        ])
    } else {
        Line::from(vec![
            Span::styled("start typing", Style::default().fg(DIM)),
            sep.clone(),
            hint("tab", "restart"),
            sep,
            hint("esc", "quit"),
        ])
    };

    Paragraph::new(line).alignment(Alignment::Center)
}

fn hint(key: &'static str, label: &'static str) -> Span<'static> {
    Span::styled(format!("{key} {label}"), Style::default().fg(FAINT))
}

fn centered(area: Rect, width: u16, height: u16) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(vertical[1])[1]
}
