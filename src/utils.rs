use bevy::math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume};

pub fn ball_collision(bullet_circle : BoundingCircle, 
    bounding_box : Aabb2d
) -> bool {
    bullet_circle.intersects(&bounding_box)
}