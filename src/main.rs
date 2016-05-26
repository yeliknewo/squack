extern crate actule;

use actule::*;

mod utils;
mod squack_entity;

use utils::*;

fn main() {
        let title = "Squack";
        let resolution = [640, 640];
        let mut window: PistonWindow = WindowSettings::new(title, resolution)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| {
                panic!("Failed to build PistonWindow: {}", e)
            });

        window.set_ups(60);

        let mut game = Game::<Id, SEntity>::new();

        let mut manager = Node::new_all();
        {
            let mut world = game.get_mut_world();

            {
                let id = manager.alloc().expect("Manager ran out of ids");
                let entity = {
                    let mut entity = SEntity::new(id);
                    entity.set_renderable(Renderable::new(0, vec!([0.0, 0.0], [100.0, 100.0], [50.0, 100.0]), [1.0, 0.0, 0.0, 1.0]));
                    entity.set_transform(Transform::new([100.0, 0.0], 0.0, [1.0, 1.0]));
                    entity.get_mut_transform().unwrap().set_position([0.0, 100.0]);
                    entity
                };
                world.add_entity(entity);
            }

        }

        game.run(&mut manager, &mut window);
}
