use crate::renderer::vertex::Vertex;

pub struct Object {
    position: (f64, f64, f64),
    size: (f32, f32, f32),
    vertices: Vec<Vertex>
}
impl Object {
    pub fn create(vertices: Vec<Vertex>) -> Self {
        Self {
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
}