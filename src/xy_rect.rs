use crate::{Hit, Hittable, Material, Ray, Vec3, AABB};
use std::rc::Rc;

pub struct XyRect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    material: Rc<dyn Material>,
}

impl XyRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, material: Box<dyn Material>) -> Self {
        Self {
            x0,
            x1,
            y0,
            y1,
            k,
            material: material.into(),
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
            min: Vec3::new(self.x0, self.y0, self.k - 0.0001),
            max: Vec3::new(self.x0, self.y0, self.k + 0.0001),
        })
    }
}
