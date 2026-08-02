use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use rayan_daemon::{GenerationRecord, RollbackTracker};
use std::io;

pub async fn run_tui() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Fetch history
    let tracker = RollbackTracker::new("history.db");
    let records = if let Ok(t) = tracker {
        t.list_generations().unwrap_or_default()
    } else {
        vec![]
    };

    // Run app loop
    let res = run_app(&mut terminal, records);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    records: Vec<GenerationRecord>,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.size());

            let title = Paragraph::new("Rayan Rollback Engine - Git Log")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let items: Vec<ListItem> = records
                .iter()
                .map(|r| {
                    ListItem::new(format!(
                        "Gen {}: {} -> {}",
                        r.id, r.timestamp, r.nix_store_path
                    ))
                })
                .collect();

            let list = List::new(items)
                .block(Block::default().title("Generations").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));

            f.render_widget(list, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}
