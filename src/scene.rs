use crate::math::{Vec3, Color};

#[derive(Debug, Clone, Copy)]
pub struct Triangle {
    pub p0: Vec3,
    pub p1: Vec3,
    pub p2: Vec3,
}

impl Triangle {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3) -> Self {
        Self { p0, p1, p2 }
    }

    pub fn normal(&self) -> Vec3 {
        let edge1 = self.p1 - self.p0;
        let edge2 = self.p2 - self.p0;

        edge2.cross(edge1).normalize()
    }
}

pub struct Light {
    pub pos: Vec3,
    pub dir: Vec3,
    pub intensity: Color,
}

pub struct Material {
    pub color: Color,
    pub kd: f64,
    pub ks: f64,
    pub ke: f64,
}
