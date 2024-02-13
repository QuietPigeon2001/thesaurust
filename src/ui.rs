use std::rc::Rc;

use ratatui::{
    layout::{Direction, Layout, Rect},
    prelude::Constraint,
};

use crate::{
    components::{
        antonym_block, banner_block, definition_block, example_block, footer, part_of_speech_block,
        popup, search_bar, synonym_block,
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

    let upper_frame = create_upper_layout(main_frame[0]);
    let lower_frame = create_lower_layout(main_frame[1]);
    let banner_frame = create_banner_layout(main_frame[1]);
    let right_frame = create_right_layout(lower_frame[1]);
    let footer_frame = create_footer_layout(main_frame[2]);
    let bottom_frame = create_bottom_layout(footer_frame[0]);

    match app.input_mode {
        InputMode::Suggesting => {
            f.render_widget(popup::new(app), upper_frame[0]);
        }
        _ => {
            f.render_widget(search_bar::new(app), upper_frame[0]);
            if !app.results.is_empty() {
                render_part_of_speech_block(app, f, lower_frame[0]);
                render_right_frame_components(app, f, right_frame);
                render_bottom_block(app, f, bottom_frame);
            } else {
                f.render_widget(banner_block::new(), banner_frame[0]);
            }
        }
    }
    render_instructions(app, f, footer_frame);
}

/// HELPER
fn create_banner_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .constraints([Constraint::Percentage(100)])
        .margin(1)
        .split(area)
}

fn create_right_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        // Definitions(50%), Examples(50%)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area)
}

fn create_upper_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        // Search bar (100%)
        .constraints([Constraint::Percentage(100)].as_ref())
        .horizontal_margin(1)
        .split(area)
}

fn create_footer_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
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
        .split(area)
}

fn create_lower_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        // Part of speech (20%), right frame (80%)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .horizontal_margin(1)
        .split(area)
}

fn create_bottom_layout(area: Rect) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        // Synonyms(50%), Antonyms(50%)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area)
}

fn render_bottom_block(app: &mut App, f: &mut Frame, bottom_frame: Rc<[Rect]>) {
    render_synonym_block(app, f, bottom_frame[0]);
    render_antonym_block(app, f, bottom_frame[1]);
}

fn render_synonym_block(app: &mut App, f: &mut Frame, area: Rect) {
    let mut cloned_state = app.synonym_list.state.clone();
    f.render_stateful_widget(synonym_block::new(app), area, &mut cloned_state);
}

fn render_antonym_block(app: &mut App, f: &mut Frame, area: Rect) {
    let mut cloned_state = app.antonym_list.state.clone();
    f.render_stateful_widget(antonym_block::new(app), area, &mut cloned_state);
}

fn render_right_frame_components(app: &mut App, f: &mut Frame, right_frame: Rc<[Rect]>) {
    let pos_list_idx = app.part_of_speech_list.state.selected().unwrap_or(0);
    let definition_list_idx = app.definition_list.state.selected().unwrap_or(0);
    let definitions = Thesaurus::unwrap_meanings_at(pos_list_idx, &app.results[0]).1;
    let d = definitions[definition_list_idx].clone();
    let definition = d.definition.unwrap_or("".to_string());
    let example = d.example.unwrap_or("".to_string());
    f.render_widget(
        definition_block::new(app, definitions, definition),
        right_frame[0],
    );
    f.render_widget(example_block::new(example), right_frame[1]);
}

fn render_part_of_speech_block(app: &mut App, f: &mut Frame, area: Rect) {
    let meanings = app.results[0].meanings.clone();
    if meanings.is_some() {
        let mut cloned_state = app.part_of_speech_list.state.clone();
        f.render_stateful_widget(part_of_speech_block::new(app), area, &mut cloned_state);
    }
}

fn render_instructions(app: &mut App, f: &mut Frame, frame: Rc<[Rect]>) {
    let instructions = App::update_instructions(app);
    f.render_widget(footer::with(&instructions), frame[1]);
    f.render_widget(footer::with("default"), frame[2]);
}
