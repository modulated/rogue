use super::{
    EntityMoved, EntryTrigger, GameLog, Hidden, InflictsDamage, Map, Name, Position,
    SingleActivation, SufferDamage,
};
use specs::prelude::*;

pub struct TriggerSystem {}

impl<'a> System<'a> for TriggerSystem {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, EntityMoved>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, EntryTrigger>,
        WriteStorage<'a, Hidden>,
        ReadStorage<'a, Name>,
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, SufferDamage>,
        ReadStorage<'a, SingleActivation>,
        ReadStorage<'a, InflictsDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            mut entity_moved,
            position,
            entry_trigger,
            mut hidden,
            names,
            entities,
            mut log,
            mut suffer_damage,
            single_activation,
            inflicts_damage,
        ) = data;
        let mut remove_entities = Vec::new();

        for (entity, mut _entity_moved, pos) in (&entities, &mut entity_moved, &position).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            for entity_id in map.tile_content[idx].iter() {
                if entity != *entity_id {
                    let maybe_trigger = entry_trigger.get(*entity_id);
                    match maybe_trigger {
                        None => {}
                        Some(_trigger) => {
                            let name_trigger = names.get(*entity_id);
                            let name = names.get(entity);
                            if let Some(name) = name {
                                log.entries.push(format!(
                                    "{} triggers the {}!",
                                    &name.name,
                                    &name_trigger.unwrap().name
                                ));
                            }
                            hidden.remove(*entity_id);

                            let damage = inflicts_damage.get(*entity_id);
                            if let Some(damage) = damage {
                                SufferDamage::new_damage(&mut suffer_damage, entity, damage.damage)
                            }

                            let sa = single_activation.get(*entity_id);
                            if let Some(_sa) = sa {
                                remove_entities.push(*entity_id);
                            }
                        }
                    }
                }
            }
        }
        for trap in remove_entities.iter() {
            entities.delete(*trap).expect("unable to delete trap")
        }
        entity_moved.clear();
    }
}
