use crate::colour;
use crate::hit::Hit;
use crate::image;
use crate::ray;
use crate::vec3;
use crate::WorldVec;

use rand::distributions::{Distribution, Uniform};
use rand::SeedableRng;

#[derive(Debug)]
pub struct Camera<T: Hit> {
    scene: std::rc::Rc<T>,
    origin: WorldVec,
    up: WorldVec,
    right: WorldVec,
    forward: WorldVec,
}

#[derive(Debug)]
pub enum RenderError {
    InvalidRegion {
        region: ((usize, usize), (usize, usize)),
        size: (usize, usize),
    },
}

impl<T: Hit> Camera<T> {
    pub fn new(
        scene: std::rc::Rc<T>,
        origin: WorldVec,
        up: WorldVec,
        forward: WorldVec,
    ) -> Camera<T> {
        let right = up.normalised().cross(forward.normalised()).normalised();
        // Now, re-generate the forward vector so that it's definitely pointing forward.
        let forward = right.cross(up).normalised();
        Camera {
            scene,
            origin,
            up: up.normalised(),
            forward,
            right,
        }
    }

    pub fn new_looking_at(
        scene: std::rc::Rc<T>,
        origin: WorldVec,
        up: WorldVec,
        looking_at: WorldVec,
    ) -> Camera<T> {
        // First, figure out what the forward vector would be.
        let forward = (looking_at - origin).normalised();

        // Now, we figure out a right vector. This is up x forward.
        let right = up.normalised().cross(forward.normalised());

        // Finally, regenerate up from these pair of vectors.
        let up = forward.normalised().cross(right.normalised());

        Camera::new(scene, origin, up, forward)
    }

    pub fn render(
        &self,
        x_size: usize,
        y_size: usize,
        fov: f64,
        bounces: usize,
        samples_per_pixel: usize,
    ) -> image::Image {
        self.render_region(
            (0, 0),
            (x_size, y_size),
            x_size,
            y_size,
            fov,
            bounces,
            samples_per_pixel,
        )
            .unwrap()
    }

