use crate::{rand_u64, Vec3};

// Must be a power of 2.
const POINT_COUNT: usize = 256;
const POINT_MASK: usize = POINT_COUNT - 1;

pub struct Perlin {
    values: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        assert!(POINT_COUNT.is_power_of_two());

        Self {
            values: (0..POINT_COUNT).map(|_| Vec3::new_random_unit()).collect(),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        // Note this is not the same as `f64::fract` for negative numbers
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let i = point.x.floor() as isize;
        let j = point.y.floor() as isize;
        let k = point.z.floor() as isize;
        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.values[self.perm_x
                        [(i + di as isize) as usize & POINT_MASK]
                        ^ self.perm_y[(j + dj as isize) as usize & POINT_MASK]
                        ^ self.perm_z[(k + dk as isize) as usize & POINT_MASK]]
                }
            }
        }

        perlin_interp(c, u, v, w)
    }

    fn generate_perm() -> Vec<usize> {
        let mut perm: Vec<_> = (0..POINT_COUNT).collect();

        // Permute
        for i in (1..POINT_COUNT).rev() {
            let target = rand_u64(0, i as u64) as usize;
            perm.swap(i, target);
        }

        perm
    }
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3.0 - 2.0 * u);
    let vv = v * v * (3.0 - 2.0 * v);
    let ww = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        let fi = i as f64;
        for j in 0..2 {
            let fj = j as f64;
            for k in 0..2 {
                let fk = k as f64;

                let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                accum += (fi * uu + (1.0 - fi) * (1.0 - uu))
                    * (fj * vv + (1.0 - fj) * (1.0 - vv))
                    * (fk * ww + (1.0 - fk) * (1.0 - ww))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }

    accum
}
