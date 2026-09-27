use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::Direction::{self};
use ratatui::{
    layout::{Constraint, Layout, Spacing},
    symbols::merge::MergeStrategy,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::io;

mod branch_data;
mod parse;

rust_i18n::i18n!("locales", fallback = "en");

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let data = match branch_data::load_branch_data() {
        Ok(data) => data,
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
            let vertical_outer_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Percentage(100)])
                .split(area);

            let horizontal_inner_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(vertical_outer_layout[0]);

            let chunks = Layout::vertical([
                Constraint::Percentage(5),
                Constraint::Fill(1),
                Constraint::Percentage(15),
            ])
            .split(horizontal_inner_layout[0]);

            let changes_layout = Layout::default()
                .direction(Direction::Vertical)
                .spacing(Spacing::Overlap(1))
                .constraints(vec![
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                    Constraint::Fill(1),
                ])
                .split(chunks[1]);

            let staged_area = changes_layout[0];
            let unstaged_area = changes_layout[1];
            let untracked_area = changes_layout[2];

            let branch_text = format!("{}", data.branch.name);
            let branch_widget = Paragraph::new(branch_text).block(
                Block::default()
                    .title(rust_i18n::t!("title.branch"))
                    .borders(Borders::ALL),
            );
            frame.render_widget(branch_widget, chunks[0]);

            let files_staged: Vec<ListItem> = data
                .updated_staged
                .iter()
                .map(|f| {
                    let line = format!("{} {} {} {}", f.status, f.name, f.added, f.removed);
                    ListItem::new(line)
                })
                .collect();
            let files_staged_widget = List::new(files_staged).block(
                Block::bordered()
                    .title(rust_i18n::t!("title.staged"))
                    .borders(Borders::ALL)
                    .merge_borders(MergeStrategy::Exact),
            );
            frame.render_widget(files_staged_widget, staged_area);

            let files_unstaged: Vec<ListItem> = data
                .updated_unstaged
                .iter()
                .map(|f| {
                    let line = format!("{} {} {} {}", f.status, f.name, f.added, f.removed);
                    ListItem::new(line)
                })
                .collect();
            let files_unstaged_widget = List::new(files_unstaged).block(
                Block::bordered()
                    .title(rust_i18n::t!("title.unstaged"))
                    .borders(Borders::ALL)
                    .merge_borders(MergeStrategy::Exact),
            );
            frame.render_widget(files_unstaged_widget, unstaged_area);

            let files_untracked: Vec<ListItem> = data
                .updated_untracked
                .iter()
                .map(|f| {
                    let line = format!("{} {} {} {}", f.status, f.name, f.added, f.removed);
                    ListItem::new(line)
                })
                .collect();
            let files_untracked_widget = List::new(files_untracked).block(
                Block::bordered()
                    .title(rust_i18n::t!("title.untracked"))
                    .borders(Borders::ALL)
                    .merge_borders(MergeStrategy::Exact),
            );
            frame.render_widget(files_untracked_widget, untracked_area);

            let commits_items: Vec<ListItem> = data
                .last_commits
                .iter()
                .map(|c| {
                    let line = format!(
                        "{} - {} ({}) - [{}]",
                        c.short_hash, c.message, c.date, c.author
                    );
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

        if let event::Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => {}
                KeyCode::Up => {}
                KeyCode::Enter => {}
                _ => {}
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
