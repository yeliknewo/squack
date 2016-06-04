use std::marker::PhantomData;

use actule::ncollide::ncollide_pipeline::world::{CollisionGroups, CollisionWorld2, GeometricQueryType};
use actule::nalgebra::{Isometry2};

use actule::actule::*;

use utils::redefines::*;

pub struct HitWatcher {
    collision_world: CollisionWorld2<Coord, PhantomData<Id>>,
    query_type: GeometricQueryType<Coord>,
}

impl HitWatcher {
    pub fn new() -> HitWatcher {
        let mut c_groups = CollisionGroups::new();
        c_groups.enable_self_interaction();
        HitWatcher {
            collision_world: CollisionWorld2::new(0.02, true),
            query_type: GeometricQueryType::Contacts(0.0),
        }
    }

    pub fn add_entity(&mut self, entity: &SEntity) {
        let hitbox = entity.get_hitbox().expect("Entity had no hitbox");
        self.collision_world.add(
            entity.get_id(),
            entity.get_transform().expect("Entity had no transform").get_isometry().clone(),
            hitbox.get_shape(),
            hitbox.get_collision_group(),
            self.query_type,
            PhantomData::default()
        );
    }

    pub fn update_hitbox(&mut self, id: Id, isometry: Isometry2<Coord>) {
        if self.collision_world.collision_object(id).is_some() {
            self.collision_world.deferred_set_position(id, isometry);
        } else {
            println!("Id was none: {}", id);
        }
    }

    pub fn tick(&mut self, world: &mut SWorld) {
        self.collision_world.update();
        for contact in self.collision_world.contacts() {
            let mut entity = world.take_entity_by_id(contact.0.uid).expect("Collison object was not an entity");
            entity.get_mut_physics_obj().expect("Entity had no physics object").collision(
                world.get_mut_entity_by_id(contact.1.uid).expect("Collision object was not an entity").get_mut_physics_obj().expect("Entity had no physics object"), contact.2
            );
            world.give_entity(entity);
            //println!("P1: ({},{}), P2: ({},{})", contact.0.position.translation.x, contact.0.position.translation.y, contact.1.position.translation.x, contact.1.position.translation.y);
        }
    }
}
