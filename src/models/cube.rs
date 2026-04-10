use crate::math::Vec3;
pub struct Cube { pub vertices: Vec<Vec3>, pub edges: Vec<(usize, usize)> }
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
        }
    }
}