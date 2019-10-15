use crate::hit;
use crate::ray;

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
    fn hit(&self, from: &ray::ColouredRay) -> Option<ray::ColouredRay> {
        // Linearly search through `objects`.
        for hittable in self.objects.iter() {
            if let Some(v) = hittable.hit(from) {
                return Some(v);
            }
        }
        None
    }
}
