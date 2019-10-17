use crate::material;
use crate::colour;
use crate::ray;

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

    fn colour(&self, start_colour: colour::Colour, angle_of_incidence: f64) -> colour::Colour {
        colour::Colour::new(0.5 * start_colour.get_red(), 0.5 * start_colour.get_green(), 0.5 * start_colour.get_blue())
    }
}
