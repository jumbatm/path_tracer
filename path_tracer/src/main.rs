use path_tracer::camera;
use path_tracer::vec3;
use path_tracer::scene;
use path_tracer::sphere;
use path_tracer::colour;
use path_tracer::lambertian;
use path_tracer::metal;

const TOP_SPHERE_RADIUS: f64 = 1.0;

fn main() {
    let mut scene = scene::Scene::new();

    let top_sphere1 = std::rc::Rc::new(sphere::Sphere::new(vec3::Vec3::new(-TOP_SPHERE_RADIUS, 0.0, 0.0), TOP_SPHERE_RADIUS, std::rc::Rc::new(lambertian::Lambertian::new(colour::Colour::new(0.5, 0.5, 0.5)))));
    let top_sphere2 = std::rc::Rc::new(sphere::Sphere::new(vec3::Vec3::new(TOP_SPHERE_RADIUS, 0.0, 0.0), TOP_SPHERE_RADIUS, std::rc::Rc::new(metal::Metal::new(colour::Colour::new(0.5, 0.5, 0.5), 1.0))));
    let bottom_sphere = std::rc::Rc::new(sphere::Sphere::new(vec3::Vec3::new(0.0, -100.0 - TOP_SPHERE_RADIUS, 0.0), 100.0, std::rc::Rc::new(lambertian::Lambertian::new(colour::Colour::new(0.5, 0.5, 0.5)))));

    scene.add_object(top_sphere1.clone());
    scene.add_object(top_sphere2.clone());
    scene.add_object(bottom_sphere.clone()); 

    let camera = camera::Camera::new(
        /*scene=*/std::rc::Rc::new(scene),
        /*origin=*/vec3::Vec3::new(0.0, 0.0, TOP_SPHERE_RADIUS + 2.0),
        /*up=*/vec3::Vec3::new(0.0, 1.0, 0.0),
        /*forward=*/vec3::Vec3::new(0.0, 0.0, -1.0),
    );

    let im = camera.render(/*x_size=*/800, /*y_size=*/600, /*fov=*/90.0, /*bounces=*/10, /*samples_per_pixel=*/100);
    println!("{}", im.to_ppm());
}
