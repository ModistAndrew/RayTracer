use crate::vec3::Vec3;
use rand::Rng;
pub struct Noise {
    random_data: [Vec3; Noise::POINT_COUNT],
    perm_x: [usize; Noise::POINT_COUNT],
    perm_y: [usize; Noise::POINT_COUNT],
    perm_z: [usize; Noise::POINT_COUNT],
}

impl Default for Noise {
    fn default() -> Self {
        let mut random_data = [Vec3::default(); Self::POINT_COUNT];
        for i in random_data.iter_mut() {
            *i = Vec3::random_in_cube().normalize();
        }
        Self {
            random_data,
            perm_x: Self::generate_perm(),
            perm_y: Self::generate_perm(),
            perm_z: Self::generate_perm(),
        }
    }
}

impl Noise {
    const POINT_COUNT: usize = 256;

    pub fn noise(&self, p: Vec3) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();
        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [Vec3::default(); 8];
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

    pub fn turbulence(&self, p: Vec3, depth: i32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p;
        let mut weight = 1.0;
        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }
        accum.abs()
    }

    fn generate_perm() -> [usize; Noise::POINT_COUNT] {
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

    fn trilinear_interpolation(c: [Vec3; 8], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_vec = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i * 4 + j * 2 + k].dot(weight_vec)
                }
            }
        }
        accum
    }
}
