use crate::{Hit, Hittable, HittableList, Material, Ray, Vec3, XyRect, XzRect, YzRect, AABB};
use std::rc::Rc;

// box and Box are "reserved" names
pub struct Carton {
    min: Vec3,
    max: Vec3,
    sides: HittableList,
}

impl Carton {
    pub fn new(p0: Vec3, p1: Vec3, material: Rc<dyn Material>) -> Self {
        let mut sides = HittableList::new();
        sides.add(Box::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p1.z,
            Rc::clone(&material),
        )));
        sides.add(Box::new(XyRect::new(
            p0.x,
            p1.x,
            p0.y,
            p1.y,
            p0.z,
            Rc::clone(&material),
        )));
        sides.add(Box::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p1.y,
            Rc::clone(&material),
        )));
        sides.add(Box::new(XzRect::new(
            p0.x,
            p1.x,
            p0.z,
            p1.z,
            p0.y,
            Rc::clone(&material),
        )));
        sides.add(Box::new(YzRect::new(
            p0.y,
            p1.y,
            p0.z,
            p1.z,
            p1.x,
            Rc::clone(&material),
        )));
        sides.add(Box::new(YzRect::new(
            p0.y, p1.y, p0.z, p1.z, p0.x, material,
        )));

        Self {
            min: p0,
            max: p1,
            sides,
        }
    }
}

impl Hittable for Carton {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.sides.hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.min,
            max: self.max,
        })
    }
}
