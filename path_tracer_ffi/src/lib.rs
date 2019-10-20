use path_tracer::*;

use std::convert::TryInto;

pub type CCamera = camera::Camera<scene::Scene<'static>>;
pub type CVec3 = vec3::Vec3<f64>;
pub type CMaterial = std::rc::Rc<dyn material::Material>;
pub type CSphere = std::rc::Rc<sphere::Sphere>;
pub type CHit = std::rc::Rc<dyn hit::Hit + 'static>;

use std::os::raw::{c_double, c_float};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Pixel {
    red: u8,
    green: u8,
    blue: u8,
}

pub struct CImage(pub image::Image);
impl CImage {
    pub fn new(image: image::Image) -> CImage {
        CImage(image)
    }
    pub fn at(&self, i: u64, j: u64) -> Pixel {
        if i > self.0.x_len() as u64 || j > self.0.y_len() as u64 {
            Pixel {
                red: 0,
                green: 0,
                blue: 0,
            }
        } else {
            let im = self.0[(i as usize, j as usize)];
            Pixel {
                red: im.0,
                green: im.1,
                blue: im.2,
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn PT_Image_delete(image: *mut CImage) {
    if !image.is_null() {
        Box::from_raw(image);
    }
}

#[no_mangle]
pub unsafe extern "C" fn PT_Vec3_new(x: c_double, y: c_double, z: c_double) -> *mut CVec3 {
    Box::into_raw(Box::new(vec3::Vec3::new(x, y, z)))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Vec3_delete(vec: *mut CVec3) {
    if !vec.is_null() {
        Box::from_raw(vec);
    }
}

#[no_mangle]
pub unsafe extern "C" fn PT_Scene_new() -> *mut scene::Scene<'static> {
    Box::into_raw(Box::new(scene::Scene::new()))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Scene_add_object(self_: *mut scene::Scene, object: *mut CHit) {
    self_.as_mut().unwrap().add_object(object.as_ref().unwrap().clone());
}

#[no_mangle]
pub unsafe extern "C" fn PT_Material_Lambertian_new(red: c_double, green: c_double, blue: c_double, fuzziness: c_float) -> *mut CMaterial {
    Box::into_raw(Box::new(std::rc::Rc::new(lambertian::Lambertian::new(colour::Colour::new(red, green, blue), fuzziness))))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Material_delete(material: *mut CMaterial) {
    if !material.is_null() {
        Box::from_raw(material);
    }
}

#[no_mangle]
pub unsafe extern "C" fn PT_Material_Debugon_new() -> *mut CMaterial {
    Box::into_raw(Box::new(std::rc::Rc::new(debugon::Debugon::new())))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Sphere_new(x: c_double, y: c_double, z: c_double, radius: c_double, material: *mut CMaterial) -> *mut CSphere {
    let material_rc = std::rc::Rc::clone(material.as_ref().unwrap());

    // Return our new sphere.
    Box::into_raw(Box::new(std::rc::Rc::new(sphere::Sphere::new(WorldVec::new(x, y, z), radius, material_rc))))
}
#[no_mangle]
pub unsafe extern "C" fn PT_Sphere_delete(sphere: *mut CSphere) {
    if !sphere.is_null() {
        Box::from_raw(sphere);
    }
}

#[no_mangle]
pub unsafe extern "C" fn PT_Camera_new(
    scene: *mut scene::Scene<'static>,
    origin: *mut CVec3,
    up: *mut CVec3,
    forward: *mut CVec3,
    _x_size: u64,
    _y_size: u64,
) -> *mut CCamera {
    Box::into_raw(Box::new(camera::Camera::new(
        *Box::from_raw(scene),
        *origin,
        *up,
        *forward,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Camera_render(
    self_: *mut CCamera,
    _topleft_x: u64,
    _topleft_y: u64,
    _bottomright_x: u64,
    _bottomright_y: u64,
    fov: f64,
    bounces: u64,
    samples_per_pixel: u64,
) -> *mut CImage {
    Box::into_raw(Box::new(CImage::new(self_.as_ref().unwrap().render(
        800,
        600,
        fov,
        bounces.try_into().unwrap(),
        samples_per_pixel.try_into().unwrap(),
    ))))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Image_get_pixel(self_: *mut CImage, i: u64, j: u64) -> Pixel {
    self_.as_ref().unwrap().at(i, j)
}

#[no_mangle]
pub unsafe extern "C" fn PT_Image_get_height(self_: *mut CImage) -> u64 {
    self_.as_ref().unwrap().0.y_len().try_into().unwrap()
}

#[no_mangle]
pub unsafe extern "C" fn PT_Image_get_width(self_: *mut CImage) -> u64 {
    self_.as_ref().unwrap().0.x_len().try_into().unwrap()
}
