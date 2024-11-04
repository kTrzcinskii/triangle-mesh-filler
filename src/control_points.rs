use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::{Context, Error, Result};
use nalgebra::Vector3;

const CONTROL_POINTS_COUNT: usize = 16;
pub const _CONTROL_POINT_ROWS: usize = 4;
pub const CONTROL_POINT_COLS: usize = 4;

pub struct ControlPoints {
    points: [Vector3<f32>; CONTROL_POINTS_COUNT],
}

impl ControlPoints {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path).context("Cannot open file")?;
        let reader = BufReader::new(file);
        let mut points = Vec::with_capacity(CONTROL_POINTS_COUNT);
        for line in reader.lines() {
            let line = line?;
            let coord: Vec<f32> = line
                .split_whitespace()
                .flat_map(|s| s.parse::<f32>())
                .collect();
            if coord.len() == 3 {
                points.push(Vector3::from_vec(coord));
            } else {
                return Err(Error::msg(format!("Invalid coords: {}", line)));
            }
        }
        if points.len() != CONTROL_POINTS_COUNT {
            return Err(Error::msg(format!(
                "Invalid number of points (got {}, expected {})",
                points.len(),
                CONTROL_POINTS_COUNT
            )));
        }

        let points: [Vector3<f32>; CONTROL_POINTS_COUNT] = points
            .try_into()
            .map_err(|_| Error::msg("Cannot parse vec to array"))?;
        Ok(Self { points })
    }

    pub fn at(&self, row: usize, col: usize) -> Vector3<f32> {
        self.points[row * CONTROL_POINT_COLS + col]
    }
}
