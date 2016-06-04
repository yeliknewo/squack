use actule::actule::*;
use actule::id_alloc::*;

use core::squack_entity::SquackEntity;

pub type Id = usize;
pub type SEntity = SquackEntity;
pub type SWorld = World<Id, SEntity>;
pub type SNode = Node<Id>;
pub type Mass = f64;
