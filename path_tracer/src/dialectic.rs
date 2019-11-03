use crate::colour;
use crate::material;
use crate::ray;

#[derive(Debug)]
pub struct Dialectic {
    refractive_index: f64,
}

impl Dialectic {
    pub fn new(refractive_index: f64) -> Dialectic {
        Dialectic { refractive_index }
    }
}

impl material::Material for Dialectic {
    fn sample_gathering_ray(
        &self,
        reflected_ray: &ray::Ray,
        surface_normal: &ray::Ray,
    ) -> ray::Ray {
        let uv = reflected_ray.get_direction().normalised();
        let n = surface_normal.get_direction().normalised();
        let refractive_index_in = 1.0;
        let dt = uv.dot(n);
        let descriminant =
            1.0 - (refractive_index_in / self.refractive_index).powi(2) * (1.0 - dt.powi(2));
        if descriminant > 0.0 {
            let direction = (uv - n * dt) * (refractive_index_in / self.refractive_index)
                - n * descriminant.sqrt();
            ray::Ray::new(*surface_normal.get_origin(), direction)
        } else {
            *reflected_ray
        }
    }
    fn colour(
        &self,
        start_colour: colour::Colour,
        _surface_normal: &ray::Ray,
        _angle_of_incidence: f64,
    ) -> colour::Colour {
        start_colour
    }
}
