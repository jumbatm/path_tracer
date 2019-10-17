use crate::vec3;
use crate::colour;
use crate::WorldVec;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: WorldVec,    // A point.
    direction: WorldVec, // A unit vector pointing in the ray's direction.
}

impl Ray {
    pub fn new(origin: WorldVec, direction: WorldVec) -> Ray {
        Ray { origin, direction }
    }

    pub fn get_origin(&self) -> &WorldVec {
        &self.origin
    }
    pub fn get_direction(&self) -> &WorldVec {
        &self.direction
    }
}
