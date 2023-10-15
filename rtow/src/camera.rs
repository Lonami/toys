use crate::{rand_range, Ray, Vec3};

pub struct Camera {
    pub origin: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub t0: f64,
    pub t1: f64,
}

impl Camera {
    /// vfov is the vertical field-of-view in degrees.
    /// vup is the view-up vector, generally the world up.
    /// t0 and t1 are the minimum and maximum time to capture the image.
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        t0: f64,
        t1: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (0.5 * theta).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = vup.cross(w).unit();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - 0.5 * horizontal - 0.5 * vertical - focus_dist * w;

        let lens_radius = 0.5 * aperture;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            w,
            lens_radius,
            t0,
            t1,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::new_random_in_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            rand_range(self.t0, self.t1),
        )
    }
}
