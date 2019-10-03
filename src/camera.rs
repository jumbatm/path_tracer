use crate::hit::Hit;
use crate::image;
use crate::vec3;

struct Camera {
    origin: vec3::Vec3<f64>,
    up: vec3::Vec3<f64>,
    right: vec3::Vec3<f64>,
    forward: vec3::Vec3<f64>,
}

impl Camera {
    fn new(
        scene: impl Hit,
        origin: vec3::Vec3<f64>,
        up: vec3::Vec3<f64>,
        forward: vec3::Vec3<f64>,
    ) -> Camera {
        Camera {
            origin,
            up: up.normalised(),
            forward: forward.normalised(),
            right: up.normalised().cross(forward.normalised()),
        }
    }

    fn render(
        x_size: usize,
        y_size: usize,
        fov: f64,
        bounces: usize,
        samples_per_pixel: usize,
    ) -> image::Image {
        let projection_plane_distance =
            (x_size as f64 / 2.0) * ((fov / 180.0 * std::f64::consts::PI) / 2.0).tan();
        let mut image = image::Image::new(x_size, y_size);
        for _ in 0..samples_per_pixel {
            for i in 0..x_size {
                for j in 0..y_size {
                    // Have a mutable coloured ray. Start it at the appropriate location on the
                    // projection plane.
                    for _ in 0..bounces {
                        // Find intersection.
                        // Have the Hit bounce it to a new direction and origin.
                        // Update the ray to this new position.
                        // (Loops around)
                    }
                    // Add to a total.
                }
            }
            // (Potentially blit an update to the screen)
        }
        // Divide by number of samples to make an average.
        // Return the result image.
        image
    }
}
