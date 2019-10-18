use crate::colour;
use crate::material;
use crate::ray;
use crate::vec3;
use crate::WorldVec;

pub struct Lambertian {
    fuzziness: f32,
    colour: colour::Colour,
}

impl Lambertian {
    pub fn new(colour: colour::Colour, fuzziness: f32) -> Lambertian {
        Lambertian { fuzziness, colour }
    }
}
/// Produces a ray starting at some origin, pointing to some uniformly-distributed vector along
/// a unit sphere.
fn random_unit_vector_in_sphere() -> WorldVec {
    use rand::distributions::{Distribution, Uniform};
    use rand::SeedableRng;
    let between = Uniform::from(0.0..=2.0 * std::f64::consts::PI);
    static mut RNG: Option<rand::rngs::SmallRng> = None;
    unsafe {
        if let None = RNG {
            eprintln!("Initialised RNG");
            RNG = Some(rand::rngs::SmallRng::from_rng(rand::thread_rng()).unwrap());
        }
        from_spherical(
            1.0,
            between.sample(RNG.iter_mut().next().unwrap()),
            between.sample(RNG.iter_mut().next().unwrap()),
        )
    }
}

/// Create a vec3 from spherical coordinates.
fn from_spherical(radius: f64, inclination: f64, azimuth: f64) -> WorldVec {
    vec3::Vec3(
        radius * inclination.sin() * azimuth.cos(),
        radius * inclination.sin() * azimuth.sin(),
        radius * azimuth.cos(),
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
            (random_unit_vector_in_sphere() * self.fuzziness.into()).normalised(),
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
            self.colour.get_blue() * start_colour.get_blue()
        )
    }
}
