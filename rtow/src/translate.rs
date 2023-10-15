use crate::{Hit, Hittable, Ray, Vec3, AABB};

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Box<dyn Hittable>, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        self.object
            .hit(
                &Ray::new(ray.origin - self.offset, ray.direction, ray.time),
                t_min,
                t_max,
            )
            .map(|mut hit| {
                hit.point = hit.point + self.offset;
                hit
            })
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        self.object.bounding_box(t0, t1).map(|aabb| AABB {
            min: aabb.min + self.offset,
            max: aabb.max + self.offset,
        })
    }
}
