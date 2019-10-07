use crate::colour;
use crate::ray;
use crate::vec3;

/// Defines a type which can be hit with a ray. The returned ray is colour-attenuated and reflected
/// in the right direction.
pub trait Hit {
    fn hit(&self, from: &ray::ColouredRay) -> Option<ray::ColouredRay>;
}

pub struct Sphere {
    origin: vec3::Vec3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: vec3::Vec3<f64>, radius: f64) -> Sphere {
        Sphere {
            origin,
            radius
        }
    }
    fn random_unit_vector_in_hemisphere() {
        // TODO
    }
}

impl Hit for Sphere {
    fn hit(&self, from: &ray::ColouredRay) -> Option<ray::ColouredRay> {
        // Given a ray O + At, we can find the intersection of a sphere with center point C and
        // radius R by solving (P - C).^2 = R^2 for t, substituting P = O + At and solving for t.
        let oc = *from.get_ray().get_origin() - self.origin;
        let a = from
            .get_ray()
            .get_direction()
            .dot(*from.get_ray().get_direction());
        let b = oc.dot(*from.get_ray().get_direction()) * 2.0;
        let c = oc.dot(oc) - self.radius.powi(2) as f64;

        // Check the descriminant.
        let descriminant = b.powf(2.0) - 4.0 * a * c;
        if descriminant < 0.0 {
            // No real solutions; ie, no intersection.
            None
        } else {
            // There is at least one intersection. 
            let t = (-b - descriminant.sqrt()) / (2.0 * a);
            if t < 0.0 {
                // We take the one that gives us positive t (indicating that the intersection is
                // ahead of the ray). Otherwise, we discard this ray.
                return None;
            }
            let intersection_point =
                *from.get_ray().get_direction() * t + *from.get_ray().get_origin();

            // For now, just send back a colour representing the normal.
            let normal = (intersection_point - self.origin).normalised();
            Some(ray::ColouredRay::new(
                colour::Colour::new(0.5 * (normal.0 + 1.0), 0.5 * (normal.1 + 1.0), 0.5 * (normal.2 + 1.0)),
                ray::Ray::new(intersection_point, normal),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hit_sphere() {
        // Have a sphere sat on the x axis, 5 units away.
        let unit_sphere = Sphere::new(vec3::Vec3::new(5.0, 0.0, 0.0), 1.0);

        // Cast a ray along the x axis. This should hit the center of the sphere.
        let ray = ray::ColouredRay::new(colour::Colour::new(1.0, 1.0, 1.0), ray::Ray::new(vec3::Vec3::new(0.0, 0.0, 0.0), vec3::Vec3::new(1.0, 0.0, 0.0)));

        unit_sphere.hit(&ray).unwrap();

        // Cast a ray along the y axis. This should not hit anything.
        let ray = ray::ColouredRay::new(colour::Colour::new(1.0, 1.0, 1.0), ray::Ray::new(vec3::Vec3::new(0.0, 0.0, 0.0), vec3::Vec3::new(0.0, 1.0, 0.0)));
        if let Some(_) = unit_sphere.hit(&ray) {
            panic!("Test failed.");
        }
    }
}
