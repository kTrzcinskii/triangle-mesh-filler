use crate::mesh::PosIn2DArr;

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
