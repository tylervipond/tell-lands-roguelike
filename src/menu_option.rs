#[derive(Clone, Debug, PartialEq)]
pub enum MenuOptionState {
    Disabled,
    Highlighted,
    Normal,
}

#[derive(Clone, Debug)]
pub struct MenuOption<'a> {
    pub text: &'a str,
    pub state: MenuOptionState,
}

impl<'a> MenuOption<'a> {
    pub fn new(text: &'a str, state: MenuOptionState) -> Self {
        Self { text, state }
    }
}

pub fn select_next_menu_index(menu: &Vec<MenuOption>, highlighted: usize) -> usize {
    if menu
        .iter()
        .filter(|o| o.state != MenuOptionState::Disabled)
        .count()
        <= 1
    {
        return 0;
    }
    let next_index = if highlighted + 1 < menu.len() {
        highlighted + 1
    } else {
        0
    };
    match menu.get(next_index).unwrap().state {
        MenuOptionState::Disabled => select_next_menu_index(menu, next_index),
        _ => next_index,
    }
}

pub fn select_previous_menu_index(menu: &Vec<MenuOption>, highlighted: usize) -> usize {
    if menu
        .iter()
        .filter(|o| o.state != MenuOptionState::Disabled)
        .count()
        <= 1
    {
        return 0;
    }
    let previous_index = if highlighted > 0 {
        highlighted - 1
    } else {
        menu.len() - 1
    };
    match menu.get(previous_index).unwrap().state {
        MenuOptionState::Disabled => select_previous_menu_index(menu, previous_index),
        _ => previous_index,
    }
}
