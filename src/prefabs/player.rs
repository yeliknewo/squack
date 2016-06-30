use actule::actule::*;
use actule::nalgebra::{Vector1, Vector2};
use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2, Cuboid2};

use utils::redefines::*;
use utils::names::*;
use utils::collision_groups::*;
use components::*;

use find_folder;
use std::path::PathBuf;

pub fn new_player(manager: &mut SNode, world: &mut SWorld, position: Vector2<Coord>, factory: &mut Factory) -> SEntity {
    let width = 50.0;
    let height = 50.0;
    let mass = 0.1;

    let id = manager.alloc().expect("Manager ran out of ids");

    let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").expect("assets folder not found");

    let entity = SEntity::new(id, 0)
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
                            width / 2.0,
                            height / 2.0
                        )
                    )
                ),
                new_collision_group(CollisionLayers::Player)
            )
        )
        .with_physics_obj(
            PhysicsObj::new(
                mass,
                Vector2::new(0.0, 0.0),
                1.0
            )
        )
        .with_renderable(
            Renderable::new(
                0
                /*
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(width, 0.0),
                    Vector2::new(width, height),
                    Vector2::new(0.0, height)
                ),
                [1.0, 1.0, 0.0, 1.0]
                */
            ).with_image(MyImage::new(factory, assets.join("knight1.png").as_path(), [0.0, 0.0, width, height])).with_shape(Shape::new(
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(width, 0.0),
                    Vector2::new(width, height),
                    Vector2::new(0.0, height)
                ),
                [1.0, 0.0, 0.0, 0.2]
            ))
        )
        .with_player(
            Player::new()
        );

    world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Watcher had no hit watcher").add_entity(&entity);

    entity
}

pub fn add_player(manager: &mut SNode, world: &mut SWorld, factory: &mut Factory) {
    let entity = new_player(manager, world, Vector2::new(0.0, 0.0), factory);
    world.add_entity(entity);
}

pub fn add_player_at_position(manager: &mut SNode, world: &mut SWorld, position: Vector2<Coord>, factory: &mut Factory) {
    let entity = new_player(manager, world, position, factory);
    world.add_entity(entity);
}
