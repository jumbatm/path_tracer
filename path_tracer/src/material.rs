use crate::colour;
use crate::ray;
use crate::WorldVec;

pub trait Material {
    /// Given some reflected ray, trace it in the backward direction. This should provide the BDRF
    /// for the material. Returns the new direction vector of the traced ray -- the origin of the
    /// ray is the origin of the surface_normal.
    fn sample_gathering_ray(&self, reflected_ray: &ray::Ray, surface_normal: &ray::Ray) -> ray::Ray;

    /// Given some ray, colour it in the forward direction based on the angle of incidence. This
    /// gives the material its colour attenuation properties. The angle of incidence is in radians.
    fn colour(&self, start_colour: colour::Colour, angle_of_incidence: f64) -> colour::Colour;

}
