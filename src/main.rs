pub mod renderer;
pub mod world;

use world::object::Object;

fn main() {
    let mut world = world::world::create_world();
    let cube_object: (Vec<[f64; 3]>, Vec<[f32; 2]>, Vec<[i8; 3]>, Vec<[f32; 3]>) = world::objects::cube::create_cube((1.0, 1.0, 1.0), (0.0, 0.0, 0.0));
    world.add_object(Object::create(
        renderer::vertex::create_vertices(cube_object.0, cube_object.2, cube_object.3, cube_object.1)
    ));
    renderer::setup::start_engine(world);
}
