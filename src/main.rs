use path_tracer::image::Image;
use path_tracer::vec3;

fn main() {
    let mut im = Image::new(800, 600);

    for y in 0..im.y_len() {
        for x in 0..im.x_len() {
            im[(x, y)] = vec3::Vec3::new((x as f32 / im.x_len() as f32 * 255.0) as u8, (y as f32 / im.y_len() as f32 * 255.0) as u8, (0.2 * 255.0) as u8);
        }
    }
    println!("{}", im.to_ppm());
}
