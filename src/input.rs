use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Quit,
    Down,
    Up,
    ToggleFull,
    Refresh,
    OpenInBrowser,
    OpenInW3m,
    StartFilter,
    Backspace,
    FilterChar(char),
    ClearFilter,
    None,
}

pub fn poll_action(filter_mode: bool) -> anyhow::Result<Action> {
    if !event::poll(Duration::from_millis(50))? {
        return Ok(Action::None);
    }

    match event::read()? {
        Event::Key(KeyEvent { code, modifiers, .. }) => {
            if filter_mode {
                return Ok(match (code, modifiers) {
                    (KeyCode::Esc, _) => Action::ClearFilter,
                    (KeyCode::Enter, _) => Action::StartFilter, // end filter mode toggle handled in main
                    (KeyCode::Backspace, _) => Action::Backspace,
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => Action::ClearFilter,
                    (KeyCode::Char(c), _) => Action::FilterChar(c),
                    _ => Action::None,
                });
            }

            Ok(match (code, modifiers) {
                (KeyCode::Char('q'), _) => Action::Quit,
                (KeyCode::Char('j'), _) | (KeyCode::Down, _) => Action::Down,
                (KeyCode::Char('k'), _) | (KeyCode::Up, _) => Action::Up,
                (KeyCode::Enter, _) => Action::ToggleFull,
                (KeyCode::Char('r'), _) => Action::Refresh,
                (KeyCode::Char('o'), _) => Action::OpenInBrowser,
                (KeyCode::Char('p'), _) => Action::OpenInW3m,
                (KeyCode::Char('/'), _) => Action::StartFilter,
                _ => Action::None,
            })
        }
        _ => Ok(Action::None),
    }
}

