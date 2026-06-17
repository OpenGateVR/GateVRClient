pub mod renderer;
pub mod world;
pub mod interract;
pub mod setup;
pub mod network;
pub mod physics;

use world::object::Object;
use std::alloc;
use cap::Cap;

use crate::setup::fonts::load_font_uvs;
use crate::world::material::Material;
use crate::world::objects::text;
use crate::world::objects::cube;
use crate::world::{object::ObjectType, objects::fbx_parser::parse};

#[global_allocator]
static ALLOCATOR: Cap<alloc::System> = Cap::new(alloc::System, usize::max_value());

fn main() {
    let mut world = world::world::create_world();

    let skybox = parse("models/skybox.fbx", (0.0, 0.0, 0.0), (150.0, 150.0, 150.0), (0.0, 0.0, 0.0));
    let mut skybox_object = Object::create(
        ObjectType::Skybox,
        renderer::vertex::create_vertices_skinned(&skybox.0)
    );
    skybox_object.set_default_texture("textures/skybox_2.png");
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
        renderer::vertex::create_vertices_skinned(&table.0)
    );
    table_object.set_default_texture("textures/table.png");
    world.add_object(table_object);

    let ground = parse("models/plane.fbx", (0.0, 2.0, 0.0), (5.0, 2.0, 5.0), (0.0, 0.0, 0.0));
    let mut ground_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices_skinned(&ground.0)
    );
    ground_object.set_default_texture("textures/ground.jpg");
    ground_object.set_displacement("textures/ground_displacement.png");
    world.add_object(ground_object);

    /*let ground_sphere = parse("models/sphere.fbx", (0.0, 4.0, 6.0), (1.0, 1.0, 1.0), (0.0, 0.0, 0.0));
    let mut ground_sphere_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(ground_sphere.0, ground_sphere.2, ground_sphere.3, ground_sphere.1)
    );
    ground_sphere_object.set_displacement("textures/ground_displacement.png");
    world.add_object(ground_sphere_object);*/

    let niko = parse("models/nikoAnimated.fbx", (0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 0.0, 0.0));
    let mut niko_object = Object::create(
        ObjectType::SkinnedMesh,
        renderer::vertex::create_vertices_skinned(&niko.0)
    );
    niko_object.set_bones(niko.1, (0.0, 2.0, -2.0), (0.0, 0.0, 0.0), (2.0, 2.0, 2.0));
    niko_object.add_material(Material::from_texture("textures/niko.png"), "NikoMaterial");
    world.add_object(niko_object);

    /*let test = parse("models/SELESTIA.fbx", (3.0, 2.0, -2.0), (2.0, 2.0, 2.0), (0.0, 0.0, 0.0));
    let mut test_object = Object::create(
        ObjectType::Mesh,
        renderer::vertex::create_vertices(&test)
    );
    test_object.add_material(Material{ texture: "textures/Selestia_costume.png".to_string() }, "Selestia_costumeMaterial");
    test_object.add_material(Material{ texture: "textures/Selestia_hair.png".to_string() }, "Selestia_hairMaterial");
    test_object.add_material(Material{ texture: "textures/Selestia_body.png".to_string() }, "Selestia_bodyMaterial");
    test_object.add_material(Material{ texture: "textures/Selestia_face.png".to_string() }, "Selestia_optionMaterial");
    world.add_object(test_object);*/

    /*let figure = parse("models/niko.fbx", (0.0, 3.0, 0.0), (0.5, 0.5, 0.5), (0.0, 0.0, 0.0));
    let mut figure_object = Object::create(
        ObjectType::Grabbable,
        renderer::vertex::create_vertices(figure.0, figure.2, figure.3, figure.1)
    );
    figure_object.set_texture("textures/niko.png");
    figure_object.set_position((0.0, 3.0, 0.0));
    world.add_object(figure_object);*/

    let tablet = cube::create_cube((0.0, 0.0, 0.0), (0.5, 0.4, 0.01));
    let mut tablet_object = Object::create(
        ObjectType::TabletMenu,
        renderer::vertex::create_vertices(&tablet)
    );
    tablet_object.set_position((0.0, -10.0, 0.0));
    tablet_object.set_default_texture("textures/wall.jpg");
    world.add_object(tablet_object);

    let font_uvs = load_font_uvs("fonts/NotoSansJP.ttf");
    let sentence = text::create_plane_with_text(
        (-0.4, 0.3, -0.02), (0.03, 0.03, 1.0), 
        &font_uvs, "goodbye :C"
    );
    let mut sentence_object = Object::create(
        ObjectType::TabletMenu,
        renderer::vertex::create_vertices(&sentence)
    );
    sentence_object.set_default_texture("fonts/NotoSansJP.ttf");
    world.add_object(sentence_object);

    let chat_button = text::create_plane_with_text(
        (-0.4, 0.0, -0.02), (0.03, 0.03, 1.0), 
        &font_uvs, "CHAT"
    );
    let mut chat_button_object = Object::create(
        ObjectType::TabletMenuButton,
        renderer::vertex::create_vertices(&chat_button)
    );
    chat_button_object.set_default_texture("fonts/NotoSansJP.ttf");
    world.add_object(chat_button_object);

    let fps_label = text::create_plane_with_text(
        (-0.4, -0.3, -0.02), (0.02, 0.02, 1.0), 
        &font_uvs, "FPS: 0"
    );
    let mut fps_label_object = Object::create(
        ObjectType::TabletMenu,
        renderer::vertex::create_vertices(&fps_label)
    );
    fps_label_object.set_default_texture("fonts/NotoSansJP.ttf");
    fps_label_object.set_tag("fps_label");
    world.add_object(fps_label_object);

    let ram_label = text::create_plane_with_text(
        (-0.4, -0.2, -0.02), (0.02, 0.02, 1.0), 
        &font_uvs, "RAM: 0"
    );
    let mut ram_label_object = Object::create(
        ObjectType::TabletMenu,
        renderer::vertex::create_vertices(&ram_label)
    );
    ram_label_object.set_default_texture("fonts/NotoSansJP.ttf");
    ram_label_object.set_tag("ram_label");
    world.add_object(ram_label_object);

    println!("Memory after startup: {} MB", ALLOCATOR.allocated() as f32 / 1000000.0);

    renderer::setup::start_engine(world);
}
