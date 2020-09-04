use crate::{Hit, Hittable, Material, Ray, Vec3, AABB};
use std::f64::consts;
use std::rc::Rc;

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f64,
    time1: f64,
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

    /// This assumes `point` is on the surface of a unit sphere.
    fn get_uv(&self, point: Vec3) -> (f64, f64) {
        // Scaling the spherical coordinates in [0, 1]:
        //   u = ϕ / 2π
        //   v = θ / π
        //
        // Because we're on a unit sphere, we just need to invert:
        //   x = cos(ϕ) cos(θ)
        //   y = sin(ϕ) cos(θ)
        //   z = sin(θ)
        let phi = point.z.atan2(point.x);
        let theta = point.y.asin();
        let u = 1.0 - (phi + consts::PI) / (2.0 * consts::PI);
        let v = (theta - 0.5 * consts::PI) / consts::PI;
        (u, v)
    }
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Box<dyn Material>,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material: material.into(),
        }
    }

    pub fn center(&self, t: f64) -> Vec3 {
        self.center0
            + ((t - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
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
                let (u, v) = self.get_uv(outward_normal);
                Some(ray.hit(point, outward_normal, Rc::clone(&self.material), t, u, v))
            } else {
                None
            }
        };

        let root = discriminant.sqrt();
        check_solution((-half_b - root) / a).or_else(|| check_solution((-half_b + root) / a))
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let offset = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB {
            min: self.center - offset,
            max: self.center + offset,
        })
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        Sphere {
            center: self.center(ray.time),
            radius: self.radius,
            material: Rc::clone(&self.material),
        }
        .hit(ray, t_min, t_max)
    }

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        let offset = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = AABB {
            min: self.center(t0) - offset,
            max: self.center(t0) + offset,
        };
        let box1 = AABB {
            min: self.center(t1) - offset,
            max: self.center(t1) + offset,
        };
        Some(AABB::surrounding(box0, box1))
    }
}
