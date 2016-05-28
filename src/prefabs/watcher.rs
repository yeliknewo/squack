use actule::actule::*;

use utils::*;
use components::*;

pub fn new_watcher(manager: &mut SNode, world: &mut SWorld) -> SEntity {
    let id = manager.alloc().expect("Manager ran out of ids");

    let name = Name::new(WATCHER_NAME, id, world);

    SEntity::new(id)
        .with_name(name)
        .with_hit_watcher(HitWatcher::new())
}

pub fn add_watcher(manager: &mut SNode, world: &mut SWorld) {
    let entity = new_watcher(manager, world);
    world.add_entity(entity);
}
