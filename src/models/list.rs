use ratatui::widgets::ListState;

#[derive(Clone, Debug)]
pub enum StatefulListType {
    PartOfSpeech,
    Definition,
    Synonym,
    Antonym,
    All,
}

impl Default for StatefulListType {
    fn default() -> Self {
        StatefulListType::All
    }
}

#[derive(Clone, Debug, Default)]
pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    pub list_type: StatefulListType,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>, list_type: StatefulListType) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            list_type: list_type,
        }
    }

    pub fn down(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn up(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
