use specs::Entity;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
  AwaitingInput,
  PreRun,
  PlayerTurn,
  MonsterTurn,
  ShowTargeting { range: i32, item: Entity },
  InventoryMenu { highlighted: usize },
  DropItemMenu { highlighted: usize },
  MainMenu { highlighted: usize },
  ExitGameMenu { highlighted: usize },
  DeathScreen,
  IntroScreen,
  FailureScreen,
  SuccessScreen,
  CreditsScreen,
}
