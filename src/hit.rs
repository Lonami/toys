use crate::{Material, Ray, Vec3, AABB};
use std::cmp::Ordering;
use std::rc::Rc;

pub struct Hit {
    pub point: Vec3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;

    fn compare_box(&self, other: &Rc<dyn Hittable>, axis: usize) -> Option<Ordering> {
        let a = self.bounding_box(0.0, 0.0)?;
        let b = other.bounding_box(0.0, 0.0)?;
        a.min.component(axis).partial_cmp(&b.min.component(axis))
    }
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut found_hit = None;
        let mut closest_so_far = t_max;
        for obj in self.objects.iter() {
            if let Some(hit) = obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                found_hit = Some(hit);
            }
        }
        found_hit
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let mut iter = self.objects.iter();

        let first = iter.next()?;
        let mut result = first.bounding_box(t0, t1)?;

        while let Some(obj) = iter.next() {
            result = AABB::surrounding(result, obj.bounding_box(t0, t1)?);
        }

        Some(result)
    }
}
