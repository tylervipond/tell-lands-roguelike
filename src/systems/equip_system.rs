use crate::components::{equipable::EquipmentPositions, Equipment, Inventory, Name, WantsToEquip};
use crate::services::GameLog;
use specs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};

fn set_at_position_in_equipment(
    equipment: &mut Equipment,
    equipment_ent: Option<Entity>,
    position: EquipmentPositions,
) {
    match position {
        EquipmentPositions::OffHand => equipment.off_hand = equipment_ent,
        EquipmentPositions::DominantHand => equipment.dominant_hand = equipment_ent,
        _ => {}
    };
}

fn get_at_position_in_equipment(
    equipment: &Equipment,
    position: EquipmentPositions,
) -> Option<Entity> {
    match position {
        EquipmentPositions::OffHand => equipment.off_hand,
        EquipmentPositions::DominantHand => equipment.dominant_hand,
        _ => None,
    }
}

pub struct EquipSystem {}

impl<'a> System<'a> for EquipSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, WantsToEquip>,
        WriteStorage<'a, Equipment>,
        WriteStorage<'a, Inventory>,
        ReadStorage<'a, Name>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut wants_to_equip,
            mut equipment,
            mut inventories,
            names,
            player_entity,
            mut log,
        ) = data;

        for (entity, ent_equipment, intent, inventory) in (
            &entities,
            &mut equipment,
            &mut wants_to_equip,
            &mut inventories,
        )
            .join()
        {
            if let Some(old_equipment_ent) =
                get_at_position_in_equipment(ent_equipment, intent.position)
            {
                inventory.items.insert(old_equipment_ent);
                if entity == *player_entity {
                    let name = &names.get(old_equipment_ent).unwrap().name;
                    log.add(format!("You unequip the {}", name));
                }
            }
            if let Some(equipment_ent) = intent.equipment {
                inventory.items.remove(&equipment_ent);
                if entity == *player_entity {
                    let name = &names.get(equipment_ent).unwrap().name;
                    log.add(format!("You equip the {}", name));
                }
            }
            set_at_position_in_equipment(ent_equipment, intent.equipment, intent.position);
        }
        wants_to_equip.clear();
    }
}
