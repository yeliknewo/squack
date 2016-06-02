use actule::actule::*;
use actule::nalgebra::{Vector1, Vector2};
use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2, Cuboid};

use utils::*;
use components::*;

pub fn new_ground_at(manager: &mut SNode, world: &mut SWorld, position: Vector2<Coord>) -> SEntity {
    let id = manager.alloc().expect("Manager ran out of ids");

    let entity = SEntity::new(id, 2)
        .with_transform(
            Transform::new(
                position,
                Vector1::new(0.0),
                Vector2::new(1.0, 1.0)
            )
        )
        .with_hitbox(
            Hitbox::new(
                ShapeHandle2::new(
                    Cuboid::new(
                        Vector2::new(
                            100.0,
                            50.0
                        )
                    )
                ),
                CollisionLayer::Ground
            )
        )
        .with_physics_obj(
            PhysicsObj::new(
                0.0,
                Vector2::new(0.0, 0.0),
                1.0
            )
        )
        .with_renderable(
            Renderable::new(
                0,
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(100.0, 0.0),
                    Vector2::new(100.0, 50.0),
                    Vector2::new(0.0, 50.0)
                ),
                [1.0, 0.0, 0.0, 1.0]
            )
        );

    world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Watcher had no hit watcher").add_entity(&entity);

    entity
}

#[inline]
pub fn new_ground(manager: &mut SNode, world: &mut SWorld) -> SEntity {
    new_ground_at(manager, world, Vector2::new(0.0, 0.0))
}

#[inline]
pub fn add_ground_at(manager: &mut SNode, world: &mut SWorld, position: Vector2<Coord>) {
    let entity = new_ground_at(manager, world, position);
    world.add_entity(entity);
}

#[inline]
pub fn add_ground(manager: &mut SNode, world: &mut SWorld) {
    let entity = new_ground(manager, world);
    world.add_entity(entity);
}