    pub fn render_region(
        &self,
        region_top_left: (usize, usize),
        region_size: (usize, usize),
        x_size: usize,
        y_size: usize,
        fov: f64,
        bounces: usize,
        samples_per_pixel: usize,
    ) -> Result<image::Image, RenderError> {
        // First, check that the region is correct. For now, we only check that top_left <
        // bottom_right && bottom_right < (x_size, y_size).
        if region_top_left.0 + region_size.0 > x_size || region_top_left.1 + region_size.1 > y_size
        {
            return Err(RenderError::InvalidRegion {
                region: (
                            region_top_left,
                            (
                                region_top_left.0 + region_size.0,
                                region_top_left.1 + region_size.1,
                            ),
                        ),
                        size: (x_size, y_size),
            });
        }
        // We define the FOV as the horizonal field of vision.
        // We define the projection plane to be at distance of 1. Therefore:
        let alpha = (fov / 180.0) * std::f64::consts::PI;
        let projection_plane_half_width = (alpha / 2.0).tan();
        let projection_plane_pixel_width =
            (2.0 * (projection_plane_half_width as f64)) / (x_size as f64);

        // When we refer to the fov, we're referring to the horizontal fov. Therefore:
        let delta_i = (projection_plane_half_width * 2.0) / (x_size as f64);

        // We crop the top and bottom image rather than warping the entire image. Therefore, we
        // use the same delta as for the vertical case.
        let projection_plane_half_height = delta_i * (y_size as f64 / 2.0);
        let projection_plane_pixel_height =
            (2.0 * (projection_plane_half_height as f64)) / (y_size as f64);

        let top_left = self.origin - self.right * projection_plane_half_width
            + self.up * projection_plane_half_height;

        // For anti-aliasing:
        let mut rng = rand::rngs::SmallRng::from_rng(rand::thread_rng()).unwrap();
        let jitter_between = Uniform::from(-0.5..=0.5);

        // We store the data as floating point to average it later.
        let mut image_data = vec![(0.0, 0.0, 0.0); region_size.0 * region_size.1];

        for _ in 1..=samples_per_pixel {
            for i in (region_top_left.0)..(region_top_left.0 + region_size.0) {
                for j in (region_top_left.1)..(region_top_left.1 + region_size.1) {
                    // Have a mutable coloured ray. Start it on the projection plane in the
                    // appropiate place.
                    let projection_plane_point = top_left + self.right * delta_i * (i as f64)
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
                        }
                    + self.forward.normalised();
                    let mut current_ray = ray::Ray::new(
                        /*origin=*/ self.origin,
                        /*direction=*/ (projection_plane_point - self.origin).normalised(),
                    );

                    // Before we do anything, first get a pretty, sky-blue gradient.
                    let t = (current_ray.get_direction().normalised().1 + 1.0) * 0.5;
                    let colour = vec3::Vec3::new(1.0, 1.0, 1.0) * (1.0 - t)
                        + vec3::Vec3::new(0.5, 0.7, 1.0) * t;
                    let start_colour = colour::Colour::new(colour.0, colour.1, colour.2);

                    let mut reverse_path = Vec::new();
                    let mut of_interest = false;
                    let mut colour = start_colour.clone();

                    // First, build the path that this will go.
                    for bounce in 1..=bounces {
                        // Find intersection. Have the Hit bounce it to a new direction and origin.
                        current_ray = match self.scene.hit(&current_ray) {
                            Some(material_hit) => {
                                let normal = material_hit.intersected_surface_normal;
                                let new_ray = material_hit
                                    .material
                                    .sample_gathering_ray(&current_ray, &normal);
                                reverse_path.push(material_hit);
                                new_ray
                            }
                            None => {
                                if bounce > 2 {
                                    of_interest = true;
                                }
                                break;
                            }
                        }
                    }

                    // Now, do some colouring.
                    let mut path_iter = reverse_path.iter().rev();
                    // For node 0, we'll say that the angle of incidence is exactly 90 degrees
                    // (or, if you prefer, pi radians), indicating no attenuation due to viewing angle.
                    if let Some(hit) = path_iter.next() {
                        colour = hit.material.colour(
                            colour,
                            &hit.intersected_surface_normal,
                            std::f64::consts::PI,
                        );
                    }

                    for (prev, current) in reverse_path.iter().rev().zip(path_iter) {
                        // We need to calculate the angle of incidence. For node _n_ in the path, the
                        // direction of the bounced vector will be node _n_'s origin - node _n - 1_'s
                        // origin, normalised.  Therefore, the angle of incidence is the angle between
                        // the normal and this vector. We can calculate this by rearranging a . b = |a| |b|
                        // cos(theta), to theta = arccos((a . b) / (|a| |b|)). With unit vectors, |a| =
                        // |b| = 1.
                        let travel_direction = (*prev.intersected_surface_normal.get_origin()
                            - *current.intersected_surface_normal.get_origin())
                            .normalised();
                        let normal_direction = current
                            .intersected_surface_normal
                            .get_direction()
                            .normalised();
                        let angle_of_incidence = travel_direction.dot(normal_direction).acos();
                        colour = current.material.colour(
                            colour,
                            &current.intersected_surface_normal,
                            angle_of_incidence,
                        );
                    }

                    if of_interest {
                        // Print some stats about this path.
                        //eprintln!("Path size: {}, Starting colour: {:?}, End colour: {:?}", reverse_path.len(), start_colour, colour);
                    }

                    // Add to a total.
                    let image_data_i = i - region_top_left.0;
                    let image_data_j = j - region_top_left.1;
                    let pixel = &mut image_data[image_data_j * region_size.0 + image_data_i];
                    pixel.0 += colour.get_red();
                    pixel.1 += colour.get_green();
                    pixel.2 += colour.get_blue();
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
        let mut result = image::Image::new(region_size.0, region_size.1);
        for i in 0..region_size.0 {
            for j in 0..region_size.1 {
                let pixel = image_data[j * region_size.0 + i];
                result[(i, j)] = vec3::Vec3::new(
                    (pixel.0 * 255.0).round() as u8,
                    (pixel.1 * 255.0).round() as u8,
                    (pixel.2 * 255.0).round() as u8,
                );
            }
        }
        Ok(result)
    }
}
