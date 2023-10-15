use crate::{rand_u64, Hit, Hittable, HittableList, Ray, AABB};
use std::cmp::Ordering;
use std::rc::Rc;

/// Node in a Bounding Volume Hierachy.
/// The root node can be seen as the tree.
pub struct BvhNode {
    pub left: Rc<dyn Hittable>,
    pub right: Rc<dyn Hittable>,
    pub aabb: AABB,
}

impl BvhNode {
    pub fn new(list: HittableList, time0: f64, time1: f64) -> Self {
        let objects = list.objects.into_iter().map(|x| x.into()).collect();
        Self::from_range(&objects, 0, objects.len(), time0, time1)
    }

    // The efficiency of the hit function depends on how well we subdivide
    // the input list here. For now, a random axis is chosen, the primitives
    // are sorted, and each half goes into its own sub-tree.
    fn from_range(
        objects: &Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Self {
        let left;
        let right;

        let axis = rand_u64(0, 3) as usize;

        let span = end - start;
        if span == 1 {
            left = Rc::clone(&objects[start]);
            right = Rc::clone(&objects[start]);
        } else if span == 2 {
            let ordering = objects[start]
                .compare_box(&objects[start + 1], axis)
                .expect("failed to compare objects");
            if ordering == Ordering::Less {
                left = Rc::clone(&objects[start]);
                right = Rc::clone(&objects[start + 1]);
            } else {
                left = Rc::clone(&objects[start + 1]);
                right = Rc::clone(&objects[start]);
            }
        } else {
            let mut sorted: Vec<_> = objects[start..end].iter().cloned().collect();
            sorted.sort_by(|a, b| a.compare_box(b, axis).expect("failed to compare objects"));

            let mid = sorted.len() / 2;
            left = Rc::new(BvhNode::from_range(&sorted, 0, mid, time0, time1));
            right = Rc::new(BvhNode::from_range(
                &sorted,
                mid,
                sorted.len(),
                time0,
                time1,
            ));
        }

        let box_left = left
            .bounding_box(time0, time1)
            .expect("no bounding box creating BvhNode");
        let box_right = right
            .bounding_box(time0, time1)
            .expect("no bounding box creating BvhNode");

        let aabb = AABB::surrounding(box_left, box_right);
        Self { left, right, aabb }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        if !self.aabb.hit(ray, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(
            ray,
            t_min,
            if let Some(hit) = hit_left.as_ref() {
                hit.t
            } else {
                t_max
            },
        );

        hit_right.or(hit_left)
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.aabb.clone())
    }
}
