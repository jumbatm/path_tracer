use crate::hit;
use crate::ray;
use crate::material;

pub struct Scene<'scene> {
    objects: Vec<Box<dyn hit::Hit + 'scene>>,
}

impl<'scene> Scene<'scene> {
    pub fn new() -> Scene<'scene> {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add_object(&mut self, hit: impl hit::Hit + 'scene) {
        self.objects.push(Box::new(hit));
    }
}

impl hit::Hit for Scene<'_> {
    fn hit(&self, from: &ray::Ray) -> Option<hit::MaterialHit> {
        // Linearly search through the objects and see if they can be hit.
        for hittable in self.objects.iter() {
            if let Some(v) = hittable.hit(from) {
                return Some(v);
            }
        }
        None
    }
}
