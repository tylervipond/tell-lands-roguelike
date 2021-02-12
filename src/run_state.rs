use crate::{components::equipable::EquipmentPositions, player::InteractionType};
use specs::Entity;

#[derive(PartialEq, Clone)]
pub enum RunState {
    AwaitingInput {
        offset_x: i32,
        offset_y: i32,
    },
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ItemUseTargeting {
        range: u32,
        item: Entity,
    },
    InteractionTypeEntityTargeting {
        target_idx: usize,
        targets: Box<[Entity]>,
        interaction_type: InteractionType,
        cta: Option<&'static str>,
    },
    InteractiveEntityTargeting {
        target_idx: usize,
    },
    InteractMenu {
        highlighted: usize,
        target: Entity,
    },
    OpenContainerMenu {
        highlighted: usize,
        container: Entity,
    },
    InventoryMenu {
        highlighted: usize,
    },
    DropItemMenu {
        highlighted: usize,
    },
    EquipMenu {
        highlighted: usize,
        position: EquipmentPositions,
    },
    EquipmentMenu {
        highlighted: usize,
        action_highlighted: usize,
        action_menu: bool,
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
    LoadingScreen {
        count_down: u32,
    },
    SavingScreen {
        count_down: u32,
    },
    NewGameScreen {
        count_down: u32,
    },
}
