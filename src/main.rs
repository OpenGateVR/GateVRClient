pub mod renderer;
pub mod world;
pub mod interract;
pub mod setup;

use world::object::Object;

use crate::setup::fonts::load_font_uvs;
use crate::world::objects::text;
use crate::world::{object::ObjectType, objects::{cube, fbx_parser::parse}};

fn main() {
    let mut world = world::world::create_world();

    let skybox = parse("models/skybox.fbx", (0.0, 0.0, 0.0), (150.0, 150.0, 150.0), (0.0, 0.0, 0.0));
    let mut skybox_object = Object::create(
        ObjectType::Skybox,
        renderer::vertex::create_vertices(skybox.0, skybox.2, skybox.3, skybox.1)
    );
    skybox_object.set_texture("textures/skybox_2.png");
    skybox_object.set_movable(true);
    world.add_object(skybox_object);

    let mut camera = Object::create(
        ObjectType::Camera,
        Vec::new()
    );
    camera.set_position((-3.0, 4.0, 3.0));
    camera.set_rotation((0.0, -45.0, 0.0));
    world.add_object(camera);

    let table = parse("models/table.fbx", (0.0, 2.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, 0.0));
    let mut table_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(table.0, table.2, table.3, table.1)
    );
    table_object.set_texture("textures/table.png");
    world.add_object(table_object);

    let ground = parse("models/plane.fbx", (0.0, 2.0, 0.0), (5.0, 2.0, 5.0), (0.0, 0.0, 0.0));
    let mut ground_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(ground.0, ground.2, ground.3, ground.1)
    );
    ground_object.set_displacement("textures/ground_displacement.png");
    world.add_object(ground_object);

    let niko = parse("models/niko.fbx", (0.0, 2.0, -2.0), (2.0, 2.0, 2.0), (0.0, 0.0, 0.0));
    let mut niko_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(niko.0, niko.2, niko.3, niko.1)
    );
    niko_object.set_texture("textures/niko.png");
    world.add_object(niko_object);

    let test = parse("models/SELESTIA.fbx", (3.0, 2.0, -2.0), (2.0, 2.0, 2.0), (0.0, 0.0, 0.0));
    let mut test_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(test.0, test.2, test.3, test.1)
    );
    test_object.set_texture("textures/Selestia_costume.png");
    world.add_object(test_object);

    let figure = parse("models/niko.fbx", (0.0, 3.0, 0.0), (0.5, 0.5, 0.5), (0.0, 0.0, 0.0));
    let mut figure_object = Object::create(
        ObjectType::Grabbable,
        renderer::vertex::create_vertices(figure.0, figure.2, figure.3, figure.1)
    );
    figure_object.set_texture("textures/niko.png");
    figure_object.set_position((0.0, 3.0, 0.0));
    world.add_object(figure_object);

    let tablet = cube::create_cube((0.0, 0.0, 0.0), (0.5, 0.4, 0.01));
    let mut tablet_object = Object::create(
        ObjectType::TabletMenu,
        renderer::vertex::create_vertices(tablet.0, tablet.2, tablet.3, tablet.1)
    );
    tablet_object.set_position((0.0, -10.0, 0.0));
    tablet_object.set_texture("textures/wall.jpg");
    world.add_object(tablet_object);

    let font_uvs = load_font_uvs("fonts/NotoSansJP.ttf");
    let sentence = text::create_plane_with_text(
        (-0.4, 0.3, -0.02), (0.03, 0.03, 1.0), 
        font_uvs, "goodbye :C"
    );
    let mut sentence_object = Object::create(
        ObjectType::TabletMenu,
        renderer::vertex::create_vertices(sentence.0, sentence.2, sentence.3, sentence.1)
    );
    sentence_object.set_texture("fonts/NotoSansJP.ttf");
    world.add_object(sentence_object);

    renderer::setup::start_engine(world);
}
