use path_tracer::camera;
use path_tracer::image;
use path_tracer::vec3;
use std::convert::TryInto;

pub type CCamera = camera::Camera<hit::Sphere>;
pub type CVec3 = vec3::Vec3<f64>;

use std::os::raw::{c_char, c_double};

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
pub unsafe extern "C" fn PT_Scene_from_json(_filename: *const c_char) -> *mut hit::Sphere {
    Box::into_raw(Box::new(hit::Sphere::new(
        vec3::Vec3::new(1.0, 0.0, 0.0),
        0.5,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Camera_new(
    scene: *mut sphere::Sphere,
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
