use crate::{
    ray::Ray,
    utils::degrees_to_radians,
    vec3::{Point3, Vec3},
};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        vfow: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta: f64 = degrees_to_radians(vfow);
        let h: f64 = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(*lookfrom - *lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, &w));
        let v = Vec3::cross(&w, &u);

        let mut camera = Camera {
            origin: *lookfrom,
            horizontal: focus_dist * viewport_width * u,
            vertical: focus_dist * viewport_height * v,
            lower_left_corner: Point3::new(),
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
        };

        camera.lower_left_corner =
            camera.origin - camera.horizontal / 2.0 - camera.vertical / 2.0 - focus_dist * w;

        camera
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd: Vec3 = self.lens_radius * Vec3::random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x + self.v * rd.y;

        Ray::with_values(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
