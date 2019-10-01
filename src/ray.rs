use crate::vec3;

struct Ray {
    origin: vec3::Vec3<f64>,    // A point.
    direction: vec3::Vec3<f64>, // A unit vector pointing in the ray's direction.
}

impl Ray {
    fn new(origin: vec3::Vec3<f64>, direction: vec3::Vec3<f64>) -> Ray {
        Ray { origin, direction }
    }
    fn get_origin(&self) -> &vec3::Vec3<f64> {
        &self.origin
    }
    fn get_direction(&self) -> &vec3::Vec3<f64> {
        &self.direction
    }
}
