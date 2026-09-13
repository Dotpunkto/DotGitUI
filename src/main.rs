use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, widgets::*};
use std::io;

mod app;
mod parse;

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
            let text2 = data.branch.name.as_str();
            let paragraph2 = Paragraph::new(text2);

            frame.render_widget(paragraph2, area);
            // let area = frame.area();

            // //let text = "Git dashboard (q = quit, r = refresh)";
            // let text2 = data.branch.name;
            // //let paragraph = Paragraph::new(text);
            // let paragraph2 = Paragraph::new(text2);

            // frame.render_widget(paragraph2, area);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
}

fn main() {
    let terminal = ratatui::init();
    let res = run(terminal);
    ratatui::restore();
    res.expect("")
}
