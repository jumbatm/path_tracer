use crate::colour;
use crate::material;
use crate::ray;
use crate::vec3;
use crate::WorldVec;

#[derive(Debug)]
pub struct Lambertian {
    colour: colour::Colour
}

impl Lambertian {
    pub fn new(colour: colour::Colour) -> Lambertian {
        Lambertian { colour }
    }
}
/// Produces a ray starting at some origin, pointing to some uniformly-distributed vector along
/// a unit sphere.
fn random_unit_vector_in_sphere() -> WorldVec {
    use rand::distributions::{Distribution, Uniform};
    let between = Uniform::from(0.0..=1.0);
    from_spherical(
        1.0,
        between.sample(&mut rand::thread_rng()) * std::f64::consts::PI,
        between.sample(&mut rand::thread_rng()) * 2.0 * std::f64::consts::PI,
    )
}

/// Create a vec3 from spherical coordinates. Note that the radius, inclination and azimuth are all
/// in radians. Inclination is from 0 to pi, azimuth is from 0 to 2 pi.
fn from_spherical(radius: f64, inclination: f64, azimuth: f64) -> WorldVec {
    vec3::Vec3(
        radius * inclination.sin() * azimuth.cos(),
        radius * inclination.sin() * azimuth.sin(),
        radius * inclination.cos(),
    )
}

impl material::Material for Lambertian {
    fn sample_gathering_ray(
        &self,
        _reflected_ray: &ray::Ray,
        surface_normal: &ray::Ray,
    ) -> ray::Ray {
        ray::Ray::new(
            /*origin=*/
            *surface_normal.get_origin(),
            /*direction=*/
            random_unit_vector_in_sphere().normalised(),
        )
    }

    fn colour(
        &self,
        start_colour: colour::Colour,
        _surface_normal: &ray::Ray,
        _angle_of_incidence: f64,
    ) -> colour::Colour {
        // Perfectly diffuse. Therefore, we ignore the angle of incidence.
        colour::Colour::new(
            self.colour.get_red() * start_colour.get_red(),
            self.colour.get_green() * start_colour.get_green(),
            self.colour.get_blue() * start_colour.get_blue(),
        )
    }
}
