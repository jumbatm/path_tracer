use path_tracer::*;

use std::convert::TryInto;

pub type CCamera = camera::Camera<scene::Scene<'static>>;
pub type CSceneBuilder = scene::Scene<'static>;
pub type CScene = std::rc::Rc<CSceneBuilder>;
pub type CVec3 = vec3::Vec3<f64>;
pub type CMaterial = std::rc::Rc<dyn material::Material>;
pub type CHit = std::rc::Rc<dyn hit::Hit + 'static>;

use std::os::raw::{c_double, c_float};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
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
pub unsafe extern "C" fn PT_SceneBuilder_new() -> *mut CSceneBuilder {
    Box::into_raw(Box::new(scene::Scene::new()))
}

#[no_mangle]
pub unsafe extern "C" fn PT_SceneBuilder_add_object(self_: *mut CSceneBuilder, object: *mut CHit) {
    self_
        .as_mut()
        .unwrap()
        .add_object(object.as_ref().unwrap().clone());
}

#[no_mangle]
pub unsafe extern "C" fn PT_SceneBuilder_into_scene(self_: *mut CSceneBuilder) -> *mut CScene {
    Box::into_raw(Box::new(CScene::new(*Box::from_raw(self_))))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Scene_dump(self_: *mut CScene) {
    eprintln!("{:?}", self_.as_ref().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn PT_Scene_delete(self_: *mut CScene) {
    if !self_.is_null() {
        Box::from_raw(self_);
    }
}

#[no_mangle]
pub unsafe extern "C" fn PT_Material_Lambertian_new(
    red: c_double,
    green: c_double,
    blue: c_double,
    fuzziness: c_float,
) -> *mut CMaterial {
    Box::into_raw(Box::new(std::rc::Rc::new(lambertian::Lambertian::new(
        colour::Colour::new(red, green, blue),
        fuzziness,
    ))))
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
pub unsafe extern "C" fn PT_Sphere_new(
    x: c_double,
    y: c_double,
    z: c_double,
    radius: c_double,
    material: *mut CMaterial,
) -> *mut CHit {
    let material_rc = std::rc::Rc::clone(material.as_ref().unwrap());

    // Return our new sphere.
    Box::into_raw(Box::new(std::rc::Rc::new(sphere::Sphere::new(
        WorldVec::new(x, y, z),
        radius,
        material_rc,
    ))))
}
#[no_mangle]
pub unsafe extern "C" fn PT_Hit_delete(hit: *mut CHit) {
    if !hit.is_null() {
        Box::from_raw(hit);
    }
}

#[no_mangle]
pub unsafe extern "C" fn PT_Camera_new(
    scene: *mut CScene,
    origin: *mut CVec3,
    up: *mut CVec3,
    forward: *mut CVec3,
) -> *mut CCamera {
    Box::into_raw(Box::new(camera::Camera::new(
        scene.as_ref().unwrap().clone(),
        *origin,
        *up,
        *forward,
    )))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Camera_render(
    self_: *mut CCamera,
    top_left_x: u64,
    top_left_y: u64,
    bottom_right_x: u64,
    bottom_right_y: u64,
    x_size: u64,
    y_size: u64,
    fov: f64,
    bounces: u64,
    samples_per_pixel: u64,
) -> *mut CImage {
    Box::into_raw(Box::new(CImage::new(
        if top_left_x == 0 && top_left_y == 0 && bottom_right_x == 0 && bottom_right_y == 0 {
            self_.as_ref().unwrap().render(
                x_size.try_into().unwrap(),
                y_size.try_into().unwrap(),
                fov,
                bounces.try_into().unwrap(),
                samples_per_pixel.try_into().unwrap(),
            )
        } else {
            self_
                .as_ref()
                .unwrap()
                .render_region(
                    (
                        top_left_x.try_into().unwrap(),
                        top_left_y.try_into().unwrap(),
                    ),
                    (
                        bottom_right_x.try_into().unwrap(),
                        bottom_right_y.try_into().unwrap(),
                    ),
                    x_size.try_into().unwrap(),
                    y_size.try_into().unwrap(),
                    fov,
                    bounces.try_into().unwrap(),
                    samples_per_pixel.try_into().unwrap(),
                )
                .unwrap()
        },
    )))
}

#[no_mangle]
pub unsafe extern "C" fn PT_Camera_dump(self_: *mut CCamera) {
    dbg!(self_.as_ref().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn PT_Camera_delete(self_: *mut CCamera) {
    if !self_.is_null() {
        Box::from_raw(self_);
    }
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
