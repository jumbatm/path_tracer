use crate::colour;
use crate::material;
use crate::ray;

#[derive(Debug)]
pub struct Metal {
    colour: colour::Colour,
    fuzziness: f32,
}

impl Metal {
    pub fn new(colour: colour::Colour, fuzziness: f32) -> Metal {
        Metal { colour, fuzziness }
    }
}

impl material::Material for Metal {
    fn sample_gathering_ray(
        &self,
        reflected_ray: &ray::Ray,
        surface_normal: &ray::Ray,
    ) -> ray::Ray {
        // Metals reflect the ray with the same angle of reflection as angle of incidence (relative
        // to the normal). We know the direction of the incident ray (we're tracing in reverse)
        // should be o + reflected_ray + 2 * |reflected_ray| * cos(theta) * N, where o represents
        // the point that was hit, , theta represents the angle of incidence and N represents the
        // normal. As we also know: a . b = |a| |b| cos(theta) We can see, given a normalised
        // surface normal, we can perform |reflected_ray| cos(theta) as just N . reflected_ray.
        ray::Ray::new(
            *surface_normal.get_origin(),
            *reflected_ray.get_direction()
                - *surface_normal.get_direction()
                    * surface_normal
                        .get_direction()
                        .normalised()
                        .dot(*reflected_ray.get_direction())
                    * 2.0,
        )
    }
    fn colour(
        &self,
        start_colour: colour::Colour,
        _surface_normal: &ray::Ray,
        _angle_of_incidence: f64,
    ) -> colour::Colour {
        colour::Colour::new(
            self.colour.get_red() * start_colour.get_red(),
            self.colour.get_green() * start_colour.get_green(),
            self.colour.get_blue() * start_colour.get_blue(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    #[test]
    fn test_reflected_ray() {
        use material::Material;

        let metal = Metal::new(colour::Colour::new(1.0, 1.0, 1.0), 1.0);
        // We expect that the angle of incidence == angle of reflectance.
        let normal = ray::Ray::new(
            vec3::Vec3::new(0.0, 0.0, 0.0),
            vec3::Vec3::new(0.0, 1.0, 0.0).normalised(),
        );
        use rand::distributions::{Distribution, Uniform};
        let between = Uniform::from(-1.0..=1.0);

        for _ in 0..500 {
            // Set up an incident ray.
            let incident_ray_direction = vec3::Vec3::new(
                between.sample(&mut rand::thread_rng()),
                between.sample(&mut rand::thread_rng()),
                between.sample(&mut rand::thread_rng()),
            );
            let incident_ray_origin = -incident_ray_direction;

            let incident_ray =
                ray::Ray::new(incident_ray_origin, incident_ray_direction.normalised());

            let cos_incident_angle = (-*incident_ray.get_direction())
                .normalised()
                .dot(normal.get_direction().normalised());

            // Now, send a ray to bounce off the metal.
            let reflected_ray = metal.sample_gathering_ray(&incident_ray, &normal);
            let cos_reflected_ray = reflected_ray
                .get_direction()
                .normalised()
                .dot(normal.get_direction().normalised());

            assert_eq!(cos_incident_angle, cos_reflected_ray);
        }
    }
}
