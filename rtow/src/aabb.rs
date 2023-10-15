use crate::{Ray, Vec3};

#[derive(Clone)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn surrounding(box0: AABB, box1: AABB) -> Self {
        let min = Vec3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let max = Vec3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        AABB { min, max }
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        // Do this for all XYZ components
        for i in 0..3 {
            let inv_dir = 1.0 / ray.direction.component(i);
            let mut t0 = inv_dir * (self.min.component(i) - ray.origin.component(i));
            let mut t1 = inv_dir * (self.max.component(i) - ray.origin.component(i));
            if inv_dir < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            if t1.min(t_max) <= t0.max(t_min) {
                return false;
            }
        }

        true
    }
}
