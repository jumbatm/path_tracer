use crate::colour;
use crate::material;
use crate::ray;

#[derive(Debug)]
pub struct Lambertian {
    colour: colour::Colour,
}

impl Lambertian {
    pub fn new(colour: colour::Colour) -> Lambertian {
        Lambertian { colour }
    }
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
            material::random_unit_vector_in_sphere().normalised(),
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
