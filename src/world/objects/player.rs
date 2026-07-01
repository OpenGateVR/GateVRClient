use crate::renderer::transform::Transform;

pub struct Player {
    transform: Transform,
    camera: Transform
}
impl Player {
    pub fn new() -> Self {
        Self {
            transform: Transform::base(),
            camera: Transform::base()
        }
    }
}