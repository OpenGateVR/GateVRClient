use rust_embed::RustEmbed;
use fbx::{File, Node};
use fbx::Property;
use std::collections::HashMap;
use std::io::{BufReader, Cursor};

#[derive(RustEmbed)]
#[folder = "client_assets/"]
struct Assets;

#[derive(Debug)]
struct Mesh {
    vertices: Vec<f64>,
    indices: Vec<i32>,
    uv: Vec<f64>,
    uv_indices: Vec<i32>,
    _normals: Vec<f64>,
    id: i64
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Mesh,
    Bone
}

#[derive(Debug)]
struct Transform {
    translation: (f64, f64, f64),
    rotation: (f64, f64, f64),
    scaling: (f64, f64, f64),
    name: String,
    id: i64,
    object: ObjectType
}

struct Connection {
    from: i64,
    to: i64,
}

fn rotate_x(v: [f64; 3], angle: f64) -> [f64; 3] {
    let (s, c) = angle.sin_cos();
    [
        v[0],
        v[1] * c - v[2] * s,
        v[1] * s + v[2] * c,
    ]
}

fn rotate_y(v: [f64; 3], angle: f64) -> [f64; 3] {
    let (s, c) = angle.sin_cos();
    [
        v[0] * c + v[2] * s,
        v[1],
        -v[0] * s + v[2] * c,
    ]
}

fn rotate_z(v: [f64; 3], angle: f64) -> [f64; 3] {
    let (s, c) = angle.sin_cos();
    [
        v[0] * c - v[1] * s,
        v[0] * s + v[1] * c,
        v[2],
    ]
}

fn get_id(node: &Node) -> Option<i64> {
    match node.properties.get(0) {
        Some(Property::I64(id)) => Some(*id),
        Some(Property::I32(id)) => Some(*id as i64),
        _ => None,
    }
}

fn parse_connections(node: &Node) -> Vec<Connection> {
    let mut conns = Vec::new();

    if node.name == "Connections" {
        for child in &node.children {
            if child.name == "C" && child.properties.len() >= 3 {
                let from = match &child.properties[1] {
                    Property::I64(v) => *v,
                    Property::I32(v) => *v as i64,
                    _ => continue,
                };
                let to = match &child.properties[2] {
                    Property::I64(v) => *v,
                    Property::I32(v) => *v as i64,
                    _ => continue,
                };

                conns.push(Connection { from, to });
            }
        }
    }

    conns
}

