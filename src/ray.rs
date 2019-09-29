use crate::vec3;

struct Ray {
    origin: vec3::Vec3<f64>,    // A point.
    direction: vec3::Vec3<f64>, // A unit vector pointing in the ray's direction.
}

impl Ray {
    fn new(origin: vec3::Vec3<f64>, direction: vec3::Vec3<f64>) -> Ray {
        Ray { origin, direction }
    }

    fn new_from_polar(origin: vec3::Vec3<f64>, elevation: f64, azimuth: f64) -> Ray {
        let direction = vec3::Vec3::new(
            azimuth.cos() * elevation.sin(),
            elevation.sin(),
            azimuth.sin(),
        );
        Ray::new(origin, direction)
    }
}
