use crate::vec3;
use crate::colour;

#[derive(Debug)]
pub struct Ray {
    origin: vec3::Vec3<f64>,    // A point.
    direction: vec3::Vec3<f64>, // A unit vector pointing in the ray's direction.
}

impl Ray {
    pub fn new(origin: vec3::Vec3<f64>, direction: vec3::Vec3<f64>) -> Ray {
        Ray { origin, direction }
    }

    pub fn from_spherical(origin: vec3::Vec3<f64>, radius: f64, inclination: f64, azimuth: f64) -> Ray {
        let direction = vec3::Vec3(
            radius * inclination.sin() * azimuth.cos(),
            radius * inclination.sin() * azimuth.sin(),
            radius * azimuth.cos()
        );

        Ray::new(origin, direction)
    }
    pub fn get_origin(&self) -> &vec3::Vec3<f64> {
        &self.origin
    }
    pub fn get_direction(&self) -> &vec3::Vec3<f64> {
        &self.direction
    }
}

#[derive(Debug)]
pub struct ColouredRay(colour::Colour, Ray);
impl ColouredRay {
    pub fn new(colour: colour::Colour, ray: Ray) -> ColouredRay {
        ColouredRay(colour, ray)
    }
    pub fn get_colour(&self) -> &colour::Colour {
        &self.0
    }
    pub fn get_ray(&self) -> &Ray {
        &self.1
    }
    pub fn mut_colour(&mut self) -> &mut colour::Colour {
        &mut self.0
    }
    pub fn mut_ray(&mut self) -> &mut Ray {
        &mut self.1
    }
}
