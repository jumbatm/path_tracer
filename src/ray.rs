use crate::vec3;

pub struct Ray {
    origin: vec3::Vec3<f64>,    // A point.
    direction: vec3::Vec3<f64>, // A unit vector pointing in the ray's direction.
}

impl Ray {
    pub fn new(origin: vec3::Vec3<f64>, direction: vec3::Vec3<f64>) -> Ray {
        Ray { origin, direction }
    }
    pub fn get_origin(&self) -> &vec3::Vec3<f64> {
        &self.origin
    }
    pub fn get_direction(&self) -> &vec3::Vec3<f64> {
        &self.direction
    }
}

pub struct Colour(vec3::Vec3<u8>);
impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Colour {
        Colour(vec3::Vec3::new(r, g, b))
    }
    pub fn get_red(&self) -> u8 {
        (self.0).0
    }
    pub fn get_green(&self) -> u8 {
        (self.0).1
    }
    pub fn get_blue(&self) -> u8 {
        (self.0).2
    }
    pub fn mut_red(&mut self) -> &mut u8 {
        &mut (self.0).0
    }
    pub fn mut_green(&mut self) -> &mut u8 {
        &mut (self.0).1
    }
    pub fn mut_blue(&mut self) -> &mut u8 {
        &mut (self.0).2
    }
}

pub struct ColouredRay(Colour, Ray);
impl ColouredRay {
    pub fn new(colour: Colour, ray: Ray) -> ColouredRay {
        ColouredRay(colour, ray)
    }
    pub fn get_colour(&self) -> &Colour {
        &self.0
    }
    pub fn get_ray(&self) -> &Ray {
        &self.1
    }
    pub fn mut_colour(&mut self) -> &mut Colour {
        &mut self.0
    }
    pub fn mut_ray(&mut self) -> &mut Ray {
        &mut self.1
    }
}
