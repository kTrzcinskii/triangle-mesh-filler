#[derive(Clone, Copy)]
pub struct PosIn2DArr {
    pub row: usize,
    pub col: usize,
}

pub struct Triangle {
    /// Array of indexes inside list of point in `Mesh`
    vertices: [PosIn2DArr; 3],
}

impl Triangle {
    pub fn new(vertices: [PosIn2DArr; 3]) -> Self {
        Self { vertices }
    }

    pub fn vertices(&self) -> &[PosIn2DArr] {
        &self.vertices
    }
}
