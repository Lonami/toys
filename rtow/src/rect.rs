use crate::{Hit, Hittable, Material, Ray, Vec3, AABB};
use std::rc::Rc;

const PAD: f64 = 0.0001;

pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Rc<dyn Material>,
}

pub struct XzRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<dyn Material>,
}

pub struct YzRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<dyn Material>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material,
        }
    }
}

impl XzRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Rc<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl YzRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, material: Rc<dyn Material>) -> Self {
        Self {
            y0,
            y1,
            z0,
            z1,
            k,
            material,
        }
    }
}

impl Hittable for XyRect {
    // To determine if a ray hits a rectangle, it has to first hit the plane.
    //
    // For a ray P(t) = A + tB, its z component and solving for a certain k:
    //   t = (k - Az) / Bz
    //
    // With t, the equations for x and y can be solved:
    //   x = Ax + t·Bx
    //   y = Ay + t·By
    //
    // It's a hit if x0 < x < x1 and y0 < y < y1.
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let y = ray.origin.y + t * ray.direction.y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        Some(ray.hit(
            ray.at(t),
            Vec3::new(0.0, 0.0, 1.0),
            Rc::clone(&self.material),
            t,
            u,
            v,
        ))
    }

    // Because the rect is infinitely thin, it has to be padded to be non-zero in each dimension.
    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.y0, self.k - PAD),
            max: Vec3::new(self.x1, self.y1, self.k + PAD),
        })
    }
}

impl Hittable for XzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin.x + t * ray.direction.x;
        let z = ray.origin.z + t * ray.direction.z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        Some(ray.hit(
            ray.at(t),
            Vec3::new(0.0, 0.0, 1.0),
            Rc::clone(&self.material),
            t,
            u,
            v,
        ))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.x0, self.k - PAD, self.z0),
            max: Vec3::new(self.x1, self.k + PAD, self.z1),
        })
    }
}

impl Hittable for YzRect {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin.y + t * ray.direction.y;
        let z = ray.origin.z + t * ray.direction.z;
        if y < self.y0 || y > self.y1 || z < self.z0 || y > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        Some(ray.hit(
            ray.at(t),
            Vec3::new(0.0, 0.0, 1.0),
            Rc::clone(&self.material),
            t,
            u,
            v,
        ))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: Vec3::new(self.k - PAD, self.y0, self.y0),
            max: Vec3::new(self.k + PAD, self.y1, self.z1),
        })
    }
}
