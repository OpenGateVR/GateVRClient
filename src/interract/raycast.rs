use crate::world::object::{Object, ObjectType};

pub fn distance(position_1: (f32, f32, f32), position_2: (f32, f32, f32)) -> f32 {
    let dx = position_1.0 - position_2.0;
    let dy = position_1.1 - position_2.1;
    let dz = position_1.2 - position_2.2;

    (dx * dx + dy * dy + dz * dz).sqrt()
}

// the function to get the nearest grabbable object
pub fn raycast_grab(objects: &Vec<Object>, position: (f32, f32, f32), direction: cgmath::Vector3<f32>, max_distance: usize) -> usize {
    let mut position_checking: (f32, f32, f32) = (0.0, 0.0, 0.0);
    position_checking.0 = position.0 + direction.x / 10.0;
    position_checking.1 = position.1 + direction.y / 10.0;
    position_checking.2 = position.2 + direction.z / 10.0;
    for _ in 0..max_distance*10 {
        for (index, object) in objects.iter().enumerate() {
            if object.get_object_type() != ObjectType::Grabbable { continue; }
            let distance_found = distance(position_checking, object.get_position());
            if distance_found < 0.5 {
                return index
            }
        }
        position_checking.0 += direction.x / 10.0;
        position_checking.1 += direction.y / 10.0;
        position_checking.2 += direction.z / 10.0;
    }
    0
}