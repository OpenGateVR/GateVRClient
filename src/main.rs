pub mod renderer;
pub mod world;

use world::object::Object;

use crate::world::{object::ObjectType, objects::fbx_parser::parse};

fn main() {
    let mut world = world::world::create_world();

    let table = parse("models/table.fbx", (0.0, 2.0, 0.0), (1.0, 1.0, 1.0));
    let mut table_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(table.0, table.2, table.3, table.1)
    );
    table_object.set_texture("textures/table.png");
    world.add_object(table_object);

    world.add_object(Object::create(
        ObjectType::Camera,
        Vec::new()
    ));

    let ground = parse("models/plane.fbx", (0.0, 2.0, 0.0), (5.0, 2.0, 5.0));
    let mut ground_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(ground.0, ground.2, ground.3, ground.1)
    );
    ground_object.set_displacement("textures/ground_displacement.png");
    world.add_object(ground_object);

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
