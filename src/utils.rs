use actule::actule::*;
use actule::id_alloc::*;

use squack_entity::*;

pub type Id = usize;
pub type SEntity = SquackEntity;
pub type SWorld = World<Id, SEntity>;
pub type SNode = Node<Id>;

pub const WATCHER_NAME: &'static str = "Watcher";
