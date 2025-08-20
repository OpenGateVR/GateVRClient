use crate::renderer::vertex::Vertex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Cube,
    Camera,
    Sphere,
    Mesh,
}

// this is a game object, and will be used to render the vertices
#[derive(Clone)]
pub struct Object {
    object_type: ObjectType,
    position: (f64, f64, f64),
    size: (f32, f32, f32),
    vertices: Vec<Vertex>
}
impl Object {
    pub fn create(object_type: ObjectType, vertices: Vec<Vertex>) -> Self {
        Self {
            object_type: object_type,
            position: (0.0, 0.0, 0.0),
            size: (0.0, 0.0, 0.0),
            vertices: vertices
        }
    }

    pub fn set_position(&mut self, position: (f64, f64, f64)) {
        self.position = position;
    }
    pub fn set_size(&mut self, size: (f32, f32, f32)) {
        self.size = size;
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    pub fn get_object_type(&self) -> ObjectType {
        self.object_type
    }
}