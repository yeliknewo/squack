use std::marker::PhantomData;

use actule::ncollide::ncollide_pipeline::world::{CollisionGroups, CollisionWorld2, GeometricQueryType};

use actule::actule::*;

use utils::*;

pub struct HitWatcher {
    collision_world: CollisionWorld2<f64, PhantomData<Id>>,
    query_type: GeometricQueryType<Coord>,
    collision_groups: CollisionGroups,
}

impl HitWatcher {
    pub fn new() -> HitWatcher {
        let mut c_groups = CollisionGroups::new();
        c_groups.enable_self_interaction();
        HitWatcher {
            collision_world: CollisionWorld2::new(0.02, true),
            query_type: GeometricQueryType::Contacts(0.0),
            collision_groups: c_groups,
        }
    }

    pub fn add_entity(&mut self, entity: &SEntity) {
        let hitbox = entity.get_hitbox().expect("Entity had no hitbox");
        self.collision_world.add(
            entity.get_id(),
            entity.get_transform().expect("Entity had no transform").get_isometry().clone(),
            hitbox.get_shape(),
            self.collision_groups,
            self.query_type,
            PhantomData::default()
        );
    }

    pub fn tick(&mut self) {
        self.collision_world.update();
        for contact in self.collision_world.contacts() {
            println!("P1: ({},{}), P2: ({},{})", contact.0.position.translation.x, contact.0.position.translation.y, contact.1.position.translation.x, contact.1.position.translation.y);
        }
    }
}
