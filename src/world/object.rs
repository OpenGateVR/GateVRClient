use std::{collections::HashMap, hash::Hash};

use crate::renderer::vertex::Vertex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Cube,
    Camera,
    Sphere,
    Mesh,
    Skybox,
    Grabbable,
    TabletMenu,
    TabletMenuButton
}

#[derive(Clone)]
pub struct Material {
    pub texture: String
}

// this is a game object, and will be used to render the vertices
#[derive(Clone)]
pub struct Object {
    object_type: ObjectType,
    position: (f64, f64, f64),
    size: (f32, f32, f32),
    rotation: (f32, f32, f32),
    vertices: Vec<(Vec<Vertex>, String)>,
    materials: HashMap<String, Material>,
    displacement_texture: String,
    movable: bool,
    tag: String
}
impl Object {
    pub fn create(object_type: ObjectType, meshes: Vec<(Vec<Vertex>, String)>) -> Self {
        let mut materials = HashMap::new();
        materials.insert("default".to_string(), Material { texture: "textures/ground.jpg".to_string() });
        Self {
            object_type: object_type,
            position: (0.0, 0.0, 0.0),
            size: (0.0, 0.0, 0.0),
            rotation: (0.0, 0.0, 0.0),
            vertices: meshes,
            materials,
            displacement_texture: "None".to_string(),
            movable: false,
            tag: "unnamed".to_string()
        }
    }

    pub fn add_material(&mut self, material: Material, name: &str) {
        self.materials.insert(name.to_string(), material);
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
    pub fn set_vertices(&mut self, vertices: Vec<(Vec<Vertex>, String)>) {
        self.vertices = vertices;
    }
    pub fn set_default_texture(&mut self, texture: &str) {
        self.materials.insert("default".to_string(), Material { texture: texture.to_string() });
    }
    pub fn set_displacement(&mut self, texture: &str) {
        self.displacement_texture = texture.to_string()
    }
    pub fn set_movable(&mut self, value: bool) {
        self.movable = value;
    }
    pub fn set_tag(&mut self, tag: &str) {
        self.tag = tag.to_string();
    }

    pub fn get_vertices(&self) -> &Vec<(Vec<Vertex>, String)> {
        &self.vertices
    }
    pub fn get_materials(&self) -> &HashMap<String, Material> {
        &self.materials
    }
    pub fn get_object_type(&self) -> ObjectType {
        self.object_type
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
    pub fn get_tag(&self) -> &str {
        &self.tag
    }
}