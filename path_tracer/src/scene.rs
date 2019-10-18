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
        let mut current_hit_candidate: Option<hit::MaterialHit> = None;
        for hittable in self.objects.iter() {
            if let Some(new_hit) = hittable.hit(from) {
                // We have a hit.
                match &current_hit_candidate {
                    Some(hit_candidate) => {
                        // Get the distance between the current hit and the previous hit.
                        let origin = from.get_origin();
                        let hit_candidate_distance = (*hit_candidate.intersected_surface_normal.get_origin() - *origin).length();
                        let new_distance = (*new_hit.intersected_surface_normal.get_origin() - *origin).length();
                        if new_distance < hit_candidate_distance {
                            current_hit_candidate = Some(new_hit);
                        }
                    }
                    None => {
                        current_hit_candidate = Some(new_hit);
                    }

                }
            }
        }
        current_hit_candidate
    }
}
