mod api_key;
mod banner;
mod client;
mod components;
mod models;
mod tui;
mod ui;

use anyhow::Result;
use client::parse_response;
use crossterm::event::{self, Event, KeyCode};
use models::{
    app::{App, InputMode},
    list,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tui::Tui;
use tui_input::backend::crossterm::EventHandler;

fn main() -> Result<()> {
    let mut app = App::new();
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let mut tui = Tui::new(terminal);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        tui.draw(&mut app)?;
        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => {
                        App::quit(&mut app);
                    }
                    KeyCode::Char('j') | KeyCode::Char('k') if !app.results.is_empty() => {
                        app.input_mode = InputMode::SelectPartOfSpeech;
                    }
                    KeyCode::Char('l') | KeyCode::Char('h')
                        if app.part_of_speech_list.items.len() == 1 =>
                    {
                        app.input_mode = InputMode::SelectDefinition;
                    }
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Editing;
                        app.input.reset();
                    }
                    KeyCode::Char(':') => {
                        app.input_mode = InputMode::Settings;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.input_mode = InputMode::Normal;
                        let results =
                            parse_response(app.input.to_string(), app.is_spelling_fix_enabled);
                        app.results = results.t;
                        app.suggested_spelling = app.results[0].clone().word.unwrap();
                        if results.is_spelling_suggested {
                            //TODO:Error handling for non-spellchecking mode.
                            app.input_mode = InputMode::Suggesting;
                        }

                        App::update_stateful_lists(&mut app, list::StatefulListType::All);
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {
                        app.input.handle_event(&Event::Key(key));
                    }
                },
                InputMode::SelectPartOfSpeech => match key.code {
                    KeyCode::Char('j') => {
                        app.part_of_speech_list.down();
                    }
                    KeyCode::Char('k') => {
                        app.part_of_speech_list.up();
                    }
                    KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                    }
                    KeyCode::Enter => {
                        app.input_mode = InputMode::SelectDefinition;
                        App::update_stateful_lists(&mut app, list::StatefulListType::Definition);
                        App::update_stateful_lists(&mut app, list::StatefulListType::Synonym);
                        App::update_stateful_lists(&mut app, list::StatefulListType::Antonym);
                    }
                    _ => {}
                },
                InputMode::SelectDefinition => match key.code {
                    KeyCode::Char('l') => {
                        app.definition_list.down();
                        App::update_stateful_lists(&mut app, list::StatefulListType::Synonym);
                        App::update_stateful_lists(&mut app, list::StatefulListType::Antonym);
                    }
                    KeyCode::Char('h') => {
                        app.definition_list.up();
                        App::update_stateful_lists(&mut app, list::StatefulListType::Synonym);
                        App::update_stateful_lists(&mut app, list::StatefulListType::Antonym);
                    }
                    KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                        app.definition_list.state.select(Some(0));
                    }
                    KeyCode::Char('/') => {
                        app.input_mode = InputMode::Editing;
                        app.input.reset();
                    }
                    _ => {}
                },
                InputMode::Suggesting => match key.code {
                    KeyCode::Char('y') | KeyCode::Enter => {
                        app.input_mode = InputMode::Normal;
                        let results = parse_response(
                            app.suggested_spelling.clone(),
                            app.is_spelling_fix_enabled,
                        );
                        // Prevents Serp API from suggesting the same word repeatedly.
                        if !results.is_spelling_suggested {
                            app.results = results.t;
                            App::update_stateful_lists(&mut app, list::StatefulListType::All);
                        }
                    }
                    KeyCode::Char('n') | KeyCode::Char('q') => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                InputMode::Settings => match key.code {
                    KeyCode::Char('q') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('h') | KeyCode::Char('l') => {
                        app.is_spelling_fix_enabled = !app.is_spelling_fix_enabled;
                    }
                    _ => {}
                },
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
