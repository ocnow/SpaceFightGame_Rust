use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};

pub fn ball_collision(bullet_circle : BoundingCircle, 
    bounding_box : Aabb2d
) -> bool {
    return bullet_circle.intersects(&bounding_box);
}