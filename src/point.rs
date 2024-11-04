use nalgebra::Vector3;

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
