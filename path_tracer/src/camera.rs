use crate::colour;
use crate::hit::Hit;
use crate::image;
use crate::ray;
use crate::vec3;

use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

pub struct Camera<T: Hit> {
    scene: T,
    origin: vec3::Vec3<f64>,
    up: vec3::Vec3<f64>,
    right: vec3::Vec3<f64>,
    forward: vec3::Vec3<f64>,
}

impl<T: Hit> Camera<T> {
    pub fn new(
        scene: T,
        origin: vec3::Vec3<f64>,
        up: vec3::Vec3<f64>,
        forward: vec3::Vec3<f64>,
    ) -> Camera<T> {
        Camera {
            scene,
            origin,
            up: up.normalised(),
            forward: forward.normalised(),
            right: up.normalised().cross(forward.normalised()).normalised(),
        }
    }

    pub fn render(
        &self,
        x_size: usize,
        y_size: usize,
        fov: f64,
        bounces: usize,
        samples_per_pixel: usize,
    ) -> image::Image {
        // We define the FOV as the horizonal field of vision.
        // We define the projection plane to be at distance of 1. Therefore:
        let alpha = (fov / 180.0) * std::f64::consts::PI;
        let projection_plane_half_width = dbg!((dbg!(alpha / 2.0)).tan());
        let projection_plane_pixel_width =
            (2.0 * (projection_plane_half_width as f64)) / (x_size as f64);

        // When we refer to the fov, we're referring to the horizontal fov. Therefore:
        let delta_i = (projection_plane_half_width * 2.0) / (x_size as f64);

        // We crop the top and bottom image rather than warping the entire image. Therefore, we
        // use the same delta as for the vertical case.
        let projection_plane_half_height = delta_i * (y_size as f64 / 2.0);
        let projection_plane_pixel_height =
            (2.0 * (projection_plane_half_height as f64)) / (y_size as f64);

        let top_left = dbg!(
            self.origin - dbg!(self.right) * projection_plane_half_width
                + self.up * projection_plane_half_height
        );

        // For anti-aliasing:
        let mut rng = rand::rngs::SmallRng::from_rng(rand::thread_rng()).unwrap();
        let jitter_between = Uniform::from(-0.5..=0.5);

        let mut image_data = vec![(0.0, 0.0, 0.0); x_size * y_size];
        for _ in 1..=samples_per_pixel {
            for i in 0..x_size {
                for j in 0..y_size {
                    // Have a mutable coloured ray. Start it on the projection plane in the
                    // appropiate place.
                    let ray = ray::Ray::new(
                        /*origin=*/
                        top_left + self.right * delta_i * (i as f64)
                            - self.up * delta_i * (j as f64)
                            + self.up // Antialiasing.
                                    * if samples_per_pixel > 0 {
                                        jitter_between.sample(&mut rng)
                                            * projection_plane_pixel_height
                                    } else {
                                        0.0
                                    }
                            + self.right
                                * if samples_per_pixel > 0 {
                                    jitter_between.sample(&mut rng) * projection_plane_pixel_width
                                } else {
                                    0.0
                                },
                        /*direction=*/ self.forward,
                    );

                    // Get a pretty, sky-blue gradient.
                    let t = (ray.get_origin().normalised().1 + 1.0) * 0.5;
                    let colour = vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + vec3::Vec3::new(0.5, 0.7, 1.0) * t;
                    let colour = colour::Colour::new(colour.0, colour.1, colour.2);

                    let mut current_ray =
                        ray::ColouredRay::new(colour, ray);
                    for _ in 0..bounces {
                        // Find intersection. Have the Hit bounce it to a new direction and origin.
                        current_ray = match self.scene.hit(&current_ray) {
                            Some(ray) => ray,
                            None => {
                                break;
                            }
                        }
                    }
                    // Add to a total.
                    let pixel = &mut image_data[j * x_size + i];
                    pixel.0 += current_ray.get_colour().get_red();
                    pixel.1 += current_ray.get_colour().get_green();
                    pixel.2 += current_ray.get_colour().get_blue();
                }
            }
            // (Potentially blit an update to the screen)
        }
        // Divide by number of samples to make an average.
        for colour_tuple in image_data.iter_mut() {
            colour_tuple.0 /= samples_per_pixel as f64;
            colour_tuple.1 /= samples_per_pixel as f64;
            colour_tuple.2 /= samples_per_pixel as f64;
        }
        // Convert this into an Image.
        let mut result = image::Image::new(x_size, y_size);
        for i in 0..x_size {
            for j in 0..y_size {
                let pixel = image_data[j * x_size + i];
                result[(i, j)] = vec3::Vec3::new(
                    (pixel.0 * 255.0).round() as u8,
                    (pixel.1 * 255.0).round() as u8,
                    (pixel.2 * 255.0).round() as u8,
                );
            }
        }
        result
    }
}
