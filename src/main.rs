use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::{
    layout::{Constraint, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;

mod app;
mod parse;

rust_i18n::i18n!("locales", fallback = "en");

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let data = match app::load_app() {
        Ok(app_data) => app_data,
        Err(erreur) => {
            eprintln!("Loading error : {erreur}");
            eprintln!("Appuyez sur Entrée pour quitter...");
            let mut ligne = String::new();
            let _ = std::io::stdin().read_line(&mut ligne);
            return Ok(());
        }
    };

    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            // 1. Découpage vertical en 3 zones
            let chunks = Layout::vertical([
                Constraint::Length(3), // branche
                Constraint::Min(9),    // fichiers
                Constraint::Min(2),    // commits
            ])
            .split(area);

            // 2. Branche courante
            let branch_text = format!("{}", data.branch.name);
            let branch_widget = Paragraph::new(branch_text).block(
                Block::default()
                    .title(rust_i18n::t!("title.branch"))
                    .borders(Borders::ALL),
            );

            frame.render_widget(branch_widget, chunks[0]);

            // 3. Fichiers modifiés
            let files_items: Vec<ListItem> = data
                .updated_files
                .iter()
                .map(|f| {
                    let line = format!("{} - {}", f.status, f.name);
                    ListItem::new(line)
                })
                .collect();

            let files_widget = List::new(files_items).block(
                Block::default()
                    .title(rust_i18n::t!("title.updated_files"))
                    .borders(Borders::ALL),
            );

            frame.render_widget(files_widget, chunks[1]);

            // 4. Derniers commits
            let commits_items: Vec<ListItem> = data
                .last_commits
                .iter()
                .map(|c| {
                    let line = format!("{} - {} ({})", c.short_hash, c.message, c.author);
                    ListItem::new(line)
                })
                .collect();

            let commits_widget = List::new(commits_items).block(
                Block::default()
                    .title(rust_i18n::t!("title.commits"))
                    .borders(Borders::ALL),
            );

            frame.render_widget(commits_widget, chunks[2]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn main() {
    rust_i18n::set_locale("en");

    let terminal = ratatui::init();
    let res = run(terminal);

    ratatui::restore();
    res.expect("")
}
