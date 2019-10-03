use crate::vec3;

pub struct Colour(vec3::Vec3<f64>);
impl Colour {
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        Colour(vec3::Vec3::new(r, g, b))
    }
    pub fn get_red(&self) -> f64 {
        (self.0).0
    }
    pub fn get_green(&self) -> f64 {
        (self.0).1
    }
    pub fn get_blue(&self) -> f64 {
        (self.0).2
    }
    pub fn mut_red(&mut self) -> &mut f64 {
        &mut (self.0).0
    }
    pub fn mut_green(&mut self) -> &mut f64 {
        &mut (self.0).1
    }
    pub fn mut_blue(&mut self) -> &mut f64 {
        &mut (self.0).2
    }
}
