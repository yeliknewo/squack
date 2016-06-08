mod watcher;
mod ground;
mod player;

pub use self::watcher::*;
pub use self::ground::*;
pub use self::player::*;

/*
Prefab naming conventions

new_PREFAB_NAME returns an entity that is that prefabs

add_PREFAB_NAME adds the entity to the world

add_PREFAB_NAME_with_PROPERTY for non essiential properties
*/
