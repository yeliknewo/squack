use actule::actule::*;
use actule::nalgebra::{Vector1, Vector2};
use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2, Cuboid2};

use utils::redefines::*;
use utils::names::*;
use utils::collision_groups::*;
use components::*;

//use actule::gfx_device_gl::Factory;

//note: you can get a Factory by using the window.factory field

pub fn new_ground_at(manager: &mut SNode, world: &mut SWorld, position: Vector2<Coord>, factory: &mut Factory) -> SEntity {
    let id = manager.alloc().expect("Manager ran out of ids");

    let dirt_img = include_bytes!("../assets/Dirt.png");

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
                            25.0
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
                /*
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(100.0, 0.0),
                    Vector2::new(100.0, 50.0),
                    Vector2::new(0.0, 50.0)
                ),
                [1.0, 0.0, 0.0, 1.0]
                */
                //MyImage::new() throws -> thread '<main>' panicked at 'assertion failed: index < self.len()' if width, height are too high
            ).with_image(MyImage::new(factory, dirt_img, 5, 5, [position.x as u32, position.y as u32, 100, 50])).with_shape(Shape::new(
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(100.0, 0.0),
                    Vector2::new(100.0, 50.0),
                    Vector2::new(0.0, 50.0)
                ),
                [0.0, 0.0, 0.0, 1.0]
            ))
        );

    world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Watcher had no hit watcher").add_entity(&entity);

    entity
}

#[inline]
pub fn new_ground(manager: &mut SNode, world: &mut SWorld, factory: &mut Factory) -> SEntity {
    new_ground_at(manager, world, Vector2::new(0.0, 0.0), factory)
}

#[inline]
pub fn add_ground_at(manager: &mut SNode, world: &mut SWorld, position: Vector2<Coord>, factory: &mut Factory) {
    let entity = new_ground_at(manager, world, position, factory);
    world.add_entity(entity);
}

#[inline]
pub fn add_ground(manager: &mut SNode, world: &mut SWorld, factory: &mut Factory) {
    let entity = new_ground(manager, world, factory);
    world.add_entity(entity);
}
