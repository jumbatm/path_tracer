use crate::vec3;

macro_rules! max {
    ($ty: ty, $a: expr, $b: expr) => {
        ($a as $ty).max($b as $ty)
    };
    ($ty: ty, $a: expr, $($rest: expr),+) => {
        ($a as $ty).max(max!($ty, $($rest),+))
    };
}

#[derive(Debug, Clone)]
pub struct Colour(vec3::Vec3<f64>);
impl Colour {
    /// Create a new colour. All components will be normalised to sit between in the range [0, 1.0]
    pub fn new(r: f64, g: f64, b: f64) -> Colour {
        let max = max!(f64, 1.0, r, g, b);
        Colour(vec3::Vec3::new(r / max, g / max, b / max))
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max() {
        assert_eq!(max!(f64, 1.0, 5.0, 10.0, 1000.0), 1000.0);
        assert_eq!(max!(f64, 1.0, 5.0, 1000.0, 10.0), 1000.0);
        assert_eq!(max!(f64, 5.0, 1000.0, 10.0, 1.0), 1000.0);
    }

    #[test]
    fn sanity_test() {
        let colour = Colour::new(0.25, 0.5, 1.0);
        assert_eq!(colour.get_red(), 0.25);
        assert_eq!(colour.get_green(), 0.5);
        assert_eq!(colour.get_blue(), 1.0);
    }

}
