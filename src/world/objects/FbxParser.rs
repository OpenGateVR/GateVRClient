use rust_embed::RustEmbed;
use fbx::{File, Node};
use fbx::Property;
use std::io::{self, BufReader, Cursor};

#[derive(RustEmbed)]
#[folder = "client_assets/"]
struct Assets;

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
                "UVIndex" => {
                    for prop in &child.properties {
                        println!("  Property: {:?}", prop);
                        match prop {
                            Property::F64Array(arr) => {
                                uv.extend(arr);
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

pub fn parse(path: &str) -> (Vec<[f64; 3]>, Vec<[f32; 2]>, Vec<[i8; 3]>, Vec<[f32; 3]>) {
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

    let mut uvs_triangles = vec![];
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
                ]);
                uvs_triangles.push([
                    current_uvs[0],
                    current_uvs[j],
                    current_uvs[j + 1],
                ])
            }
            current_polygon.clear();
        }
    }

    for (i, tri) in triangles.iter().enumerate() {
        vertices.push([
            vertices_unparsed[tri[0]*3],
            vertices_unparsed[tri[0]*3+1],
            vertices_unparsed[tri[0]*3+2],
        ]);
        uvs.push([
            uvs_unparsed[uvs_triangles[i][0] as usize * 2] as f32, 
            uvs_unparsed[uvs_triangles[i][0] as usize * 2 + 1] as f32
        ]);
        normals.push([0, 0, 1]);
        colors.push([1.0, 0.5, 0.5]);
        vertices.push([
            vertices_unparsed[tri[1]*3],
            vertices_unparsed[tri[1]*3+1],
            vertices_unparsed[tri[1]*3+2],
        ]);
        uvs.push([
            uvs_unparsed[uvs_triangles[i][1] as usize * 2] as f32, 
            uvs_unparsed[uvs_triangles[i][1] as usize * 2 + 1] as f32
        ]);
        normals.push([0, 0, 1]);
        colors.push([1.0, 0.5, 0.5]);
        vertices.push([
            vertices_unparsed[tri[2]*3],
            vertices_unparsed[tri[2]*3+1],
            vertices_unparsed[tri[2]*3+2],
        ]);
        uvs.push([
            uvs_unparsed[uvs_triangles[i][2] as usize * 2] as f32, 
            uvs_unparsed[uvs_triangles[i][2] as usize * 2 + 1] as f32
        ]);
        normals.push([0, 0, 1]);
        colors.push([1.0, 0.5, 0.5]);
    }

    (vertices, uvs, normals, colors)
}