use nalgebra::Matrix3;

pub struct Rotations;

impl Rotations {
    pub fn create_x_rotation_matrix(beta: f32) -> Matrix3<f32> {
        let beta = beta.to_radians();
        let sin_beta = beta.sin();
        let cos_beta = beta.cos();
        Matrix3::<f32>::new(
            1.0, 0.0, 0.0, 0.0, cos_beta, -sin_beta, 0.0, sin_beta, cos_beta,
        )
    }

    pub fn create_z_rotation_matrix(alfa: f32) -> Matrix3<f32> {
        let alfa = alfa.to_radians();
        let sin_alfa = alfa.sin();
        let cos_alfa = alfa.cos();
        Matrix3::<f32>::new(
            cos_alfa, -sin_alfa, 0.0, sin_alfa, cos_alfa, 0.0, 0.0, 0.0, 1.0,
        )
    }
}
