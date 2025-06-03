use std:: {mem};

// The vertex struct is one singular vertex
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
    pub uv: [f32; 4],
}
impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4, 3=>Float32x4];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

// Convert vertices to set vertex size
#[allow(dead_code)]
fn vertex(p:[f64;3], n:[i8; 3], c:[f32; 3], u:[f32; 2]) -> Vertex {
    return Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
        normal: [n[0] as f32, n[1] as f32, n[2] as f32, 1.0],
        color: [c[0], c[1], c[2], 1.0],
        uv: [u[0], u[1], 0.0, 0.0],
    }
}

// Convert object to list of vertices
pub fn create_vertices(vertices: Vec<[f64; 3]>, normals: Vec<[i8; 3]>, colors: Vec<[f32; 3]>, uvs: Vec<[f32; 2]>) -> Vec<Vertex> {
    let mut vertex_list: Vec<Vertex> = Vec::with_capacity(vertices.len());
    for i in 0..vertices.len() {
        vertex_list.push(vertex(vertices[i], normals[i], colors[i], uvs[i]));
    }
    return vertex_list.to_vec()
}