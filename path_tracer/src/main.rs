use path_tracer::camera;
use path_tracer::hit;
use path_tracer::vec3;
use path_tracer::scene;

fn main() {
    let mut scene = scene::Scene::new();
    scene.add_object(hit::Sphere::new(vec3::Vec3::new(0.5, 0.0, 0.0), 0.5));
    scene.add_object(hit::Sphere::new(vec3::Vec3::new(-0.5, 0.0, 0.0), 0.5));
    let camera = camera::Camera::new(
        /*scene=*/scene,
        /*origin=*/vec3::Vec3::new(0.0, 0.0, 1.0),
        /*up=*/vec3::Vec3::new(0.0, 1.0, 0.0),
        /*forward=*/vec3::Vec3::new(0.0, 0.0, -1.0),
    );

    let im = camera.render(800, 600, 90.0, 100, 100);
    println!("{}", im.to_ppm());
}
