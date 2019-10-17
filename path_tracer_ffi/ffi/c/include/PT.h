/**
 * @file PT.h
 *
 * Defines the interface for the C interface into the path tracing library.
 */
#include <stdint.h>

struct Scene;
struct Vec3;
struct Camera;
struct Image;

struct Pixel {
    uint8_t red;
    uint8_t green;
    uint8_t blue;
};

/// Create a 3-dimensional vector.
struct Vec3 *PT_Vec3_new(double x, double y, double z);
void PT_Vec3_delete(struct Vec3 *vec);

/// Create a Scene from a JSON scene specifier.
struct Scene *PT_Scene_from_json(const char *filename);

/// Create a camera for some scene. This function will take ownership of the
/// scene.
struct Camera *PT_Camera_new(struct Scene *scene, struct Vec3 *origin, struct Vec3 *up, struct Vec3 *forward, uint64_t x_size, uint64_t y_size);

/// Render a portion of an image using some camera. If the arguments specifying
/// the portion of the image to render are all left at (uint64_t)-1, the entire image is
/// rendered in one portion.
struct Image *PT_Camera_render(struct Camera *self, uint64_t topleft_x, uint64_t topleft_y, uint64_t bottomright_x, uint64_t bottomright_y, double fov, uint64_t bounces, uint64_t samples_per_pixel);

/// Free the image.
void PT_Image_delete(struct Image *image);

/// Once we have an Image, get the colour of a certain pixel of it. Returns
/// black if out of bounds access is attempted.
struct Pixel PT_Image_get_pixel(struct Image *image, uint64_t i, uint64_t j);

uint64_t PT_Image_get_height(struct Image *image);

uint64_t PT_Image_get_width(struct Image *image);
