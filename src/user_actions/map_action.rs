use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Eq, PartialEq, Serialize, Deserialize, Copy, Clone, Debug)]
pub enum MapAction {
    #[cfg(debug_assertions)]
    ShowDebugMenu,
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    MoveUpLeft,
    MoveUpRight,
    MoveDownLeft,
    MoveDownRight,
    SearchContainer,
    StayStill,
    OpenDoor,
    PickupItem,
    ShowInventoryMenu,
    ShowDropMenu,
    ShowActionMenu,
    ShowEquipmentMenu,
    SearchHidden,
    DisarmTrap,
    ArmTrap,
    GrabFurniture,
    ReleaseFurniture,
    Attack,
    Hide,
    GoDownStairs,
    GoUpStairs,
    Exit,
    LeaveDungeon,
    Interact,
    ScrollLeft,
    ScrollRight,
    ScrollDown,
    ScrollUp,
}

impl MapAction {
    pub fn actions() -> Box<[Self]> {
        Box::new([
            Self::MoveLeft,
            Self::MoveRight,
            Self::MoveUp,
            Self::MoveDown,
            Self::MoveUpLeft,
            Self::MoveUpRight,
            Self::MoveDownLeft,
            Self::MoveDownRight,
            Self::SearchContainer,
            Self::StayStill,
            Self::OpenDoor,
            Self::PickupItem,
            Self::ShowInventoryMenu,
            Self::ShowDropMenu,
            Self::ShowActionMenu,
            Self::ShowEquipmentMenu,
            Self::SearchHidden,
            Self::DisarmTrap,
            Self::ArmTrap,
            Self::GrabFurniture,
            Self::ReleaseFurniture,
            Self::Attack,
            Self::Hide,
            Self::GoDownStairs,
            Self::GoUpStairs,
            Self::Exit,
            Self::LeaveDungeon,
            Self::Interact,
            Self::ScrollLeft,
            Self::ScrollRight,
            Self::ScrollDown,
            Self::ScrollUp,
        ])
    }
}

impl Display for MapAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            String::from(match self {
                #[cfg(debug_assertions)]
                MapAction::ShowDebugMenu => "Show Debug Menu",
                MapAction::MoveLeft => "Move Left",
                MapAction::MoveRight => "Move Right",
                MapAction::MoveUp => "Move Up",
                MapAction::MoveDown => "Move Down",
                MapAction::MoveUpLeft => "Move Up Left",
                MapAction::MoveUpRight => "Move Up Right",
                MapAction::MoveDownLeft => "Move Down Left",
                MapAction::MoveDownRight => "Move Down Right",
                MapAction::SearchContainer => "Search Container",
                MapAction::StayStill => "Stay Still",
                MapAction::OpenDoor => "Open Door",
                MapAction::PickupItem => "Pickup Item",
                MapAction::ShowInventoryMenu => "Show Inventory Menu",
                MapAction::ShowDropMenu => "Show Drop Menu",
                MapAction::ShowActionMenu => "Show Action Menu",
                MapAction::ShowEquipmentMenu => "Show Equipment Menu",
                MapAction::SearchHidden => "Search Area",
                MapAction::DisarmTrap => "Disarm Trap",
                MapAction::ArmTrap => "Arm Trap",
                MapAction::GrabFurniture => "Grab Furniture",
                MapAction::ReleaseFurniture => "Release Furniture",
                MapAction::Attack => "Attack",
                MapAction::Hide => "Hide",
                MapAction::GoDownStairs => "Go Downstairs",
                MapAction::GoUpStairs => "Go Upstairs",
                MapAction::Exit => "Exit",
                MapAction::LeaveDungeon => "Leave Dungeon",
                MapAction::Interact => "Interact",
                MapAction::ScrollLeft => "Scroll Left",
                MapAction::ScrollRight => "Scroll Right",
                MapAction::ScrollDown => "Scroll Down",
                MapAction::ScrollUp => "Scroll Up",
            })
        )
    }
}
