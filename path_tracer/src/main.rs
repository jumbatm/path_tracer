use path_tracer::camera;
use path_tracer::vec3;
use path_tracer::scene;
use path_tracer::sphere;
use path_tracer::colour;
use path_tracer::lambertian;
use path_tracer::debugon;

fn main() {
    let mut scene = scene::Scene::new();
    scene.add_object(sphere::Sphere::new(vec3::Vec3::new(-0.5, 0.0, 0.0), 0.5, std::rc::Rc::new(lambertian::Lambertian::new(colour::Colour::new(0.5, 0.5, 0.5), 1.0))));
    scene.add_object(sphere::Sphere::new(vec3::Vec3::new(0.5, 0.0, -0.75), 0.5, std::rc::Rc::new(lambertian::Lambertian::new(colour::Colour::new(0.5, 0.5, 0.5), 1.0))));
    let camera = camera::Camera::new(
        /*scene=*/scene,
        /*origin=*/vec3::Vec3::new(0.0, 0.0, 1.0),
        /*up=*/vec3::Vec3::new(0.0, 1.0, 0.0),
        /*forward=*/vec3::Vec3::new(0.0, 0.0, -1.0),
    );

    let im = camera.render(/*x_size=*/800, /*y_size=*/600, /*fov=*/90.0, /*bounces=*/10, /*samples_per_pixel=*/100);
    println!("{}", im.to_ppm());
}
