pub fn create_cube(position: (f32, f32, f32), scale: (f32, f32, f32)) -> Vec<(Vec<[f32; 3]>, Vec<[i8; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, String)> {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[i8; 3]> = Vec::new();
    let mut colors: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);

    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([1.0, 0.0]);

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

    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);

    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([1.0, 0.0]);

    normals.push([-1, 0, 0]);
    normals.push([-1, 0, 0]);
    normals.push([-1, 0, 0]);
    normals.push([-1, 0, 0]);
    normals.push([-1, 0, 0]);
    normals.push([-1, 0, 0]);

    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);

    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);

    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([1.0, 0.0]);

    normals.push([0, 1, 0]);
    normals.push([0, 1, 0]);
    normals.push([0, 1, 0]);
    normals.push([0, 1, 0]);
    normals.push([0, 1, 0]);
    normals.push([0, 1, 0]);

    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);

    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);

    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([1.0, 0.0]);

    normals.push([0, -1, 0]);
    normals.push([0, -1, 0]);
    normals.push([0, -1, 0]);
    normals.push([0, -1, 0]);
    normals.push([0, -1, 0]);
    normals.push([0, -1, 0]);

    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);

    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1,  1.0 * scale.2 + position.2]);

    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([1.0, 0.0]);

    normals.push([0, 0, 1]);
    normals.push([0, 0, 1]);
    normals.push([0, 0, 1]);
    normals.push([0, 0, 1]);
    normals.push([0, 0, 1]);
    normals.push([0, 0, 1]);

    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);

    vertices.push([ 1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([ 1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0, -1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);
    vertices.push([-1.0 * scale.0 - position.0,  1.0 * scale.1 + position.1, -1.0 * scale.2 + position.2]);

    uvs.push([0.0, 1.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([0.0, 0.0]);
    uvs.push([1.0, 1.0]);
    uvs.push([1.0, 0.0]);

    normals.push([0, 0, -1]);
    normals.push([0, 0, -1]);
    normals.push([0, 0, -1]);
    normals.push([0, 0, -1]);
    normals.push([0, 0, -1]);
    normals.push([0, 0, -1]);

    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);
    colors.push([1.0, 1.0, 1.0]);

    return vec![(vertices, normals, colors, uvs, "default".to_string())];
}