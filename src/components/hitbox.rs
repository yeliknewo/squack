use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2};
use actule::actule::*;

use utils::*;

pub struct Hitbox {
    shape_handle: ShapeHandle2<Coord>,
    collision_layer: CollisionLayer,
}

impl Hitbox {
    pub fn new(shape_handle: ShapeHandle2<Coord>, collision_layer: CollisionLayer) -> Hitbox {
        Hitbox {
            shape_handle: shape_handle,
            collision_layer: collision_layer,
        }
    }

    pub fn get_shape(&self) -> ShapeHandle2<Coord> {
        self.shape_handle.clone()
    }

    pub fn get_collision_group(&self) -> CollisionLayer {
        self.collision_layer
    }

    pub fn tick(&self, id: Id, world: &mut SWorld, transform: &Transform) {
        world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Entity had no hit watcher").update_hitbox(id, transform.get_isometry().clone());
    }
}
