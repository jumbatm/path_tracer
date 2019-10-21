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

            let camera = ffi::PT_Camera_new(
                scene,
                /*origin=*/ ffi::PT_Vec3_new(0.0, 0.0, -1.0),
                /*forward=*/ ffi::PT_Vec3_new(0.0, 1.0, 0.0),
                /*up=*/ ffi::PT_Vec3_new(0.0, 0.0, 1.0),
            );
            let image = ffi::PT_Camera_render(camera, 0, 0, 0, 0, 200, 100, 90.0, 10, 10);

            assert_eq!(ffi::PT_Image_get_width(image), 200);
            assert_eq!(ffi::PT_Image_get_height(image), 100);
        }
    }
    #[test]
    fn can_render_regions() {
        unsafe {
            let scene = ffi::PT_Scene_new();
            let material = ffi::PT_Material_Debugon_new();
            let sphere = ffi::PT_Sphere_new(0.0, 0.0, 0.0, 1.0, material);
            ffi::PT_Scene_add_object(scene, sphere);

            let camera = ffi::PT_Camera_new(
                scene,
                /*origin=*/ ffi::PT_Vec3_new(0.0, 0.0, -1.0),
                /*forward=*/ ffi::PT_Vec3_new(0.0, 1.0, 0.0),
                /*up=*/ ffi::PT_Vec3_new(0.0, 0.0, 1.0),
            );
            // Top left.
            let image1 = ffi::PT_Camera_render(camera, 0, 0, 100, 50, 200, 100, 90.0, 10, 10);
            // Top right.
            let image2 = ffi::PT_Camera_render(camera, 100, 0, 100, 50, 200, 100, 90.0, 10, 10);
            // Bottom left.
            let image3 = ffi::PT_Camera_render(camera, 0, 50, 100, 50, 200, 100, 90.0, 10, 10);
            // Bottom right.
            let image4 = ffi::PT_Camera_render(camera, 100, 50, 100, 50, 200, 100, 90.0, 10, 10);

            assert_eq!(ffi::PT_Image_get_width(image1), 100);
            assert_eq!(ffi::PT_Image_get_height(image1), 50);

            assert_eq!(ffi::PT_Image_get_width(image2), 100);
            assert_eq!(ffi::PT_Image_get_height(image2), 50);

            assert_eq!(ffi::PT_Image_get_width(image3), 100);
            assert_eq!(ffi::PT_Image_get_height(image3), 50);

            assert_eq!(ffi::PT_Image_get_width(image4), 100);
            assert_eq!(ffi::PT_Image_get_height(image4), 50);

        }
    }
}
