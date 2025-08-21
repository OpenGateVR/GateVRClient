use rust_embed::RustEmbed;
use fbx::{File, Node};
use fbx::Property;
use std::io::{self, BufReader, Cursor};

#[derive(RustEmbed)]
#[folder = "client_assets/"]
struct Assets;

fn traverse_nodes(node: &Node) {
    //println!("Node: {}", node.name);
    if node.name == "Geometry" {
        let mut vertices: Vec<f64> = vec![];
        let mut indices: Vec<i32> = vec![];

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

        println!("Vertices: {:?}", vertices);
        println!("Indices: {:?}", indices);
    }

    for child in &node.children {
        traverse_nodes(child);
    }
}

pub fn parse(path: &str) {
    let data = Assets::get(path).expect("Failed to get asset").data;
    let cursor = Cursor::new(data);
    let mut reader = BufReader::new(cursor);
    let file = File::read_from(&mut reader).expect("Failed to parse FBX file");

    println!("FBX Version: {:?}", file.version);
    for node in &file.children {
        traverse_nodes(node);
    }
}