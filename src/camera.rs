use crate::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let mut camera = Camera {
            origin: Point3::new(),
            horizontal: Vec3::with_values(viewport_width, 0.0, 0.0),
            vertical: Vec3::with_values(0.0, viewport_height, 0.0),
            lower_left_corner: Point3::new(),
        };

        camera.lower_left_corner = camera.origin
            - camera.horizontal / 2.0
            - camera.vertical / 2.0
            - Vec3::with_values(0.0, 0.0, focal_length);

        camera
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::with_values(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
