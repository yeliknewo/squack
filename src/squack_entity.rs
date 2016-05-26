use actule::*;
use utils::*;

pub struct SquackEntity {
    id: Id,
    renderable: Option<Box<Renderable>>,
    transform: Option<Box<Transform>>,
    name: Option<Box<Name>>,
}

impl SquackEntity {
    pub fn new(id: Id) -> SquackEntity {
        SquackEntity {
            id: id,
            renderable: None,
            transform: None,
            name: None,
        }
    }

    pub fn set_renderable(&mut self, renderable: Renderable) {
        self.renderable = Some(Box::new(renderable));
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = Some(Box::new(transform));
    }

    pub fn get_name(&self) -> Option<&Box<Name>> {
        self.name.as_ref()
    }

    pub fn take_transform(&mut self) -> Option<Box<Transform>> {
        self.transform.take()
    }

    pub fn give_transform(&mut self, transform: Box<Transform>) {
        self.transform = Some(transform);
    }
}

impl Entity<Id, SquackEntity> for SquackEntity {
    fn get_id(&self) -> Id {
        self.id
    }

    fn get_renderable(&self) -> Option<&Box<Renderable>> {
        self.renderable.as_ref()
    }

    fn get_transform(&self) -> Option<&Box<Transform>> {
        self.transform.as_ref()
    }

    fn get_mut_renderable(&mut self) -> Option<&mut Box<Renderable>> {
        self.renderable.as_mut()
    }

    fn get_mut_transform(&mut self) -> Option<&mut Box<Transform>> {
        self.transform.as_mut()
    }

    fn tick(&mut self, dt: f64, manager: &mut Node<Id>, world: &mut SWorld) {
        if self.transform.is_some() && self.renderable.is_some() {
            let mut transform = self.take_transform().expect("Transform was none after being some?");
            transform.tick(&mut self.renderable.as_mut().expect("Renderable was none after being some?"));
            self.give_transform(transform);
        }
    }

    fn is_tick(&self) -> bool {
        self.transform.is_some() && self.renderable.is_some()
    }
}
