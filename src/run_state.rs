use specs::Entity;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowTargeting {
        range: i32,
        item: Entity,
    },
    ShowTargetingOpenContainer,
    ShowTargetingDisarmTrap,
    ShowTargetingGrabFurniture,
    OpenContainerMenu {
        highlighted: usize,
        page: usize,
        container: Entity,
    },
    InventoryMenu {
        highlighted: usize,
        page: usize,
    },
    DropItemMenu {
        highlighted: usize,
        page: usize,
    },
    MainMenu {
        highlighted: usize,
    },
    ExitGameMenu {
        highlighted: usize,
    },
    ActionMenu {
        highlighted: usize,
    },
    #[cfg(debug_assertions)]
    DebugMenu {
        highlighted: usize,
    },
    DeathScreen,
    IntroScreen,
    FailureScreen,
    SuccessScreen,
    CreditsScreen,
}
