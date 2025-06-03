use crate::world::{object::Object};

pub struct World {
    objects: Vec<Object>
}
impl World {
    pub fn new() -> Self{
        Self {
            objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }
}

pub fn create_world() -> World {
    let world: World = World::new();
    world
}