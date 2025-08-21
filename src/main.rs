pub mod renderer;
pub mod world;

use world::object::Object;

use crate::world::{object::ObjectType, objects::{cube, fbx_parser::parse}};

fn main() {
    let mut world = world::world::create_world();

    let skybox = parse("models/skybox.fbx", (0.0, 0.0, 0.0), (150.0, 150.0, 150.0));
    let mut skybox_object = Object::create(
        ObjectType::Skybox,
        renderer::vertex::create_vertices(skybox.0, skybox.2, skybox.3, skybox.1)
    );
    skybox_object.set_texture("textures/skybox.png");
    skybox_object.set_movable(true);
    world.add_object(skybox_object);

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

    let niko = parse("models/niko.fbx", (0.0, 2.0, -2.0), (2.0, 2.0, 2.0));
    let mut niko_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(niko.0, niko.2, niko.3, niko.1)
    );
    niko_object.set_texture("textures/niko.png");
    world.add_object(niko_object);

    let wall_1 = cube::create_cube((0.0, 2.5, 5.0), (5.0, 0.5, 0.5));
    let mut wall_1_object = Object::create(
        ObjectType::Cube,
        renderer::vertex::create_vertices(wall_1.0, wall_1.2, wall_1.3, wall_1.1)
    );
    wall_1_object.set_texture("textures/wall.jpg");
    world.add_object(wall_1_object);

    let wall_2 = cube::create_cube((0.0, 2.5, -5.0), (5.0, 0.5, 0.5));
    let mut wall_2_object = Object::create(
        ObjectType::Cube,
        renderer::vertex::create_vertices(wall_2.0, wall_2.2, wall_2.3, wall_2.1)
    );
    wall_2_object.set_texture("textures/wall.jpg");
    world.add_object(wall_2_object);

    let wall_3 = cube::create_cube((-5.0, 2.5, 0.0), (0.5, 0.5, 5.0));
    let mut wall_3_object = Object::create(
        ObjectType::Cube,
        renderer::vertex::create_vertices(wall_3.0, wall_3.2, wall_3.3, wall_3.1)
    );
    wall_3_object.set_texture("textures/wall.jpg");
    world.add_object(wall_3_object);

    renderer::setup::start_engine(world);
}
