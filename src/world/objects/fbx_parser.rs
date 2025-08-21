use rust_embed::RustEmbed;
use fbx::{File, Node};
use fbx::Property;
use std::io::{BufReader, Cursor};

#[derive(RustEmbed)]
#[folder = "client_assets/"]
struct Assets;

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

fn traverse_nodes(node: &Node) -> (Vec<f64>, Vec<i32>, Vec<f64>, Vec<i32>) {
    let mut vertices: Vec<f64> = vec![];
    let mut indices: Vec<i32> = vec![];

    let mut uv: Vec<f64> = vec![];
    let mut uv_indices: Vec<i32> = vec![];

    if node.name == "Geometry" {
        for child in &node.children {
            match child.name.as_str() {
                "Vertices" => {
                    for prop in &child.properties {
                        match prop {
                            Property::F64Array(arr) => {
                                vertices.extend(arr);
                            }
                            _ => {}
                        }
                    }
                }
                "PolygonVertexIndex" => {
                    for prop in &child.properties {
                        match prop {
                            Property::I32Array(arr) => {
                                indices.extend(arr);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    } else if node.name == "UV" {
        for prop in &node.properties {
            match prop {
                Property::F64Array(arr) => {
                    uv.extend(arr);
                }
                _ => {}
            }
        }
    } else if node.name == "UVIndex" {
        for prop in &node.properties {
            match prop {
                Property::I32Array(arr) => {
                    uv_indices.extend(arr);
                }
                _ => {}
            }
        }
    }

    for child in &node.children {
        let (vertices_out, indices_out, uv_out, uv_indices_out) = traverse_nodes(child);
        vertices.extend(vertices_out);
        indices.extend(indices_out);
        uv.extend(uv_out);
        uv_indices.extend(uv_indices_out);
    }

    (vertices, indices, uv, uv_indices)
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

    println!("FBX Version: {:?}", file.version);
    let mut vertices_unparsed: Vec<f64> = vec![];
    let mut indices_unparsed: Vec<i32> = vec![];
    let mut uvs_unparsed: Vec<f64> = vec![];
    let mut uv_indices_unparsed: Vec<i32> = vec![];
    for node in &file.children {
        let (vertices_out, indices_out, uv_out, uv_indices_out) = traverse_nodes(node);
        vertices_unparsed.extend(vertices_out);
        indices_unparsed.extend(indices_out);
        uvs_unparsed.extend(uv_out);
        uv_indices_unparsed.extend(uv_indices_out);
    }

    let mut triangles = vec![];
    let mut current_polygon = vec![];
    let mut current_uvs = vec![];

    for (index_uv, i) in indices_unparsed.iter().enumerate() {
        let index = if *i < 0 {
            ((-i) - 1) as usize
        } else {
            *i as usize
        };
        current_polygon.push(index);
        current_uvs.push(uv_indices_unparsed[index_uv as usize]);

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

    //println!("{:?}", triangles);

    for tri in triangles {
        let mut v = [
            vertices_unparsed[tri[0]*3] * scale.0,
            vertices_unparsed[tri[0]*3+1] * scale.2,
            vertices_unparsed[tri[0]*3+2] * scale.1,
        ];

        v = rotate_x(v, -std::f64::consts::FRAC_PI_2);
        v = rotate_x(v, rotation.0 * 0.0174532925);
        v = rotate_y(v, rotation.1 * 0.0174532925);
        v = rotate_z(v, rotation.2 * 0.0174532925);

        // translate
        v[0] += position.0;
        v[1] += position.1;
        v[2] += position.2;

        vertices.push(v);
        uvs.push([
            uvs_unparsed[tri[3] * 2] as f32, 
            1.0 - uvs_unparsed[tri[3] * 2 + 1] as f32
        ]);
        normals.push([0, 1, 0]);
        colors.push([1.0, 1.0, 1.0]);
        
        let mut v = [
            vertices_unparsed[tri[1]*3] * scale.0,
            vertices_unparsed[tri[1]*3+1] * scale.2,
            vertices_unparsed[tri[1]*3+2] * scale.1,
        ];

        v = rotate_x(v, -std::f64::consts::FRAC_PI_2);
        v = rotate_x(v, rotation.0 * 0.0174532925);
        v = rotate_y(v, rotation.1 * 0.0174532925);
        v = rotate_z(v, rotation.2 * 0.0174532925);

        // translate
        v[0] += position.0;
        v[1] += position.1;
        v[2] += position.2;

        vertices.push(v);
        uvs.push([
            uvs_unparsed[tri[4] * 2] as f32, 
            1.0 - uvs_unparsed[tri[4] * 2 + 1] as f32
        ]);
        normals.push([0, 1, 0]);
        colors.push([1.0, 1.0, 1.0]);

        let mut v = [
            vertices_unparsed[tri[2]*3] * scale.0,
            vertices_unparsed[tri[2]*3+1] * scale.2,
            vertices_unparsed[tri[2]*3+2] * scale.1,
        ];

        v = rotate_x(v, -std::f64::consts::FRAC_PI_2);
        v = rotate_x(v, rotation.0 * 0.0174532925);
        v = rotate_y(v, rotation.1 * 0.0174532925);
        v = rotate_z(v, rotation.2 * 0.0174532925);

        // translate
        v[0] += position.0;
        v[1] += position.1;
        v[2] += position.2;

        vertices.push(v);
        uvs.push([
            uvs_unparsed[tri[5] * 2] as f32, 
            1.0 - uvs_unparsed[tri[5] * 2 + 1] as f32
        ]);
        normals.push([0, 1, 0]);
        colors.push([1.0, 1.0, 1.0]);
    }

    (vertices, uvs, normals, colors)
}