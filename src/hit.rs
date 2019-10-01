use crate::ray;
use crate::vec3;

/// Defines a type which can be hit with a ray. The returned ray is colour-attenuated and reflected
/// in the right direction.
pub trait Hit {
    fn hit(&self, from: ray::ColouredRay) -> Option<ray::ColouredRay>;
}

pub struct Sphere {
    origin: vec3::Vec3<f64>,
    radius: usize
}

impl Sphere {
    fn random_unit_vector_in_hemisphere() {
        // TODO
    }
}

impl Hit for Sphere {
    fn hit(&self, from: ray::ColouredRay) -> Option<ray::ColouredRay> {
        // Given a ray O + At, we can find the intersection of a sphere with center point C and
        // radius R by solving (P - C).^2 = R^2 for t, substituting P = O + At and solving for t. 
        let oc = *from.get_ray().get_origin() - self.origin;
        let a = from.get_ray().get_direction().dot(*from.get_ray().get_direction());
        let b = oc.dot(*from.get_ray().get_direction()) * 2.0;
        let c = oc.dot(oc) - self.radius.pow(2) as f64;

        // Check the descriminant.
        let descriminant = b.powf(2.0) - 4.0 * a * c;
        if descriminant < 0.0 {
            // No real solutions; ie, no intersection.
            None
        } else {
            // There is at least one intersection.
            let t = (b - descriminant.sqrt()) / (2.0 * a);
            let intersection_point = *from.get_ray().get_direction() * t + *from.get_ray().get_origin();
            
            // For now, just send back a test colour.
            Some(ray::ColouredRay::new(ray::Colour::new(255, 0, 0),ray::Ray::new(intersection_point, vec3::Vec3::new(1.0, 0.0, 0.0))))
        }
    }
}
