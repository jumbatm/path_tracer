use crate::colour;
use crate::ray;
use crate::WorldVec;
use crate::vec3;

pub trait Material: std::fmt::Debug {
    /// Given some reflected ray, trace it in the backward direction. This should provide the BDRF
    /// for the material. Returns the new direction vector of the traced ray -- the origin of the
    /// ray is the origin of the surface_normal.
    fn sample_gathering_ray(&self, reflected_ray: &ray::Ray, surface_normal: &ray::Ray) -> ray::Ray;

    /// Given some ray, colour it in the forward direction based on the angle of incidence. This
    /// gives the material its colour attenuation properties. The angle of incidence is in radians.
    fn colour(&self, start_colour: colour::Colour, surface_normal: &ray::Ray, angle_of_incidence: f64) -> colour::Colour;

}

/// Produces a ray starting at some origin, pointing to some uniformly-distributed vector along
/// a unit sphere.
pub fn random_unit_vector_in_sphere() -> WorldVec {
    use rand::distributions::{Distribution, Uniform};
    let between = Uniform::from(0.0..=1.0);
    /// Create a vec3 from spherical coordinates. Note that the radius, inclination and azimuth are all
    /// in radians. Inclination is from 0 to pi, azimuth is from 0 to 2 pi.
    fn from_spherical(radius: f64, inclination: f64, azimuth: f64) -> WorldVec {
        vec3::Vec3(
            radius * inclination.sin() * azimuth.cos(),
            radius * inclination.sin() * azimuth.sin(),
            radius * inclination.cos(),
        )
    }
    from_spherical(
        1.0,
        between.sample(&mut rand::thread_rng()) * std::f64::consts::PI,
        between.sample(&mut rand::thread_rng()) * 2.0 * std::f64::consts::PI,
    )
}

