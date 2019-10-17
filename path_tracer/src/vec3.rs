#[derive(Debug, Clone)]
pub struct Vec3<T>(pub T, pub T, pub T);

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Vec3(T::default(), T::default(), T::default())
    }
}

impl<T: Copy> Copy for Vec3<T> {}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3(x, y, z)
    }
}

impl<T> Vec3<T>
where
    T: Copy + std::ops::Mul<Output = f64> + std::ops::Add<Output = f64>,
{
    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}

impl<T> Vec3<T>
where
    T: Copy
        + std::ops::Mul<Output = f64>
        + std::ops::Add<Output = f64>
        + std::ops::Div<Output = f64>
        + std::ops::Div<f64, Output = T>,
{
    pub fn normalised(self) -> Vec3<T> {
        Vec3(
            self.0 / self.length(),
            self.1 / self.length(),
            self.2 / self.length(),
        )
    }
}
impl<T: std::ops::Mul<Output = T> + std::ops::Add<Output = T>> Vec3<T> {
    pub fn dot(self, rhs: Self) -> T {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }
}

impl<T> Vec3<T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Copy,
{
    pub fn cross(self, rhs: Self) -> Self {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Vec3<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
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

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Vec3<T> {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3(-self.0, -self.1, -self.2)
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
        let u = Vec3::new(1, 2, 3);
        let v = Vec3::new(4, 3, 2);

        assert_eq!(u + v, Vec3::new(5, 5, 5));
    }

    #[test]
    fn test_dot() {
        let u = Vec3::new(1, 2, 3);
        let v = Vec3::new(4, 5, 6);

        assert_eq!(u.dot(v), 4 + 10 + 18);
    }

    #[test]
    fn test_cross() {
        let i = Vec3::new(1, 0, 0);
        let j = Vec3::new(0, 1, 0);
        let k = Vec3::new(0, 0, 1);

        assert_eq!(i.cross(j), k);
        assert_eq!(j.cross(i), -k);

        assert_eq!(j.cross(k), i);
        assert_eq!(k.cross(j), -i);

        assert_eq!(k.cross(i), j);
        assert_eq!(i.cross(k), -j);
    }
}
