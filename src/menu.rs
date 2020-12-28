use crate::menu_option::{MenuOption, MenuOptionState};

pub struct Menu<'a> {
    pub options: Box<[MenuOption<'a>]>,
    pub options_per_page: usize,
}

impl<'a> Menu<'a> {
    pub fn new(options: Box<[MenuOption<'a>]>, options_per_page: usize) -> Self {
        Self {
            options,
            options_per_page,
        }
    }

    pub fn get_page(&self, page: usize) -> Box<[&MenuOption]> {
        let page = self
            .options
            .iter()
            .skip(self.options_per_page * page)
            .take(self.options_per_page)
            .collect();
        page
    }

    pub fn page_count(&self) -> usize {
        self.options.len() / self.options_per_page
    }

    pub fn page_number_at_index(&self, index: usize) -> usize {
        index / self.options_per_page
    }

    pub fn get_page_at_index(&self, index: usize) -> Box<[&MenuOption]> {
        let page = self.page_number_at_index(index);
        self.get_page(page)
    }

    pub fn get_next_index(&self, current_option_idx: usize) -> usize {
        let next_index = if current_option_idx + 1 < self.options.len() {
            current_option_idx + 1
        } else {
            0
        };
        // this is an infinite loop if all options are disabled
        match self.options.get(next_index).unwrap().state {
            MenuOptionState::Disabled => self.get_next_index(next_index),
            _ => next_index,
        }
    }

    pub fn get_previous_index(&self, current_option_idx: usize) -> usize {
        let previous_index = if current_option_idx > 0 {
            current_option_idx - 1
        } else {
            self.options.len() - 1
        };
        // this is an infinite loop if all options are disabled
        match self.options.get(previous_index).unwrap().state {
            MenuOptionState::Disabled => self.get_previous_index(previous_index),
            _ => previous_index,
        }
    }

    pub fn get_next_page_index(&self, current_idx: usize) -> usize {
        let current_page_idx = self.page_number_at_index(current_idx);
        let next_page_index = (self.page_count() - 1) % current_page_idx - 1;
        next_page_index * self.options_per_page
    }

    pub fn get_previous_page_index(&self, current_idx: usize) -> usize {
        let current_page_idx = self.page_number_at_index(current_idx);
        let previous_page_index = if current_page_idx > 0 {
            current_page_idx - 1
        } else {
            self.page_count() - 1
        };
        previous_page_index * self.options_per_page
    }
}
