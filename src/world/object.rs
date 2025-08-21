use crate::renderer::vertex::Vertex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Cube,
    Camera,
    Sphere,
    Mesh,
    Skybox,
    Grabbable,
    TabletMenu
}

// this is a game object, and will be used to render the vertices
#[derive(Clone)]
pub struct Object {
    object_type: ObjectType,
    position: (f64, f64, f64),
    size: (f32, f32, f32),
    rotation: (f32, f32, f32),
    vertices: Vec<Vertex>,
    texture: String,
    displacement_texture: String,
    movable: bool
}
impl Object {
    pub fn create(object_type: ObjectType, vertices: Vec<Vertex>) -> Self {
        Self {
            object_type: object_type,
            position: (0.0, 0.0, 0.0),
            size: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            vertices: vertices,
            texture: "textures/ground.jpg".to_string(),
            displacement_texture: "None".to_string(),
            movable: false
        }
    }

    pub fn set_position(&mut self, position: (f64, f64, f64)) {
        self.position = position;
    }
    pub fn set_size(&mut self, size: (f32, f32, f32)) {
        self.size = size;
    }
    pub fn set_rotation(&mut self, rotation: (f32, f32, f32)) {
        self.rotation = rotation;
    }
    pub fn set_rotation_y(&mut self, rotation: f32) {
        self.rotation.1 = rotation;
    }
    pub fn set_texture(&mut self, texture: &str) {
        self.texture = texture.to_string()
    }
    pub fn set_displacement(&mut self, texture: &str) {
        self.displacement_texture = texture.to_string()
    }
    pub fn set_movable(&mut self, value: bool) {
        self.movable = value;
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    pub fn get_object_type(&self) -> ObjectType {
        self.object_type
    }
    pub fn get_texture(&self) -> &str {
        &self.texture
    }
    pub fn get_displacement(&self) -> &str {
        &self.displacement_texture
    }
    pub fn get_movable(&self) -> bool {
        self.movable
    }
    pub fn get_position(&self) -> (f64, f64, f64) {
        self.position
    }
    pub fn get_rotation(&self) -> (f32, f32, f32) {
        self.rotation
    }
}