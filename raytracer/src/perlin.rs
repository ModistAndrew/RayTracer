use crate::vec3::Vec3;
use rand::Rng;
pub struct Perlin {
    random_data: [f64; Perlin::POINT_COUNT],
    perm_x: [usize; Perlin::POINT_COUNT],
    perm_y: [usize; Perlin::POINT_COUNT],
    perm_z: [usize; Perlin::POINT_COUNT],
}

impl Default for Perlin {
    fn default() -> Self {
        let mut random_data = [0.0; Self::POINT_COUNT];
        let mut rng = rand::thread_rng();
        for i in random_data.iter_mut() {
            *i = rng.gen();
        }
        Self {
            random_data,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = (4.0 * p.x) as i32 & 255;
        let j = (4.0 * p.y) as i32 & 255;
        let k = (4.0 * p.z) as i32 & 255;
        self.random_data
            [self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }

    fn generate_perm() -> [usize; Perlin::POINT_COUNT] {
        let mut perm = [0; Self::POINT_COUNT];
        for (i, item) in perm.iter_mut().enumerate() {
            *item = i;
        }
        Self::permute(&mut perm);
        perm
    }

    fn permute(perm: &mut [usize]) {
        let mut rng = rand::thread_rng();
        for i in (1..perm.len()).rev() {
            let target = rng.gen_range(0..i);
            perm.swap(i, target);
        }
    }
}
