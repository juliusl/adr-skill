//! TUI for creating new problem files.
//!
//! Single-screen interactive form: navigate sections with single keys,
//! each opens `$EDITOR` for input. Preview and save the assembled problem.

use crossterm::{
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

/// Sections the user can fill in.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Section {
    Details,
    Goals,
    Constraints,
    Stakeholders,
    Decisions,
}

impl Section {
    fn all() -> &'static [Section] {
        &[
            Section::Details,
            Section::Goals,
            Section::Constraints,
            Section::Stakeholders,
            Section::Decisions,
        ]
    }

    fn key(&self) -> char {
        match self {
            Section::Details => 'd',
            Section::Goals => 'g',
            Section::Constraints => 'c',
            Section::Stakeholders => 's',
            Section::Decisions => 'n',
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Section::Details => "details",
            Section::Goals => "goals",
            Section::Constraints => "constraints",
            Section::Stakeholders => "stakeholders",
            Section::Decisions => "decisions needed",
        }
    }

    fn heading(&self) -> &'static str {
        match self {
            Section::Details => "Problem Statement",
            Section::Goals => "Goals",
            Section::Constraints => "Constraints",
            Section::Stakeholders => "Stakeholders",
            Section::Decisions => "Decisions Needed",
        }
    }

    fn placeholder(&self) -> &'static str {
        match self {
            Section::Details => "Describe the problem. What system is affected? What triggered this?",
            Section::Goals => "What does success look like? One goal per line.",
            Section::Constraints => "Known constraints, requirements, or boundaries. One per line.",
            Section::Stakeholders => "Who cares about this decision? One per line.",
            Section::Decisions => "What distinct decisions does this problem require? One per line.\nEach decision becomes one ADR.",
        }
    }

    fn from_key(c: char) -> Option<Section> {
        Section::all().iter().find(|s| s.key() == c).copied()
    }
}

/// State for the problem being built.
struct ProblemBuilder {
    title: String,
    sections: Vec<(Section, String)>,
}

impl ProblemBuilder {
    fn new(title: String) -> Self {
        Self {
            title,
            sections: Section::all().iter().map(|s| (*s, String::new())).collect(),
        }
    }

    fn get(&self, section: Section) -> &str {
        self.sections
            .iter()
            .find(|(s, _)| *s == section)
            .map(|(_, v)| v.as_str())
            .unwrap_or("")
    }

    fn set(&mut self, section: Section, value: String) {
        if let Some(entry) = self.sections.iter_mut().find(|(s, _)| *s == section) {
            entry.1 = value;
        }
    }

    fn is_filled(&self, section: Section) -> bool {
        !self.get(section).trim().is_empty()
    }

    fn filled_count(&self) -> usize {
        self.sections
            .iter()
            .filter(|(_, v)| !v.trim().is_empty())
            .count()
    }

    fn slug(&self) -> String {
        self.title
            .to_lowercase()
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>()
            .trim_matches('-')
            .to_string()
            .chars()
            .take(50)
            .collect()
    }

    fn to_markdown(&self) -> String {
        let date = chrono_date();
        let slug = self.slug();
        let decisions_count = if self.is_filled(Section::Decisions) {
            self.get(Section::Decisions).lines().filter(|l| !l.trim().is_empty()).count()
        } else {
            0
        };

        let mut md = format!(
            "---\ntitle: {title}\ndate: {date}\nstatus: intake\nslug: {slug}\ndecisions: {decisions_count}\n---\n\n# {title}\n",
            title = self.title,
        );

        for (section, content) in &self.sections {
            md.push_str(&format!("\n## {}\n\n", section.heading()));
            if content.trim().is_empty() {
                md.push_str(&format!("<!-- {} -->\n", section.placeholder().lines().next().unwrap_or("")));
            } else {
                // Format as list items for multi-line sections
                match section {
                    Section::Details => {
                        md.push_str(content.trim());
                        md.push('\n');
                    }
                    _ => {
                        for line in content.lines() {
                            let line = line.trim();
                            if !line.is_empty() {
                                if line.starts_with("- ") || line.starts_with("* ") || line.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                                    md.push_str(&format!("{line}\n"));
                                } else {
                                    md.push_str(&format!("- {line}\n"));
                                }
                            }
                        }
                    }
                }
            }
        }

        md
    }
}

fn chrono_date() -> String {
    // Simple date without chrono dependency
    let output = Command::new("date")
        .arg("+%Y-%m-%d")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        });
    output.unwrap_or_else(|| "2026-01-01".to_string())
}

