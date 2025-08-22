pub fn create_plane(position: (f64, f64, f64), scale: (f64, f64, f64), atlas_size: (f32, f32), atlas_index: f32) -> (Vec<[f64; 3]>, Vec<[f32; 2]>, Vec<[i8; 3]>, Vec<[f32; 3]>) {
    let mut vertices: Vec<[f64; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let atlas_width = atlas_size.0;
    let atlas_height = atlas_size.1;

    vertices.push([ position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);

    let uv_x = (atlas_index as f32 % atlas_width).floor();
    let uv_y = (atlas_index as f32 / atlas_height).floor();
    uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
    uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
    uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
    uvs.push([0.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
    uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 1.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);
    uvs.push([1.0 / atlas_width + 1.0 / atlas_width * (uv_x), 0.0 / atlas_height + 1.0 / atlas_height * (uv_y)]);

    normals.push([1, 0, 0]);
    normals.push([1, 0, 0]);
    normals.push([1, 0, 0]);
    normals.push([1, 0, 0]);
    normals.push([1, 0, 0]);
    normals.push([1, 0, 0]);

    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);

    return (vertices, uvs, normals, colors);
}