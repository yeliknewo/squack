use actule::actule::*;
use utils::*;
use components::*;

pub struct SquackEntity {
    id: Id,
    renderable: Option<Box<Renderable>>,
    transform: Option<Box<Transform>>,
    name: Option<Box<Name>>,
    hitbox: Option<Box<Hitbox>>,
    hit_watcher: Option<Box<HitWatcher>>,
}

impl_component_for_entity!(SquackEntity, name, Name, set_option_name, set_name, with_name, get_name, get_mut_name, take_name, give_name);
impl_component_for_entity!(SquackEntity, hitbox, Hitbox, set_option_hitbox, set_hitbox, with_hitbox, get_hitbox, get_mut_hitbox, take_hitbox, give_hitbox);
impl_component_for_entity!(SquackEntity, hit_watcher, HitWatcher, set_option_hit_watcher, set_hit_watcher, with_hit_watcher, get_hit_watcher, get_mut_hit_watcher, take_hit_watcher, give_hit_watcher);

impl SquackEntity {
    pub fn new(id: Id) -> SquackEntity {
        SquackEntity {
            id: id,
            renderable: None,
            transform: None,
            name: None,
            hitbox: None,
            hit_watcher: None,
        }
    }

    #[inline]
    pub fn set_option_renderable(&mut self, renderable: Option<Box<Renderable>>) {
        self.renderable = renderable;
    }

    #[inline]
    pub fn set_option_transform(&mut self, transform: Option<Box<Transform>>) {
        self.transform = transform;
    }

    #[inline]
    pub fn set_renderable(&mut self, renderable: Renderable) {
        self.set_option_renderable(Some(Box::new(renderable)));
    }

    #[inline]
    pub fn set_transform(&mut self, transform: Transform) {
        self.set_option_transform(Some(Box::new(transform)));
    }

    #[inline]
    pub fn with_renderable(mut self, renderable: Renderable) -> SquackEntity {
        self.set_renderable(renderable);
        self
    }

    #[inline]
    pub fn with_transform(mut self, transform: Transform) -> SquackEntity {
        self.set_transform(transform);
        self
    }

    #[inline]
    pub fn take_renderable(&mut self) -> Option<Box<Renderable>> {
        self.renderable.take()
    }

    #[inline]
    pub fn take_transform(&mut self) -> Option<Box<Transform>> {
        self.transform.take()
    }

    #[inline]
    pub fn give_renderable(&mut self, renderable: Box<Renderable>) {
        self.set_option_renderable(Some(renderable));
    }

    #[inline]
    pub fn give_transform(&mut self, transform: Box<Transform>) {
        self.set_option_transform(Some(transform));
    }
}

impl Entity<Id, SquackEntity> for SquackEntity {
    #[inline]
    fn get_id(&self) -> Id {
        self.id
    }

    #[inline]
    fn get_renderable(&self) -> Option<&Box<Renderable>> {
        self.renderable.as_ref()
    }

    #[inline]
    fn get_transform(&self) -> Option<&Box<Transform>> {
        self.transform.as_ref()
    }

    #[inline]
    fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>> {
        self.renderable.as_mut()
    }

    #[inline]
    fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>> {
        self.transform.as_mut()
    }

    fn tick(&mut self, dt: f64, manager: &mut SNode, world: &mut SWorld, keyboard: &Keyboard) {
        if self.transform.is_some() && self.renderable.is_some() {
            let mut transform = self.take_transform().expect("Transform was none after being some?");
            transform.tick(&mut self.renderable.as_mut().expect("Renderable was none after being some?"));
            self.give_transform(transform);
        }
        if self.hit_watcher.is_some() {
            self.get_mut_hit_watcher().unwrap().tick();
        }
    }

    #[inline]
    fn is_tick(&self) -> bool {
        (self.transform.is_some() && self.renderable.is_some()) ||
        self.hit_watcher.is_some()
    }
}