fn traverse_nodes(node: &Node) -> Vec<Mesh> {
    let mut meshes = Vec::new();

    if node.name == "Geometry" {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut uv = Vec::new();
        let mut uv_indices = Vec::new();
        let mut normals: Vec<f64> = Vec::new();

        for child in &node.children {
            match child.name.as_str() {
                "Vertices" => {
                    for prop in &child.properties {
                        if let Property::F64Array(arr) = prop {
                            vertices.extend(arr);
                        }
                    }
                }
                "PolygonVertexIndex" => {
                    for prop in &child.properties {
                        if let Property::I32Array(arr) = prop {
                            indices.extend(arr);
                        }
                    }
                }
                "LayerElementUV" => {
                    for sub in &child.children {
                        match sub.name.as_str() {
                            "UV" => {
                                for prop in &sub.properties {
                                    if let Property::F64Array(arr) = prop {
                                        uv.extend(arr);
                                    }
                                }
                            }
                            "UVIndex" => {
                                for prop in &sub.properties {
                                    if let Property::I32Array(arr) = prop {
                                        uv_indices.extend(arr);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                "LayerElementNormal" => {
                    for sub in &child.children {
                        match sub.name.as_str() {
                            "Normals" => {
                                for prop in &sub.properties {
                                    if let Property::F64Array(arr) = prop {
                                        normals.extend(arr);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        //println!("{:?}", normals);

        let id: i64;
        if let Some(id_found) = get_id(node) {
            id = id_found;
        } else {
            id = 0;
        }

        meshes.push(Mesh {
            vertices,
            indices,
            uv,
            uv_indices,
            _normals: normals,
            id
        });
    }

    // Recurse into children
    for child in &node.children {
        meshes.extend(traverse_nodes(child));
    }

    meshes
}

fn get_transform(node: &Node) -> Option<Transform> {
    if node.name == "Model" {
        let mut name = String::new();
        if let Some(Property::String(s)) = node.properties.get(1) {
            name = s.clone();
        }

        let mut translation = (0.0, 0.0, 0.0);
        let mut rotation = (0.0, 0.0, 0.0);
        let mut scaling = (1.0, 1.0, 1.0);

        for child in &node.children {
            if child.name == "Properties70" {
                for p in &child.children {
                    if p.name == "P" {
                        // Property format: ["Lcl Translation", "Lcl Translation", "", "A", x, y, z]
                        if p.properties.len() >= 7 {
                            // First element is a String
                            let prop_name = match &p.properties[0] {
                                Property::String(s) => s.as_str(),
                                _ => continue,
                            };

                            // Extract x, y, z as f64
                            let mut nums = [0.0, 0.0, 0.0];
                            for (i, n) in nums.iter_mut().enumerate() {
                                let idx = 4 + i;
                                if let Some(prop) = p.properties.get(idx) {
                                    *n = match prop {
                                        Property::F64(v) => *v,
                                        Property::I32(v) => *v as f64,
                                        Property::I64(v) => *v as f64,
                                        _ => 0.0,
                                    };
                                }
                            }
                            
                            match prop_name {
                                "Lcl Translation" => translation = (nums[0] / 100.0, nums[1] / 100.0, nums[2] / 100.0),
                                "Lcl Rotation" => rotation = (nums[0], nums[1], nums[2]),
                                "Lcl Scaling" => scaling = (nums[0] / 100.0, nums[1] / 100.0, nums[2] / 100.0),
                                _ => {}
                            }
                        }
                    }
                }
            }
        }

        let node_type = match node.properties.get(2) {
        Some(Property::String(s)) => s.as_str(),
            _ => "",
        };


        let id: i64;
        if let Some(id_found) = get_id(node) {
            id = id_found;
        } else {
            id = 0;
        }

        let mut object = ObjectType::Mesh;
        if node_type == "LimbNode" {
            object = ObjectType::Bone;
        }

        return Some(Transform {
            translation,
            rotation,
            scaling,
            name,
            id,
            object
        });
    }

    None
}

pub fn parse(path: &str, position: (f64, f64, f64), scale: (f64, f64, f64), rotation: (f64, f64, f64)) -> (Vec<[f64; 3]>, Vec<[f32; 2]>, Vec<[i8; 3]>, Vec<[f32; 3]>) {
    let data = Assets::get(path).expect("Failed to get asset").data;
    let cursor = Cursor::new(data);
    let mut reader = BufReader::new(cursor);
    let file = File::read_from(&mut reader).expect("Failed to parse FBX file");

    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut transforms = HashMap::new();
    let mut connections = Vec::new();

    for node in &file.children {
        if node.name == "Objects" {
            for child in &node.children {
                if let Some(transform) = get_transform(child) {
                    transforms.insert(transform.id, transform);
                }
            }
        }
        connections.extend(parse_connections(node));
    }

    // map bones
    /*for transform in &transforms {
        for connection in &connections {
            if &connection.from == transform.0 && transform.1.object == ObjectType::Bone {
                println!("Bone from {} to {}", connection.from, connection.to);
            }
        }
    }*/

    for node in &file.children {
        let meshes = traverse_nodes(node);
        for (index, mesh) in meshes.iter().enumerate() {
            let mut transform = &Transform{
                translation: (3.0 * index as f64, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0),
                scaling: (0.0, 0.0, 0.0),
                name: "Unknown".to_string(),
                id: 0,
                object: ObjectType::Bone
            };
            for connection in &connections {
                if connection.from == mesh.id {
                    if let Some(transform_found) = transforms.get(&connection.to) {
                        if transform_found.object == ObjectType::Bone { continue; }
                        transform = transform_found;
                        println!("Mesh: {}", transform.name);
                    }
                }
            }

            let mut triangles = vec![];
            let mut current_polygon = vec![];
            let mut current_uvs = vec![];

            for (index_uv, i) in mesh.indices.iter().enumerate() {
                let index = if *i < 0 {
                    ((-i) - 1) as usize
                } else {
                    *i as usize
                };
                current_polygon.push(index);
                current_uvs.push(mesh.uv_indices[index_uv as usize]);

                if *i < 0 {
                    // polygon ended, triangulate
                    for j in 1..current_polygon.len() - 1 {
                        triangles.push([
                            current_polygon[0],
                            current_polygon[j],
                            current_polygon[j + 1],
                            current_uvs[0] as usize,
                            current_uvs[j] as usize,
                            current_uvs[j + 1] as usize,
                        ]);
                    }
                    current_polygon.clear();
                    current_uvs.clear();
                }
            }

            for tri in triangles {
                let mut v = [
                    mesh.vertices[tri[0]*3] * scale.0 * transform.scaling.0,
                    mesh.vertices[tri[0]*3+1] * scale.2 * transform.scaling.2,
                    mesh.vertices[tri[0]*3+2] * scale.1 * transform.scaling.1,
                ];

                v = rotate_x(v, (rotation.0 + transform.rotation.0) * 0.0174532925);
                v = rotate_y(v, (rotation.1 + transform.rotation.1) * 0.0174532925);
                v = rotate_z(v, (rotation.2 + transform.rotation.2) * 0.0174532925);

                // translate
                v[0] += position.0 + transform.translation.0;
                v[1] += position.1 + transform.translation.1;
                v[2] += position.2 + transform.translation.2;

                vertices.push(v);
                uvs.push([
                    mesh.uv[tri[3] * 2] as f32, 
                    1.0 - mesh.uv[tri[3] * 2 + 1] as f32
                ]);
                normals.push([0, 1, 0]);
                colors.push([1.0, 1.0, 1.0]);
                
                let mut v = [
                    mesh.vertices[tri[1]*3] * scale.0 * transform.scaling.0,
                    mesh.vertices[tri[1]*3+1] * scale.2 * transform.scaling.2,
                    mesh.vertices[tri[1]*3+2] * scale.1 * transform.scaling.1,
                ];

                v = rotate_x(v, (rotation.0 + transform.rotation.0) * 0.0174532925);
                v = rotate_y(v, (rotation.1 + transform.rotation.1) * 0.0174532925);
                v = rotate_z(v, (rotation.2 + transform.rotation.2) * 0.0174532925);

                // translate
                v[0] += position.0 + transform.translation.0;
                v[1] += position.1 + transform.translation.1;
                v[2] += position.2 + transform.translation.2;

                vertices.push(v);
                uvs.push([
                    mesh.uv[tri[4] * 2] as f32, 
                    1.0 - mesh.uv[tri[4] * 2 + 1] as f32
                ]);
                normals.push([0, 1, 0]);
                colors.push([1.0, 1.0, 1.0]);

                let mut v = [
                    mesh.vertices[tri[2]*3] * scale.0 * transform.scaling.0,
                    mesh.vertices[tri[2]*3+1] * scale.2 * transform.scaling.2,
                    mesh.vertices[tri[2]*3+2] * scale.1 * transform.scaling.1,
                ];

                v = rotate_x(v, (rotation.0 + transform.rotation.0) * 0.0174532925);
                v = rotate_y(v, (rotation.1 + transform.rotation.1) * 0.0174532925);
                v = rotate_z(v, (rotation.2 + transform.rotation.2) * 0.0174532925);

                // translate
                v[0] += position.0 + transform.translation.0;
                v[1] += position.1 + transform.translation.1;
                v[2] += position.2 + transform.translation.2;

                vertices.push(v);
                uvs.push([
                    mesh.uv[tri[5] * 2] as f32, 
                    1.0 - mesh.uv[tri[5] * 2 + 1] as f32
                ]);
                normals.push([0, 1, 0]);
                colors.push([1.0, 1.0, 1.0]);
            }
        }
    }

    (vertices, uvs, normals, colors)
}