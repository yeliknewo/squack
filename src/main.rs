#![allow(dead_code)]

#[macro_use]
extern crate actule;

use actule::piston_window::*;
use actule::id_alloc::*;
use actule::actule::*;
use actule::nalgebra::{Vector2};

mod utils;
mod prefabs;
mod components;
mod core;

use utils::redefines::*;
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
        let mut i = 0;
        while i <= 800 {
            add_ground_at(&mut manager, &mut world, Vector2::new(i as f64, 400.0));
            i += 100;
        }
        new_player(&mut manager, &mut world, Vector2::new(50.0, 50.0));
    }

    game.run(&mut manager, &mut window);
}
