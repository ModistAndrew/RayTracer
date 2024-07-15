use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    matrix: [[f64; 4]; 4],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl Transform {
    pub fn pos(&self, v: Vec3) -> Vec3 {
        let x = self.matrix[0][0] * v.x
            + self.matrix[0][1] * v.y
            + self.matrix[0][2] * v.z
            + self.matrix[0][3];
        let y = self.matrix[1][0] * v.x
            + self.matrix[1][1] * v.y
            + self.matrix[1][2] * v.z
            + self.matrix[1][3];
        let z = self.matrix[2][0] * v.x
            + self.matrix[2][1] * v.y
            + self.matrix[2][2] * v.z
            + self.matrix[2][3];
        Vec3::new(x, y, z)
    }

    pub fn direction(&self, v: Vec3) -> Vec3 {
        let x = self.matrix[0][0] * v.x + self.matrix[0][1] * v.y + self.matrix[0][2] * v.z;
        let y = self.matrix[1][0] * v.x + self.matrix[1][1] * v.y + self.matrix[1][2] * v.z;
        let z = self.matrix[2][0] * v.x + self.matrix[2][1] * v.y + self.matrix[2][2] * v.z;
        Vec3::new(x, y, z)
    }

    pub fn translate(v: Vec3) -> Self {
        let mut ret = Self::default();
        ret.matrix[0][3] = v.x;
        ret.matrix[1][3] = v.y;
        ret.matrix[2][3] = v.z;
        ret
    }

    pub fn scale(v: Vec3) -> Self {
        let mut ret = Self::default();
        ret.matrix[0][0] = v.x;
        ret.matrix[1][1] = v.y;
        ret.matrix[2][2] = v.z;
        ret
    }

    pub fn rotate_x(angle: f64) -> Self {
        let radians = angle.to_radians();
        let mut ret = Self::default();
        let c = radians.cos();
        let s = radians.sin();
        ret.matrix[1][1] = c;
        ret.matrix[1][2] = -s;
        ret.matrix[2][1] = s;
        ret.matrix[2][2] = c;
        ret
    }

    pub fn rotate_y(angle: f64) -> Self {
        let radians = angle.to_radians();
        let mut ret = Self::default();
        let c = radians.cos();
        let s = radians.sin();
        ret.matrix[0][0] = c;
        ret.matrix[0][2] = s;
        ret.matrix[2][0] = -s;
        ret.matrix[2][2] = c;
        ret
    }

    pub fn rotate_z(angle: f64) -> Self {
        let radians = angle.to_radians();
        let mut ret = Self::default();
        let c = radians.cos();
        let s = radians.sin();
        ret.matrix[0][0] = c;
        ret.matrix[0][1] = -s;
        ret.matrix[1][0] = s;
        ret.matrix[1][1] = c;
        ret
    }
}
