use crate::math::{Vec3, Model};

pub struct Cube {
    pub vertices: Vec<Vec3>,
    pub edges: Vec<(usize, usize)>,
    pub triangles: Vec<(usize, usize, usize)>,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            vertices: vec![
                Vec3::new(-1., -1., -1.), Vec3::new(1., -1., -1.),
                Vec3::new(1., 1., -1.), Vec3::new(-1., 1., -1.),
                Vec3::new(-1., -1., 1.), Vec3::new(1., -1., 1.),
                Vec3::new(1., 1., 1.), Vec3::new(-1., 1., 1.),
            ],
            edges: vec![
                (0,1),(1,2),(2,3),(3,0),(4,5),(5,6),(6,7),(7,4),(0,4),(1,5),(2,6),(3,7)
            ],
            triangles: vec![
                // Front face (z=-1)
                (0,1,2), (0,2,3),
                // Back face (z=1)
                (5,4,7), (5,7,6),
                // Top face (y=1)
                (3,2,6), (3,6,7),
                // Bottom face (y=-1)
                (4,5,1), (4,1,0),
                // Right face (x=1)
                (1,5,6), (1,6,2),
                // Left face (x=-1)
                (4,0,3), (4,3,7),
            ],
        }
    }
}

impl Model for Cube {
    fn get_vertices(&self) -> &Vec<Vec3> {
        &self.vertices
    }
    
    fn get_edges(&self) -> &Vec<(usize, usize)> {
        &self.edges
    }
    
    fn get_triangles(&self) -> &Vec<(usize, usize, usize)> {
        &self.triangles
    }
}