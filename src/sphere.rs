use crate::{Hit, Hittable, Material, Ray, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material: material.into(),
        }
    }
}

impl Hittable for Sphere {
    // A point P is in the sphere at center C if:
    //   (x - Cx)² + (y - Cy)² + (z - Cz)² = r²
    //
    // This equals the dot product of the vector from C to P:
    //   (P - C) ⋅ (P - C) = r²
    //
    // We want to solve for our ray, which gives a point in P(t) = A + tB:
    //   (P(t) - C) ⋅ (P(t) - C) = r²
    //   (A + tB - C) ⋅ (A + tB - C) = r²
    //   t²B ⋅ B + 2tB ⋅ (A - C) + (A - C) ⋅ (A - C) - r² = 0
    //   -------   ------------   ----------------------
    //   2nd deg   1st degree     constant
    //
    // We have a quadratic equation in terms of the unknown t, which becomes:
    //   at² + bx + c = 0
    //   t = (-b ± √(b² - 4ac)) / 2a
    //
    // If the square root in the solution has a real solution (> 0), we hit the sphere.
    //
    // Note the 2 factor in our first-degree component. This lets us simplify:
    //
    //   (-2h ± √((2h)² - 4ac)) / 2a
    //   (-2h ± √2(h² - ac)) / 2a
    //   (-h ± √(h² - ac)) / a
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let ac = ray.origin - self.center;

        let a = ray.direction.len_sq(); // = B ⋅ B
        let half_b = ray.direction.dot(ac);
        let c = ac.len_sq() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let check_solution = |sol| {
            if t_min < sol && sol < t_max {
                let t = sol;
                let point = ray.at(t);
                let outward_normal = (point - self.center) / self.radius;
                Some(ray.hit(point, outward_normal, Rc::clone(&self.material), t))
            } else {
                None
            }
        };

        let root = discriminant.sqrt();
        check_solution((-half_b - root) / a).or_else(|| check_solution((-half_b + root) / a))
    }
}
