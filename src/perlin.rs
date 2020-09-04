use crate::{rand_f64, rand_u64, Vec3};

// Must be a power of 2.
const POINT_COUNT: usize = 256;

pub struct Perlin {
    values: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        assert!(POINT_COUNT.is_power_of_two());

        Self {
            values: (0..POINT_COUNT).map(|_| rand_f64()).collect(),
            perm_x: Perlin::generate_perm(),
            perm_y: Perlin::generate_perm(),
            perm_z: Perlin::generate_perm(),
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        let _u = point.x.fract();
        let _v = point.y.fract();
        let _w = point.z.fract();

        let i = (4.0 * point.x) as usize & (POINT_COUNT - 1);
        let j = (4.0 * point.y) as usize & (POINT_COUNT - 1);
        let k = (4.0 * point.z) as usize & (POINT_COUNT - 1);

        self.values[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
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
