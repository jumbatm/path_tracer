#[derive(Debug, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Vec3(T::default(), T::default(), T::default())
    }
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3(x, y, z)
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vec3<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Vec3<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl<T: Eq> Eq for Vec3<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let x = Vec3::new(1, 2, 3);
        let y = Vec3::new(4, 3, 2);

        assert_eq!(x + y, Vec3::new(5, 5, 5));
    }
}
