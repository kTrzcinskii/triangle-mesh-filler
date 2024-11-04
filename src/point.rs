use nalgebra::{Matrix3, Vector3};

#[derive(Clone, Copy)]
pub struct Point {
    before_rotation: PData,
    after_rotation: PData,
}

impl Point {
    pub const ZERO: Self = Self {
        before_rotation: PData::ZERO,
        after_rotation: PData::ZERO,
    };

    pub fn new(before_rotation: PData, after_rotation: PData) -> Self {
        Self {
            before_rotation,
            after_rotation,
        }
    }

    pub fn before_rotation(&self) -> &PData {
        &self.before_rotation
    }

    pub fn after_rotation(&self) -> &PData {
        &self.after_rotation
    }

    pub fn apply_rotation(&mut self, rotation: &Matrix3<f32>) {
        let p: Vector3<f32> = rotation * self.before_rotation().p;
        let pu: Vector3<f32> = rotation * self.before_rotation().pu;
        let pv: Vector3<f32> = rotation * self.before_rotation().pv;
        let n: Vector3<f32> = rotation * self.before_rotation().n;
        self.after_rotation = PData::new(p, pu, pv, n);
    }
}

#[derive(Clone, Copy)]
pub struct PData {
    /// Point
    p: Vector3<f32>,
    /// Tangent vector u
    pu: Vector3<f32>,
    /// Tangent vector v
    pv: Vector3<f32>,
    /// Normal vector
    n: Vector3<f32>,
}

impl PData {
    pub const ZERO: Self = Self {
        p: Vector3::new(0.0, 0.0, 0.0),
        pu: Vector3::new(0.0, 0.0, 0.0),
        pv: Vector3::new(0.0, 0.0, 0.0),
        n: Vector3::new(0.0, 0.0, 0.0),
    };

    pub fn new(p: Vector3<f32>, pu: Vector3<f32>, pv: Vector3<f32>, n: Vector3<f32>) -> Self {
        Self { p, pu, pv, n }
    }

    pub fn p(&self) -> Vector3<f32> {
        self.p
    }
}
