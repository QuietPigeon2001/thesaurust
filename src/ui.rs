use ratatui::{
    layout::{Direction, Layout},
    prelude::Constraint,
};

use crate::{
    components::{
        banner_block, definition_block, example_block, footer, part_of_speech_block, popup,
        search_bar, synonym_block,
    },
    models::{
        app::{App, InputMode},
        data::Thesaurus,
    },
    tui::Frame,
};

pub fn render(app: &mut App, f: &mut Frame) {
    // Main frame.
    let main_frame = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(9),
                Constraint::Length(9),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    // The `upper_frame` consists of the search bar and the help bar.
    let upper_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .horizontal_margin(1)
        .split(main_frame[0]);

    let lower_frame = Layout::default()
        .direction(Direction::Horizontal)
        // Part of speech (20%), right frame (80%)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .horizontal_margin(1)
        .split(main_frame[1]);

    let banner_frame = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(main_frame[1]);

    // The `right_frame` consists of the `definition` block and the `example` block.
    let right_frame = Layout::default()
        .direction(Direction::Vertical)
        // Definitions(50%), Examples(50%)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(lower_frame[1]);

    // The `footer` consists of a spacer and two sets of instructions.
    let footer_frame = Layout::default()
        .direction(Direction::Vertical)
        // Spacer(50%), Instructions(25%), Default instructions(25%)
        .constraints(
            [
                Constraint::Percentage(80),
                Constraint::Percentage(10),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .horizontal_margin(1)
        .split(main_frame[2]);

    let bottom_frame = Layout::default()
        .direction(Direction::Horizontal)
        // Synonyms(50%), Antonyms(50%)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(footer_frame[0]);

    // Index of the item selected by the user in `part_of_speech_list`.
    let mut idx = 0;
    if let Some(i) = app.part_of_speech_list.state.selected() {
        idx = i;
    }

    match app.input_mode {
        InputMode::Suggesting => {
            f.render_widget(popup::new(app), upper_frame[0]);
        }
        _ => {
            f.render_widget(search_bar::new(app), upper_frame[0]);
            let mut definition = String::from("");
            let mut example = String::from("");
            if !app.results.is_empty() {
                let definitions = Thesaurus::unwrap_meanings_at(idx, &app.results[0]).1;
                if let Some(d_idx) = app.definition_list.state.selected() {
                    if let Some(d) = definitions[d_idx].definition.clone() {
                        definition = d;
                        if let Some(e) = definitions[d_idx].example.clone() {
                            example = e;
                        }
                    }
                }

                let meanings = app.results[0].meanings.clone();
                if meanings.is_some() {
                    let mut cloned_state = app.part_of_speech_list.state.clone();
                    f.render_stateful_widget(
                        part_of_speech_block::new(app),
                        lower_frame[0],
                        &mut cloned_state,
                    );
                }

                f.render_widget(
                    definition_block::new(app, definitions, definition),
                    right_frame[0],
                );
                f.render_widget(example_block::new(example), right_frame[1]);
                let mut cloned_state = app.synonym_list.state.clone();
                f.render_stateful_widget(
                    synonym_block::new(app),
                    bottom_frame[0],
                    &mut cloned_state,
                );
            } else {
                f.render_widget(banner_block::new(), banner_frame[0]);
            }
        }
    }

    let instructions = App::update_instructions(app);
    f.render_widget(footer::with(&instructions), footer_frame[1]);
    f.render_widget(footer::with("default"), footer_frame[2]);
}
