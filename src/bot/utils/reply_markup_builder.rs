use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[derive(Debug, Clone)]
pub struct ReplyMarkupBuilder {
    rows: Vec<Vec<InlineKeyboardButton>>,
}

impl ReplyMarkupBuilder {
    pub fn new() -> Self {
        Self { rows: Vec::new() }
    }

    pub fn row(mut self, buttons: Vec<InlineKeyboardButton>) -> Self {
        self.rows.push(buttons);
        self
    }

    pub fn button_row(self, buttons: Vec<(&str, String)>) -> Self {
        let button_row = buttons
            .into_iter()
            .map(|(text, callback_data)| InlineKeyboardButton::callback(text, callback_data))
            .collect();
        self.row(button_row)
    }

    pub fn single_button(self, text: &str, callback_data: String) -> Self {
        self.button_row(vec![(text, callback_data)])
    }

    pub fn build(self) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::new(self.rows)
    }
}
