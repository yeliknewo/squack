#![allow(dead_code)]

#[macro_use]
extern crate actule;

use actule::piston_window::*;
use actule::id_alloc::*;
use actule::actule::*;
use actule::nalgebra::{Vector1, Vector2};
use actule::ncollide::ncollide_geometry::shape::{ShapeHandle2, Cuboid};

mod utils;
mod squack_entity;
mod prefabs;
mod components;

use utils::*;
use components::*;
use prefabs::*;

fn main() {
    let mut window: PistonWindow = {
        let title = "Squack";
        let resolution = [640, 640];
        let opengl = OpenGL::V3_2;
        let mut window_result = WindowSettings::new(title, resolution)
            .exit_on_esc(true)
            .srgb(true)
            .opengl(opengl)
            .build();
        if window_result.is_err() {
            window_result = WindowSettings::new(title, resolution)
                .exit_on_esc(true)
                .srgb(false)
                .opengl(opengl)
                .build();
        }
        PistonWindow::new(opengl, 0, window_result
            .unwrap_or_else(|e| {
                panic!("Failed to build PistonWindow: {}", e)
            }))
    };

    window.set_ups(60);

    let mut game = Game::<Id, SEntity>::new([0.0, 0.0, 0.0, 1.0]);

    let mut manager = Node::new_all();
    {
        let mut world = game.get_mut_world();

        add_watcher(&mut manager, &mut world);

        {
            let id = manager.alloc().expect("Manager ran out of ids");

            let entity = SEntity::new(id, 1)
            .with_renderable(Renderable::new(
                0,
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(100.0, 0.0),
                    Vector2::new(100.0, 100.0),
                    Vector2::new(0.0, 100.0)
                ),
                [1.0, 0.0, 0.0, 1.0]
            ))
            .with_transform(Transform::new(
                Vector2::new(0.0, 0.0),
                Vector1::new(0.0),
                Vector2::new(1.0, 1.0)
            ))
            .with_hitbox(Hitbox::new(
                ShapeHandle2::new(Cuboid::new(Vector2::new(50.0, 50.0)))
            ))
            .with_physics_obj(PhysicsObj::new(
                1.0,
                Vector2::new(10.0, 0.0),
                1.0
            ));

            world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Watcher had no hit watcher").add_entity(&entity);

            world.add_entity(entity);
        }

        {
            let id = manager.alloc().expect("Manager ran out of ids");

            let entity = SEntity::new(id, 1)
            .with_renderable(Renderable::new(
                0,
                vec!(
                    Vector2::new(0.0, 0.0),
                    Vector2::new(100.0, 0.0),
                    Vector2::new(100.0, 100.0),
                    Vector2::new(0.0, 100.0)
                ),
                [1.0, 0.0, 0.0, 1.0]
            ))
            .with_transform(Transform::new(
                Vector2::new(200.0, 0.0),
                Vector1::new(0.0),
                Vector2::new(1.0, 1.0)
            ))
            .with_hitbox(Hitbox::new(
                ShapeHandle2::new(Cuboid::new(Vector2::new(50.0, 50.0)))
            ))
            .with_physics_obj(PhysicsObj::new(
                1.0,
                Vector2::new(-10.0, 0.0),
                1.0
            ));

            world.get_mut_entity_by_name(WATCHER_NAME).expect("Watcher was none").get_mut_hit_watcher().expect("Watcher had no hit watcher").add_entity(&entity);

            world.add_entity(entity);
        }
    }

    game.run(&mut manager, &mut window);
}
