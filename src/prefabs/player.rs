use actule::actule::*;
use actule::nalgebra::{Vector1, Vector2};
use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2, Cuboid2};

use utils::redefines::*;
use utils::names::*;
use utils::collision_groups::*;
use components::*;


pub fn new_player(manager: &mut SNode, world: &mut SWorld, position: Vector2<Coord>) -> SEntity {
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
                    Cuboid2::new(
                        Vector2::new(
                            50.0,
                            1.0
                        )
                    )
                ),
                new_collision_group(CollisionLayers::Ground)
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
        )
        .with_player(
            Player::new()
        );

    world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Watcher had no hit watcher").add_entity(&entity);

    entity
}
