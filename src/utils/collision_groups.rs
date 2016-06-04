use actule::ncollide::ncollide_pipeline::world::{CollisionGroups};

mod collision_ids {
    pub const GROUND: usize = 0;
    pub const PLAYER: usize = 1;
}

pub enum CollisionLayers {
    Ground,
    Player,
}

pub fn new_collision_group(layer: CollisionLayers) -> CollisionGroups {
    match layer {
        CollisionLayers::Ground => new_collision_group_ground(),
        CollisionLayers::Player => new_collision_group_player(),
    }
}

fn new_collision_group_ground() -> CollisionGroups {
    let mut group = CollisionGroups::new();

    group.set_membership(&[collision_ids::GROUND]);

    group.set_blacklist(&[collision_ids::GROUND]);

    group
}

fn new_collision_group_player() -> CollisionGroups {
    let mut group = CollisionGroups::new();

    group.set_membership(&[collision_ids::PLAYER]);

    group.set_whitelist(&[collision_ids::GROUND]);

    group.set_blacklist(&[collision_ids::PLAYER]);

    group
}
