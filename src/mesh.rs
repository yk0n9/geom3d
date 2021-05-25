use crate::Point3;
use grid::Grid;

pub struct TriangleMesh {
    pub vertices: Vec<Point3>,
    /// Indices of points forming triangle list
    pub triangles: Vec<u32>,
}

impl From<Grid<Point3>> for TriangleMesh {
    fn from(grid: Grid<Point3>) -> TriangleMesh {
        let mut triangles = Vec::with_capacity((grid.rows() - 1) * (grid.cols() - 1) * 6);
        for row in 0..grid.rows() - 1 {
            for col in 0..grid.cols() - 1 {
                // first triangle
                triangles.push((row * grid.cols() + col) as u32);
                triangles.push(((row + 1) * grid.cols() + col) as u32);
                triangles.push((row * grid.cols() + col + 1) as u32);
                // second triangle
                triangles.push((row * grid.cols() + col + 1) as u32);
                triangles.push(((row + 1) * grid.cols() + col) as u32);
                triangles.push(((row + 1) * grid.cols() + col + 1) as u32);
            }
        }
        TriangleMesh {
            vertices: grid.flatten().clone(),
            triangles,
        }
    }
}