fn get_editor() -> String {
    std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| "vi".to_string())
}

/// Open `$EDITOR` with initial content, return edited content.
fn open_editor(initial: &str, placeholder: &str) -> io::Result<String> {
    let mut tmpfile = tempfile::NamedTempFile::new()?;

    if initial.trim().is_empty() {
        // Write placeholder as comments
        for line in placeholder.lines() {
            writeln!(tmpfile, "# {line}")?;
        }
        writeln!(tmpfile, "# Lines starting with # are removed.")?;
        writeln!(tmpfile)?;
    } else {
        write!(tmpfile, "{initial}")?;
    }
    tmpfile.flush()?;

    let path = tmpfile.path().to_path_buf();
    let editor = get_editor();

    let status = Command::new(&editor).arg(&path).status()?;

    if !status.success() {
        return Ok(initial.to_string());
    }

    let content = std::fs::read_to_string(&path)?;
    // Strip comment lines
    let cleaned: Vec<&str> = content
        .lines()
        .filter(|l| !l.starts_with('#'))
        .collect();
    Ok(cleaned.join("\n").trim().to_string())
}

/// The TUI screen state.
enum Screen {
    Form,
    Preview,
}

/// Run the TUI for creating a new problem.
pub fn run_new_problem(title: String) -> io::Result<Option<PathBuf>> {
    let mut builder = ProblemBuilder::new(title);
    let mut screen = Screen::Form;
    let mut message: Option<String> = None;

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = run_loop(&mut terminal, &mut builder, &mut screen, &mut message);

    // Restore terminal
    terminal::disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    builder: &mut ProblemBuilder,
    screen: &mut Screen,
    message: &mut Option<String>,
) -> io::Result<Option<PathBuf>> {
    loop {
        terminal.draw(|frame| {
            match screen {
                Screen::Form => draw_form(frame, builder, message.as_deref()),
                Screen::Preview => draw_preview(frame, builder),
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match screen {
                Screen::Form => {
                    match key.code {
                        // Quit
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(None);
                        }
                        KeyCode::Char('q') => {
                            return Ok(None);
                        }
                        // Section keys
                        KeyCode::Char(c) => {
                            if let Some(section) = Section::from_key(c) {
                                // Suspend TUI, open editor
                                terminal::disable_raw_mode()?;
                                io::stdout().execute(LeaveAlternateScreen)?;

                                let current = builder.get(section).to_string();
                                match open_editor(&current, section.placeholder()) {
                                    Ok(edited) => {
                                        builder.set(section, edited);
                                        *message = Some(format!("Updated {}", section.label()));
                                    }
                                    Err(e) => {
                                        *message = Some(format!("Editor error: {e}"));
                                    }
                                }

                                // Resume TUI
                                io::stdout().execute(EnterAlternateScreen)?;
                                terminal::enable_raw_mode()?;
                            }
                        }
                        // Preview / save
                        KeyCode::Enter => {
                            *screen = Screen::Preview;
                            *message = None;
                        }
                        KeyCode::Esc => {
                            return Ok(None);
                        }
                        _ => {}
                    }
                }
                Screen::Preview => {
                    match key.code {
                        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                            return Ok(None);
                        }
                        // Save
                        KeyCode::Enter => {
                            let saved = save_problem(builder)?;
                            return Ok(Some(saved));
                        }
                        // Back to form
                        KeyCode::Esc | KeyCode::Char('q') => {
                            *screen = Screen::Form;
                            *message = Some("Back to editing".to_string());
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn draw_form(frame: &mut ratatui::Frame, builder: &ProblemBuilder, message: Option<&str>) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(3),  // title
        Constraint::Length(2),  // spacer + section header
        Constraint::Min(10),   // sections
        Constraint::Length(3),  // status bar
    ])
    .split(area);

    // Title
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(" new problem ");
    let title_text = Paragraph::new(Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled(&builder.title, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]))
    .block(title_block);
    frame.render_widget(title_text, chunks[0]);

    // Section header
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "  Press a key to edit that section:",
            Style::default().fg(Color::DarkGray),
        ),
    ]));
    frame.render_widget(header, chunks[1]);

    // Sections
    let mut lines: Vec<Line> = Vec::new();
    lines.push(Line::from(""));

    for section in Section::all() {
        let filled = builder.is_filled(*section);
        let indicator = if filled { "●" } else { "○" };
        let indicator_color = if filled { Color::Green } else { Color::DarkGray };

        let preview = if filled {
            let content = builder.get(*section);
            let first_line = content.lines().next().unwrap_or("");
            let line_count = content.lines().filter(|l| !l.trim().is_empty()).count();
            if line_count > 1 {
                format!("{} (+{} more)", truncate(first_line, 50), line_count - 1)
            } else {
                truncate(first_line, 60).to_string()
            }
        } else {
            "—".to_string()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("  {indicator} "), Style::default().fg(indicator_color)),
            Span::styled(
                format!("{}", section.key()),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::styled(format!("  {:<20}", section.label()), Style::default().fg(Color::White)),
            Span::styled(preview, Style::default().fg(Color::DarkGray)),
        ]));
        lines.push(Line::from(""));
    }

    let sections = Paragraph::new(lines);
    frame.render_widget(sections, chunks[2]);

    // Status bar
    let filled = builder.filled_count();
    let total = Section::all().len();

    let status_line = if let Some(msg) = message {
        Line::from(vec![
            Span::styled(format!("  {msg}  "), Style::default().fg(Color::Yellow)),
            Span::styled("│  ", Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{filled}/{total} filled"), Style::default().fg(Color::DarkGray)),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("[enter]", Style::default().fg(Color::Cyan)),
            Span::styled(" preview  ", Style::default().fg(Color::White)),
            Span::styled("[q/esc]", Style::default().fg(Color::Cyan)),
            Span::styled(" quit", Style::default().fg(Color::White)),
        ])
    } else {
        Line::from(vec![
            Span::styled(format!("  {filled}/{total} filled"), Style::default().fg(Color::DarkGray)),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::styled("[enter]", Style::default().fg(Color::Cyan)),
            Span::styled(" preview  ", Style::default().fg(Color::White)),
            Span::styled("[q/esc]", Style::default().fg(Color::Cyan)),
            Span::styled(" quit", Style::default().fg(Color::White)),
        ])
    };

    let status_block = Block::default().borders(Borders::TOP);
    let status = Paragraph::new(status_line).block(status_block);
    frame.render_widget(status, chunks[3]);
}

