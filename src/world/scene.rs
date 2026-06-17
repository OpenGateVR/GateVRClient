use rust_embed::RustEmbed;
use serde::{ Serialize, Deserialize };

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Assets;

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub id: u32,
    pub creator: String,
    pub objects: Vec<Object>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Object {
    pub is_static: bool,

    pub reference: String,

    #[serde(default = "default_texture")]
    pub texture: String,

    pub transform: Transform,
}

fn default_texture() -> String {
    "textures/missing.png".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    #[serde(default = "default_position")]
    pub position: [f32; 3],

    #[serde(default = "default_rotation")]
    pub rotation: [f32; 3],

    #[serde(default = "default_scale")]
    pub scale: [f32; 3],
}

fn default_position() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_rotation() -> [f32; 3] {
    [0.0, 0.0, 0.0]
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

pub fn load_scene(path: &str) -> Result<Scene, Box<dyn std::error::Error>> {
    let data = Assets::get(path).expect("Failed to get scene").data;
    let scene: Scene = serde_json::from_slice(&data)?;
    Ok(scene)
}