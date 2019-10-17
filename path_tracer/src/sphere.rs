use crate::hit;
use crate::ray;
use crate::material;
use crate::lambertian;
use crate::WorldVec;

pub struct Sphere {
    origin: WorldVec,
    radius: f64,
    material: std::rc::Rc<dyn material::Material>
}

impl Sphere {
    pub fn new(origin: WorldVec, radius: f64, material: std::rc::Rc<lambertian::Lambertian>) -> Sphere {
        Sphere { origin, radius, material }
    }
}

impl hit::Hit for Sphere {
    fn hit(&self, from: &ray::Ray) -> Option<hit::MaterialHit> {
        // Given a ray O + At, we can find the intersection of a sphere with center point C and
        // radius R by solving (P - C).^2 = R^2 for t, substituting P = O + At and solving for t.
        let oc = *from.get_origin() - self.origin;
        let a = from.get_direction().dot(*from.get_direction());
        let b = oc.dot(*from.get_direction()) * 2.0;
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
            let intersection_point = *from.get_direction() * t + *from.get_origin();

            let normal_direction = (intersection_point - self.origin).normalised();
            let intersected_surface_normal = ray::Ray::new(intersection_point, normal_direction);
            Some(
                hit::MaterialHit {
                    material: std::rc::Rc::clone(&self.material),
                    intersected_surface_normal,
            }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::colour;
    use crate::vec3;
    use crate::ray;

    use crate::hit::Hit;

    #[test]
    fn test_hit_sphere() {
        // Have a sphere sat on the x axis, 5 units away.
        let unit_sphere = Sphere::new(vec3::Vec3::new(5.0, 0.0, 0.0), 1.0, std::rc::Rc::new(lambertian::Lambertian::new(colour::Colour::new(0.9, 0.0, 0.0), 1.0)));

        // Cast a ray along the x axis. This should hit the center of the sphere.
        let ray = ray::Ray::new(
                vec3::Vec3::new(0.0, 0.0, 0.0),
                vec3::Vec3::new(1.0, 0.0, 0.0),
            );

        unit_sphere.hit(&ray).expect("The ray should hit.");

        // Cast a ray along the y axis. This should not hit anything.
        let ray = 
            ray::Ray::new(
                vec3::Vec3::new(0.0, 0.0, 0.0),
                vec3::Vec3::new(0.0, 1.0, 0.0),
        );
        if let Some(_) = unit_sphere.hit(&ray) {
            panic!("Test failed.");
        }
    }
}
