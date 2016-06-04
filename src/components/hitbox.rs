use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2};
use actule::ncollide::ncollide_pipeline::world::{CollisionGroups};
use actule::actule::*;

use utils::redefines::*;
use utils::names::*;

pub struct Hitbox {
    shape_handle: ShapeHandle2<Coord>,
    collision_group: CollisionGroups,
}

impl Hitbox {
    pub fn new(shape_handle: ShapeHandle2<Coord>, collision_group: CollisionGroups) -> Hitbox {
        Hitbox {
            shape_handle: shape_handle,
            collision_group: collision_group,
        }
    }

    pub fn get_shape(&self) -> ShapeHandle2<Coord> {
        self.shape_handle.clone()
    }

    pub fn get_collision_group(&self) -> CollisionGroups {
        self.collision_group
    }

    pub fn tick(&self, id: Id, world: &mut SWorld, transform: &Transform) {
        world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Entity had no hit watcher").update_hitbox(id, transform.get_isometry().clone());
    }
}
