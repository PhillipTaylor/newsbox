mod app;
mod input;
mod model;
mod rss;
mod ui;

use anyhow::Result;
use app::App;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use reqwest::Client;
use std::io::{self, Stdout};
use tokio::sync::mpsc;
use std::process::{Command, Stdio};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = enter_tui()?;
    let res = run(&mut terminal).await;
    let _ = leave_tui(&mut terminal);
    res
}

async fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;

    let feeds = match rss::load_feeds_from_yaml() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error loading feeds.yml:\n{:#}", e);
            std::process::exit(1);
        }
    };

    let (tx, mut rx) = mpsc::channel::<Result<Vec<model::Article>>>(1);

    let mut app = App::new();
    let mut filter_mode = false;

    loop {
        // Handle background refresh results
        while let Ok(msg) = rx.try_recv() {
            app.loading = false;
            match msg {
                Ok(items) => app.set_articles(items),
                Err(e) => app.status = format!("Refresh failed: {:#}", e),
            }
        }

        terminal.draw(|f| ui::draw(f, &app, filter_mode))?;

        match input::poll_action(filter_mode)? {
            input::Action::Quit => break,
            input::Action::Down => app.move_down(),
            input::Action::Up => app.move_up(),
            input::Action::ToggleFull => app.show_full = !app.show_full,

            input::Action::Refresh => {
                if !app.loading {
                    app.loading = true;
                    app.status = "Refreshing…".to_string();
                    let tx = tx.clone();
                    let client = client.clone();
                    let feeds = feeds.clone();
                    tokio::spawn(async move {
                        let out = rss::fetch_all(&client, &feeds).await;
                        let _ = tx.send(out).await;
                    });
                }
            }

            input::Action::OpenInBrowser => {
                if let Some(a) = app.selected_article() {
                    if let Err(e) = open::that(&a.link) {
                        app.status = format!("Could not open browser: {}", e);
                    } else {
                        app.status = "Opened in browser.".to_string();
                    }
                }
            }

            input::Action::StartFilter => {
                // Toggle filter mode
                filter_mode = !filter_mode;
                if filter_mode {
                    app.status = "Filter mode: type to filter, Enter/Esc to exit, Ctrl+U clears".to_string();
                } else {
                    app.apply_filter();
                    app.status = format!("Filter applied ({} results)", app.filtered.len());
                }
            }

            input::Action::Backspace => {
                app.filter.pop();
            }
            input::Action::FilterChar(c) => {
                app.filter.push(c);
            }
            input::Action::ClearFilter => {
                app.filter.clear();
                app.apply_filter();
                app.status = "Filter cleared.".to_string();
                filter_mode = false;
            }
            input::Action::OpenInW3m => {
                if let Some(a) = app.selected_article() {
                    match open_in_w3m(terminal, &a.link) {
                        Ok(()) => app.status = "Returned from w3m.".to_string(),
                        Err(e) => app.status = format!("Could not launch w3m: {}", e),
                    }
                }
            }


            input::Action::None => {}
        }
    }

    Ok(())
}

fn open_in_w3m(terminal: &mut Terminal<CrosstermBackend<Stdout>>, url: &str) -> Result<()> {
    // Leave TUI so w3m can use the terminal normally
    leave_tui(terminal)?;

    // Run w3m attached to the current terminal session
    let status = Command::new("w3m")
        .arg(url)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    // Re-enter TUI no matter what
    *terminal = enter_tui()?;

    // Optional: surface non-zero exit as a status message upstream
    match status {
        Ok(_s) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

fn enter_tui() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

fn leave_tui(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

