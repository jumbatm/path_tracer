use crate::material;
use crate::colour;
use crate::ray;
use crate::WorldVec;

#[derive(Debug)]
pub struct Debugon;
impl Debugon {
    pub fn new() -> Debugon {
        Debugon
    }
}
impl material::Material for Debugon {
    fn sample_gathering_ray(&self, reflected_ray: &ray::Ray, surface_normal: &ray::Ray) -> ray::Ray {
        *surface_normal
    }

    fn colour(&self, _start_colour: colour::Colour, surface_normal: &ray::Ray, _angle_of_incidence: f64) -> colour::Colour {
        let normal_direction = (surface_normal.get_direction().normalised() + WorldVec::new(1.0, 1.0, 1.0)) * 0.5;
        colour::Colour::new(normal_direction.0, normal_direction.1, normal_direction.2)
    }
}
