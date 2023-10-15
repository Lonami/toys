use crate::{Hit, Material, Vec3};
use std::rc::Rc;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn hit(
        &self,
        point: Vec3,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
        t: f64,
        u: f64,
        v: f64,
    ) -> Hit {
        // This front face detection could be left up until later, and have the
        // normal always point outwards the surface instead of towards the ray.
        let front_face = self.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Hit {
            point,
            normal,
            material,
            t,
            u,
            v,
            front_face,
        }
    }
}
