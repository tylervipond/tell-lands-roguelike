mod map_action;
mod menu_action;
mod static_action;
mod targeting_action;

pub use map_action::{map_input_to_map_action, MapAction};
pub use menu_action::{map_input_to_horizontal_menu_action, map_input_to_menu_action, MenuAction};
pub use static_action::{map_input_to_static_action, StaticAction};
pub use targeting_action::{
    map_input_to_interaction_targeting_action, map_input_to_targeting_action,
    InteractionTargetingAction, TargetingAction,
};
