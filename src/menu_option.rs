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
