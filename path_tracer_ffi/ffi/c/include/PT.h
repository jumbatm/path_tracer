/**
 * @file PT.h
 *
 * Defines the interface for the C interface into the path tracing library.
 */
#include <stdint.h>

////////////////////////////////////////////////////////////////////////////////
// Opaque types.
////////////////////////////////////////////////////////////////////////////////

struct Scene;
struct SceneBuilder;
struct Vec3;
struct Camera;
struct Image;
struct Hit;
struct Material;

////////////////////////////////////////////////////////////////////////////////
// The Pixel type. Used to represent the colour of pixels in a render.
////////////////////////////////////////////////////////////////////////////////

struct Pixel {
    uint8_t red;
    uint8_t green;
    uint8_t blue;
};

/// Create a 3-dimensional vector.
struct Vec3 *PT_Vec3_new(double x, double y, double z);
void PT_Vec3_delete(struct Vec3 *vec);

/// Create a Scene.
struct SceneBuilder *PT_SceneBuilder_new();

/// Add some object to a scene. The scene will take ownership of the object.
void PT_SceneBuilder_add_object(struct SceneBuilder *self, struct Hit *object);

/// Convert the SceneBuilder into an immutable Scene. This invalidates the
/// original SceneBuilder reference -- discard it.
struct Scene *PT_SceneBuilder_into_scene(struct SceneBuilder *self);

/// Dump the contents of a scene to stderr. Useful for debugging.
/// TODO: May make more sense for this to return a string, and let the client
/// language perform any kind of printing it needs.
void PT_Scene_dump(struct Scene *self);

/// Free a Scene. This won't invalidate any objects the scene was using (unless
/// they were explicitly freed), nor invalidate any cameras using this scene. If
/// you wish to deallocate a SceneBuilder, / / convert the SceneBuilder into a
/// Scene first using PT_SceneBuilder_into_scene, / then pass it to this function.
void *PT_Scene_delete(struct Scene *self);

////////////////////////////////////////////////////////////////////////////////
// Materials.
////////////////////////////////////////////////////////////////////////////////

/// Destroy a material. This will not invalidate objects using this material -
/// only invalidate using this material in any new objects.
void PT_Material_delete(struct Material *handle);

/// Create a Lambertian material.
struct Material *PT_Material_Lambertian_new(double red, double green, double blue);

/// Create Debugon, a magical material which has no attenution and is coloured
/// by its surface normal.
struct Material *PT_Material_Debugon_new();

////////////////////////////////////////////////////////////////////////////////
// Shapes.
////////////////////////////////////////////////////////////////////////////////

/// Destroy a Hittable object.
struct Hit *PT_Hit_delete(struct Hit *hit);

/// Create a Sphere with a given material.
struct Hit *PT_Sphere_new(double x, double y, double z, double radius, struct Material *material);
/// Destroy a sphere handle. This will not invalidate any scenes using this
/// sphere -- only adding this sphere to anymore scenes.

////////////////////////////////////////////////////////////////////////////////
// Camera.
////////////////////////////////////////////////////////////////////////////////

/// Create a camera for some scene. This function will take ownership of the
/// scene.
struct Camera *PT_Camera_new(struct Scene *scene, struct Vec3 *origin, struct Vec3 *up, struct Vec3 *forward);

/// Dump a Camera's information and the scene information contained within to
/// stderr. Useful for debugging.
void PT_Camera_dump(struct Camera *camera);

/// Render a portion of an image using some camera. If the arguments specifying
/// the portion of the image to render are all left at 0, the entire image is
/// rendered in one portion.
struct Image *PT_Camera_render(struct Camera *self, uint64_t top_left_x, uint64_t top_left_y, uint64_t region_x_size, uint64_t region_y_size, uint64_t total_x_size, uint64_t total_y_size, double fov, uint64_t bounces, uint64_t samples_per_pixel);

void PT_Camera_delete(struct Camera *self);

////////////////////////////////////////////////////////////////////////////////
// Image.
////////////////////////////////////////////////////////////////////////////////

/// Free the image.
void PT_Image_delete(struct Image *image);

/// Get a certain pixel's colour.
struct Pixel PT_Image_get_pixel(struct Image *image, uint64_t i, uint64_t j); 

/// Get the image's height.
uint64_t PT_Image_get_height(struct Image *image); 

/// Get the images width.
uint64_t PT_Image_get_width(struct Image *image);
