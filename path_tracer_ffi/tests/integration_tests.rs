#[cfg(test)]
mod tests {
    use path_tracer_ffi as ffi;
    #[test]
    fn can_render_basic() {
        unsafe {
            let scene = ffi::PT_Scene_new();
            let material = ffi::PT_Material_Debugon_new();
            let sphere = ffi::PT_Sphere_new(0.0, 0.0, 0.0, 1.0, material);
            ffi::PT_Scene_add_object(scene, sphere);

            let camera = ffi::PT_Camera_new(scene, ffi::PT_Vec3_new(0.0, 0.0, -1.0),
                ffi::PT_Vec3_new(0.0, 1.0, 0.0), ffi::PT_Vec3_new(0.0, 0.0, 1.0), 200, 100);
            let image = ffi::PT_Camera_render(camera, 0, 0, 0, 0, 90.0, 10, 10);

            assert_eq!(ffi::PT_Image_get_width(image), 200);
            assert_eq!(ffi::PT_Image_get_height(image), 100);
        }
    }
}