fn draw_preview(frame: &mut ratatui::Frame, builder: &ProblemBuilder) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Min(5),    // preview content
        Constraint::Length(3), // action bar
    ])
    .split(area);

    let md = builder.to_markdown();
    let lines: Vec<Line> = md
        .lines()
        .map(|l| {
            if l.starts_with('#') {
                Line::from(Span::styled(l, Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
            } else if l.starts_with("---") {
                Line::from(Span::styled(l, Style::default().fg(Color::DarkGray)))
            } else if l.starts_with("<!--") {
                Line::from(Span::styled(l, Style::default().fg(Color::DarkGray)))
            } else if l.starts_with("- ") {
                Line::from(Span::styled(l, Style::default().fg(Color::White)))
            } else {
                Line::from(l)
            }
        })
        .collect();

    let preview_block = Block::default()
        .borders(Borders::ALL)
        .title(" preview ");
    let preview = Paragraph::new(lines).block(preview_block);
    frame.render_widget(preview, chunks[0]);

    let action_line = Line::from(vec![
        Span::styled("  ", Style::default()),
        Span::styled("[enter]", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Span::styled(" save  ", Style::default().fg(Color::White)),
        Span::styled("[esc]", Style::default().fg(Color::Cyan)),
        Span::styled(" back to editing  ", Style::default().fg(Color::White)),
        Span::styled("[q]", Style::default().fg(Color::Cyan)),
        Span::styled(" quit without saving", Style::default().fg(Color::White)),
    ]);
    let action_block = Block::default().borders(Borders::TOP);
    let action = Paragraph::new(action_line).block(action_block);
    frame.render_widget(action, chunks[1]);
}

fn save_problem(builder: &ProblemBuilder) -> io::Result<PathBuf> {
    let slug = builder.slug();
    let problems_dir = PathBuf::from(".adr/problems");
    std::fs::create_dir_all(&problems_dir)?;

    let filepath = problems_dir.join(format!("{slug}.md"));
    if filepath.exists() {
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!("Problem already exists: {}", filepath.display()),
        ));
    }

    let md = builder.to_markdown();
    std::fs::write(&filepath, md)?;
    Ok(filepath)
}

fn truncate(s: &str, max: usize) -> &str {
    if s.len() <= max {
        s
    } else {
        &s[..max]
    }
}
