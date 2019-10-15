use path_tracer::camera;
use path_tracer::hit;
use path_tracer::vec3;

fn main() {
    let camera = camera::Camera::new(
        /*scene=*/hit::Sphere::new(vec3::Vec3::new(1.0, 0.0, 0.0), 0.5),
        /*origin=*/vec3::Vec3::new(0.0, 0.0, 0.0),
        /*up=*/vec3::Vec3::new(0.0, 1.0, 0.0),
        /*forward=*/vec3::Vec3::new(1.0, 0.0, 0.0),
    );
    let im = camera.render(800, 600, 90.0, 1, 100);
    println!("{}", im.to_ppm());
}
