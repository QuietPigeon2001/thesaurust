use tui_input::Input;

use crate::models::{
    data::Thesaurus,
    list::{StatefulList, StatefulListType},
};

#[derive(Clone, Debug)]
pub enum InputMode {
    Normal,
    Editing,
    SelectPartOfSpeech,
    SelectDefinition,
}

impl Default for InputMode {
    fn default() -> Self {
        InputMode::Normal
    }
}

/// Application.
#[derive(Clone, Debug, Default)]
pub struct App {
    pub should_quit: bool,
    pub input: Input,
    pub input_mode: InputMode,
    pub results: Vec<Thesaurus>,
    pub part_of_speech_list: StatefulList<String>,
    pub definition_list: StatefulList<String>,
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Update the instructions in the footer depending on the `input_mode`.
    pub fn update_instructions(&mut self) -> &str {
        match self.input_mode {
            InputMode::Normal if self.part_of_speech_list.items.len() == 1 => {
                "l, h: Change definition; /: Insert"
            }
            InputMode::Normal if !self.results.is_empty() => {
                "j, k: Change part of speech; /: Insert"
            }
            InputMode::Editing => "<ENTER>: Search",
            InputMode::SelectPartOfSpeech => "<ENTER>: Select",
            InputMode::SelectDefinition => "l, h: Change definition; /: Insert",
            _ => "/: Insert",
        }
    }

    /// Update the stateful lists.
    pub fn update_stateful_lists(&mut self, list_type: StatefulListType) {
        match list_type {
            StatefulListType::PartOfSpeech => {
                self.update_part_of_speech_list();
            }
            StatefulListType::Definition => {
                self.update_definition_list();
            }
            _ => {
                self.update_part_of_speech_list();
                self.update_definition_list();
            }
        }
    }

    /// Update the part of speech list.
    fn update_part_of_speech_list(&mut self) {
        if !self.results.is_empty() {
            let meanings = self.results[0].meanings.clone();
            if meanings.is_some() {
                let part_of_speech_list: Vec<String> = meanings
                    .unwrap()
                    .iter()
                    .map(|i| i.partOfSpeech.clone().unwrap_or(String::from("")))
                    .collect();
                self.part_of_speech_list =
                    StatefulList::with_items(part_of_speech_list, StatefulListType::PartOfSpeech);

                // Select the first item as default.
                self.part_of_speech_list.state.select(Some(0))
            }
        }
    }

    /// Update the definition list.
    fn update_definition_list(&mut self) {
        if !self.results.is_empty() {
            if let Some(idx) = self.part_of_speech_list.state.selected() {
                let definitions = Thesaurus::unwrap_meanings_at(idx, &self.results[0]).1;
                let definitions: Vec<String> = definitions
                    .iter()
                    .map(|i| i.definition.clone().unwrap_or(String::from("")))
                    .collect();
                self.definition_list =
                    StatefulList::with_items(definitions, StatefulListType::Definition);

                // Select the first item as default.
                self.definition_list.state.select(Some(0))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::data::{Definition, Meaning};

    use super::*;
    use pretty_assertions::assert_eq;

    fn mock_app_in(input_mode: InputMode) -> App {
        let mut mock_app = App::new();
        mock_app.input_mode = input_mode;
        return mock_app;
    }

    fn mock_part_of_speech() -> String {
        String::from("noun")
    }

    fn mock_meaning_with(p: Option<String>, d: Option<Vec<Definition>>) -> Meaning {
        Meaning {
            partOfSpeech: p,
            definitions: d,
        }
    }

    fn mock_definition_with(d: Option<String>) -> Definition {
        Definition {
            definition: d,
            example: None,
            synonyms: None,
            antonyms: None,
        }
    }

    fn mock_results_with(m: Vec<Meaning>) -> Vec<Thesaurus> {
        vec![Thesaurus {
            word: Some(String::from("mock")),
            origin: None,
            meanings: Some(m),
        }]
    }

    #[test]
    fn test_update_part_of_speech_list() {
        let mut mock_app = mock_app_in(InputMode::default());
        let mock_parts_of_speech = vec![
            String::from("noun"),
            String::from("verb"),
            String::from("adjective"),
        ];
        let mock_meanings = mock_parts_of_speech
            .clone()
            .iter()
            .map(|i| mock_meaning_with(Some(i.to_string()), None))
            .collect();
        mock_app.results = mock_results_with(mock_meanings);
        App::update_stateful_lists(&mut mock_app, StatefulListType::PartOfSpeech);
        assert_eq!(
            mock_parts_of_speech.len(),
            mock_app.part_of_speech_list.items.len()
        );
        assert_eq!(Some(0), mock_app.part_of_speech_list.state.selected())
    }

    #[test]
    fn test_update_definition_list() {
        let mut mock_app = mock_app_in(InputMode::default());
        let mock_definitions = vec![
            mock_definition_with(Some(String::from("Definition 1"))),
            mock_definition_with(Some(String::from("Definition 2"))),
            mock_definition_with(Some(String::from("Definition 3"))),
        ];
        let mock_meanings = vec![mock_meaning_with(
            Some(mock_part_of_speech()),
            Some(mock_definitions.clone()),
        )];
        mock_app.results = mock_results_with(mock_meanings);
        App::update_stateful_lists(&mut mock_app, StatefulListType::All);
        assert_eq!(mock_definitions.len(), mock_app.definition_list.items.len());
        assert_eq!(Some(0), mock_app.definition_list.state.selected());
    }

    #[test]
    fn test_instructions_in_normal_mode() {
        let mut mock_app = mock_app_in(InputMode::Normal);
        assert_eq!(App::update_instructions(&mut mock_app), "/: Insert");
    }

    #[test]
    fn test_instructions_for_word_with_single_part_of_speech() {
        let mut mock_app = mock_app_in(InputMode::default());
        mock_app.results =
            mock_results_with(vec![mock_meaning_with(Some(mock_part_of_speech()), None)]);
        App::update_part_of_speech_list(&mut mock_app);
        assert_eq!(
            App::update_instructions(&mut mock_app),
            "l, h: Change definition; /: Insert"
        );
    }

    #[test]
    fn test_instructions_in_normal_mode_with_results() {
        let mut mock_app = mock_app_in(InputMode::Normal);
        mock_app.results =
            mock_results_with(vec![mock_meaning_with(Some(mock_part_of_speech()), None)]);
        assert_eq!(true, !mock_app.results.is_empty());
        assert_eq!(
            App::update_instructions(&mut mock_app),
            "j, k: Change part of speech; /: Insert"
        );
    }

    #[test]
    fn test_instructions_in_editing_mode() {
        let mut mock_app = mock_app_in(InputMode::Editing);
        assert_eq!(App::update_instructions(&mut mock_app), "<ENTER>: Search");
    }

    #[test]
    fn test_instructions_in_part_of_speech_selection_mode() {
        let mut mock_app = mock_app_in(InputMode::SelectPartOfSpeech);
        assert_eq!(App::update_instructions(&mut mock_app), "<ENTER>: Select");
    }

    #[test]
    fn test_instructions_in_definition_selection_mode() {
        let mut mock_app = mock_app_in(InputMode::SelectDefinition);
        assert_eq!(
            App::update_instructions(&mut mock_app),
            "l, h: Change definition; /: Insert"
        );
    }
}
