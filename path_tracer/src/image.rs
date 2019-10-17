pub use crate::vec3;

type ImageDataType = u8;

// TODO: Provide an Iterator implementation which traverses the image in row major. Should be as
// easy as just exposing the underyling vec's.

pub struct Image {
    data: Vec<vec3::Vec3<ImageDataType>>,
    x_size: usize,
    y_size: usize,
}

impl Image {
    pub fn new(x_size: usize, y_size: usize) -> Image {
        Image {
            data: vec![vec3::Vec3::new(0, 0, 0); x_size * y_size],
            x_size,
            y_size,
        }
    }
    pub fn to_ppm(&self) -> String {
        let mut result = String::new();
        // PPM starts with magic number -- P3
        result += "P3\n";

        // Then, the width and height of the image in pixels.
        result += &format!("{} {}\n", self.x_size, self.y_size);

        // Next, the maximum value for each number.
        result += &format!("{}\n", ImageDataType::max_value());

        // Next, follows a series of RGB triplets.
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let colour = &self[(x, y)];
                result += &format!("{} {} {}", colour.0, colour.1, colour.2);
                if x != self.x_size - 1 {
                    result += "    "; // Space between each triplet.
                }
            }
            if y != self.y_size - 1 {
                result += "\n";
            }
        }
        result
    }

    pub fn x_len(&self) -> usize {
        self.x_size
    }

    pub fn y_len(&self) -> usize {
        self.y_size
    }
}

impl std::ops::Index<(usize, usize)> for Image {
    type Output = crate::vec3::Vec3<u8>;
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * self.x_size]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Image {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0 + index.1 * self.x_size]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialise() {
        let mut im = Image::new(3, 2);
        im[(0, 0)] = vec3::Vec3::new(255, 0, 0);
        im[(1, 0)] = vec3::Vec3::new(0, 255, 0);
        im[(2, 0)] = vec3::Vec3::new(0, 0, 255);
        im[(0, 1)] = vec3::Vec3::new(255, 255, 0);
        im[(1, 1)] = vec3::Vec3::new(255, 255, 255);
        im[(2, 1)] = vec3::Vec3::new(0, 0, 0);

        assert_eq!(
            im.to_ppm(),
            r#"P3
3 2
255
255 0 0    0 255 0    0 0 255
255 255 0    255 255 255    0 0 0"#
        )
    }
}
