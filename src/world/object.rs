use std::collections::HashMap;

use crate::{renderer::{transform::Transform, vertex::Vertex}, world::material::Material};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Cube,
    Camera,
    Sphere,
    Mesh,
    StaticMesh,
    SkinnedMesh,
    Skybox,
    Grabbable,
    TabletMenu,
    TabletMenuButton
}

fn new_bone_vec(amount: usize) -> Vec<Transform> {
    let mut bones = Vec::new();
    for _ in 0..amount {
        bones.push(Transform::zero());
    }
    bones
}

// this is a game object, and will be used to render the vertices
#[derive(Clone)]
pub struct Object {
    object_type: ObjectType,
    transform: Transform,
    vertices: Vec<(Vec<Vertex>, String)>,
    materials: HashMap<String, Material>,
    bones: Vec<Transform>,
    skeleton: HashMap<String, usize>,
    movable: bool,
    tag: String
}
impl Object {
    pub fn create(object_type: ObjectType, meshes: Vec<(Vec<Vertex>, String)>) -> Self {
        let mut materials = HashMap::new();
        materials.insert("default".to_string(), Material { texture: "textures/missing.png".to_string(), displacement: "".to_string() });
        Self {
            object_type: object_type,
            transform: Transform::zero(),
            vertices: meshes,
            materials,
            bones: Vec::new(),
            skeleton: HashMap::new(),
            movable: false,
            tag: "unnamed".to_string()
        }
    }

    pub fn add_material(&mut self, material: Material, name: String) {
        self.materials.insert(name, material);
    }
    pub fn add_meshes(&mut self, meshes: Vec<(Vec<Vertex>, String)>) {
        self.vertices.extend(meshes);
    }

    pub fn set_position(&mut self, position: (f32, f32, f32)) {
        self.transform.position = position;
    }
    pub fn set_size(&mut self, size: (f32, f32, f32)) {
        self.transform.scale = size;
    }
    pub fn set_rotation(&mut self, rotation: (f32, f32, f32)) {
        self.transform.rotation = rotation;
    }
    pub fn set_rotation_y(&mut self, rotation: f32) {
        self.transform.rotation.1 = rotation;
    }
    pub fn set_vertices(&mut self, vertices: Vec<(Vec<Vertex>, String)>) {
        self.vertices = vertices;
    }
    pub fn clear_vertices(&mut self) {
        self.vertices.clear();
    }
    pub fn set_default_texture(&mut self, texture: &str) {
        self.materials.insert("default".to_string(), Material { texture: texture.to_string(), displacement: "".to_string() });
    }
    pub fn set_displacement(&mut self, texture: &str) {
        for material in self.materials.values_mut() {
            material.displacement = texture.to_string();
        }
    }
    pub fn set_bones(&mut self, bones: HashMap<i64, (usize, Transform)>,
        position: (f32, f32, f32), rotation: (f32, f32, f32), scale: (f32, f32, f32)
    ) {
        let mut bones_converted: Vec<Transform> = new_bone_vec(bones.len());
        for bone in bones {
            if bone.1.0 > bones_converted.len() { continue; }
            bones_converted[bone.1.0] = Transform { position: position, rotation: rotation, scale: scale };
        }
        self.bones = bones_converted;
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
    pub fn get_bones(&self) -> &Vec<Transform> {
        &self.bones
    }
    pub fn get_movable(&self) -> bool {
        self.movable
    }
    pub fn get_position(&self) -> (f32, f32, f32) {
        self.transform.position
    }
    pub fn get_rotation(&self) -> (f32, f32, f32) {
        self.transform.rotation
    }
    pub fn get_tag(&self) -> &str {
        &self.tag
    }
}