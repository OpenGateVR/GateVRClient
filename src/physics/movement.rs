use cgmath::*;

pub fn get_camera_rotation(camera_rotation_x: f32, camera_rotation_y: f32, mouse: [f32; 2], frame_time: f32) -> (f32, f32) {
    let mut rotation_x = camera_rotation_x + mouse[1] * (frame_time * 0.02);
    rotation_x = rotation_x.clamp(-std::f32::consts::FRAC_PI_2 / 1.01, std::f32::consts::FRAC_PI_2 / 1.01);
    let rotation_y = camera_rotation_y - mouse[0] * (frame_time * 0.02);
    (rotation_x, rotation_y)
}

pub fn get_camera_movement(
    mut camera_acceleration_walking: (f32, f32, f32), keys: [bool; 6], 
    forward: Vector3<f32>, frame_time: f32, camera_rotation: (f32, f32, f32)) -> (f32, f32, f32) {
    
    let mut forward_x = forward[0];
    let mut forward_z = forward[2];

    let len = (forward_x * forward_x + forward_z * forward_z).sqrt();
    if len > 0.0 {
        forward_x /= len;
        forward_z /= len;
    }

    let right = Vector3::new(
        camera_rotation.1.sin(),
        0.0,
        -camera_rotation.1.cos(),
    ).normalize();

    camera_acceleration_walking = (0.0, camera_acceleration_walking.1, 0.0).into();
    if keys[0] {
        camera_acceleration_walking.0 += frame_time * forward_x;
        camera_acceleration_walking.2 += frame_time * forward_z;
    }
    if keys[2] {
        camera_acceleration_walking.0 -= frame_time * forward_x;
        camera_acceleration_walking.2 -= frame_time * forward_z;
    }
    if keys[1] {
        camera_acceleration_walking.0 += frame_time * right[0];
        camera_acceleration_walking.2 += frame_time * right[2];
    }
    if keys[3] {
        camera_acceleration_walking.0 -= frame_time * right[0];
        camera_acceleration_walking.2 -= frame_time * right[2];
    }
    if keys[4] {
        camera_acceleration_walking.1 += frame_time * 10.0;
    }
    (camera_acceleration_walking.0 * 1.0 * frame_time, camera_acceleration_walking.1 * 1.0 * frame_time, camera_acceleration_walking.2 * 1.0 * frame_time)
}