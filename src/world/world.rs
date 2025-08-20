use crate::world::object::{Object, ObjectType};

pub struct World {
    objects: Vec<Object>,
    cameras: Vec<Object>
}
impl World {
    pub fn new() -> Self{
        Self {
            objects: Vec::new(),
            cameras: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: Object) {
        if object.get_object_type() == ObjectType::Camera {
            self.cameras.push(object);
        } else {
            self.objects.push(object);
        }
    }

    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }
}

pub fn create_world() -> World {
    let world: World = World::new();
    world
}