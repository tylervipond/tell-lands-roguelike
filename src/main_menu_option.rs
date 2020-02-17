pub struct MainMenuOption<'a> {
  pub text: &'a str,
  pub disabled: bool,
}

impl<'a> MainMenuOption<'a> {
  pub fn new(text: &'a str, disabled: bool) -> Self {
    Self {text, disabled}
  }
}