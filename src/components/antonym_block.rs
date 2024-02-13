use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use crate::models::app::App;

pub fn new(app: &mut App) -> List {
    let cloned_list = app.antonym_list.clone();
    let antonyms: Vec<ListItem> = cloned_list
        .items
        .iter()
        .map(|i| ListItem::new(i.clone()))
        .collect();
    let antonyms = List::new(antonyms)
        .block(Block::default().borders(Borders::ALL).title("Antonym"))
        .style(Style::default().fg(Color::Green))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Cyan));
    antonyms
}
