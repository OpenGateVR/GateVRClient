use crate::{renderer::vertex::create_vertices_skinned, world::{material::Material, object::{Object, ObjectType}, objects::fbx_parser::parse, scene::load_scene}};

pub struct World {
    pub objects: Vec<Object>,
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

    pub fn get_object(&self, index: usize) -> &Object {
        &self.objects[index]
    }
    pub fn get_objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn get_camera(&self, index: usize) -> &Object {
        &self.cameras[index]
    }
    pub fn get_cameras(&self) -> &Vec<Object> {
        &self.cameras
    }

    pub fn load_world(&mut self, path: &str) {
        let scene = load_scene(path);
        if let Ok(scene) = scene {
            let mut static_world_object = Object::create(
                ObjectType::StaticMesh,
                Vec::new()
            );

            for scene_object in scene.objects {
                let model_parsed = parse(&scene_object.reference,
                    (scene_object.transform.position[0], scene_object.transform.position[1], scene_object.transform.position[2]),
                    (scene_object.transform.scale[0], scene_object.transform.scale[1], scene_object.transform.scale[2]),
                    (scene_object.transform.rotation[0], scene_object.transform.rotation[1], scene_object.transform.rotation[2])
                );

                if scene_object.is_static {
                    let mut vertices = create_vertices_skinned(&model_parsed.0);
                    for mesh in &mut vertices {
                        if mesh.1 == "default" {
                            mesh.1 = format!("material{}", static_world_object.get_materials().len());
                        }
                    }
                    let material_name = format!("material{}", static_world_object.get_materials().len());
                    let mut object_material = Material::from_texture(&scene_object.texture);
                    if scene_object.displace != "".to_string() {
                        object_material.set_displacement(&scene_object.displace);
                    }
                    static_world_object.add_material(object_material, material_name);
                    static_world_object.add_meshes(vertices);
                } else {
                    let object = Object::create(
                        ObjectType::Mesh,
                        create_vertices_skinned(&model_parsed.0)
                    );
                    self.add_object(object);
                }
            }

            self.add_object(static_world_object);
        } else {
            println!("Error while loading scene");
        }
    }
}

pub fn create_world() -> World {
    let world: World = World::new();
    world
}