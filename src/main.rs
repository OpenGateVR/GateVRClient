pub mod renderer;
pub mod world;

use world::object::Object;

use crate::world::{object::ObjectType, objects::fbx_parser::parse};

fn main() {
    let mut world = world::world::create_world();

    let table = parse("models/table.fbx", (0.0, 3.0, 0.0), (1.0, 1.0, 1.0));
    world.add_object(Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(table.0, table.2, table.3, table.1)
    ));

    world.add_object(Object::create(
        ObjectType::Camera,
        Vec::new()
    ));

    /*let cube_object1 = world::objects::cube::create_cube((1.0, 1.0, 4.0), (0.0, 0.0, 0.0));
    world.add_object(Object::create(
        ObjectType::Cube,
        renderer::vertex::create_vertices(cube_object1.0, cube_object1.2, cube_object1.3, cube_object1.1)
    ));

    let cube_object2 = world::objects::cube::create_cube((1.0, 1.0, 1.0), (0.0, 3.0, -2.0));
    world.add_object(Object::create(
        ObjectType::Cube,
        renderer::vertex::create_vertices(cube_object2.0, cube_object2.2, cube_object2.3, cube_object2.1)
    ));

    let cube_object3 = world::objects::cube::create_cube((1.0, 1.0, 1.0), (0.0, 3.0, 2.0));
    world.add_object(Object::create(
        ObjectType::Cube,
        renderer::vertex::create_vertices(cube_object3.0, cube_object3.2, cube_object3.3, cube_object3.1)
    ));*/

    renderer::setup::start_engine(world);
}
