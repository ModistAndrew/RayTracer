use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn dot(self, other: Vec3d) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Vec3d {
        *self / self.length()
    }
}

impl ops::Neg for Vec3d {
    type Output = Vec3d;

    fn neg(self) -> Vec3d {
        Vec3d::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Add for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl ops::Sub for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl ops::Mul<f64> for Vec3d {
    type Output = Vec3d;

    fn mul(self, t: f64) -> Vec3d {
        Vec3d::new(self.x * t, self.y * t, self.z * t)
    }
}

impl ops::Mul<Vec3d> for f64 {
    type Output = Vec3d;
    fn mul(self, v: Vec3d) -> Vec3d {
        Vec3d::new(self * v.x, self * v.y, self * v.z)
    }
}

impl ops::Mul for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: Vec3d) -> Vec3d {
        Vec3d::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl ops::Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, t: f64) -> Vec3d {
        Vec3d::new(self.x / t, self.y / t, self.z / t)
    }
}

#[test]
fn test_vec3d() {
    let v1 = Vec3d::new(1.0, 2.0, 3.0);
    let v2 = Vec3d::new(4.0, 5.0, 6.0);
    assert_eq!(v1.dot(v2), 32.0);
    assert_eq!(v1.length_squared(), 14.0);
}
