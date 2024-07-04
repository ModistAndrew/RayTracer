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
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let u = u * u * (3.0 - 2.0 * u);
        let v = v * v * (3.0 - 2.0 * v);
        let w = w * w * (3.0 - 2.0 * w);
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [0.0; 8];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x_index = ((i + di) & 255) as usize;
                    let y_index = ((j + dj) & 255) as usize;
                    let z_index = ((k + dk) & 255) as usize;
                    c[(di * 4 + dj * 2 + dk) as usize] = self.random_data
                        [self.perm_x[x_index] ^ self.perm_y[y_index] ^ self.perm_z[z_index]];
                }
            }
        }
        Self::trilinear_interpolation(c, u, v, w)
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

    fn trilinear_interpolation(c: [f64; 8], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i * 4 + j * 2 + k];
                }
            }
        }
        accum
    }
}
