#[derive(Clone)]
pub struct Material {
    pub texture: String,
    pub displacement: String
}
impl Material {
    pub fn from_texture(texture: &str) -> Self {
        Self {
            texture: texture.to_string(),
            displacement: "".to_string()
        }
    }

    pub fn get_texture(&self) -> &str {
        &self.texture
    }
    pub fn get_displacement(&self) -> Option<&str> {
        if self.displacement != "".to_string() {
            Some(&self.displacement)
        } else {
            None
        }
    }
}