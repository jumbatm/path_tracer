use crate::colour;
use crate::material;
use crate::ray;
use crate::vec3;

#[derive(Clone)]
pub struct MaterialHit {
    pub material: std::rc::Rc<dyn material::Material>,
    pub intersected_surface_normal: ray::Ray,
}

/// Defines a type which can be hit with a ray. The returned ray is colour-attenuated and reflected
/// in the right direction.
///
pub trait Hit {
    /// Given some ray `from`, return a MaterialHit representing the material and surface normal. Note
    /// that this is "from" in the reverse direction.
    fn hit(&self, from: &ray::Ray) -> Option<MaterialHit>;
} 
