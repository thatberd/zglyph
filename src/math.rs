#[derive(Copy, Clone)]
pub struct Vec3 { pub x: f32, pub y: f32, pub z: f32 }
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    
    pub fn rotate(&self, ax: f32, ay: f32, az: f32) -> Vec3 {
        let mut v = Vec3::new(self.x, self.y, self.z);
        // Y-axis
        let x = v.x * ay.cos() + v.z * ay.sin();
        let z = -v.x * ay.sin() + v.z * ay.cos();
        v.x = x; v.z = z;
        // X-axis
        let y = v.y * ax.cos() - v.z * ax.sin();
        let z = v.y * ax.sin() + v.z * ax.cos();
        v.y = y; v.z = z;
        // Z-axis
        let x = v.x * az.cos() - v.y * az.sin();
        let y = v.x * az.sin() + v.y * az.cos();
        v.x = x; v.y = y;
        v
    }
    
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    
    pub fn normalize(&self) -> Vec3 {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vec3::new(self.x / mag, self.y / mag, self.z / mag)
        } else {
            Vec3::new(self.x, self.y, self.z)
        }
    }
    
    pub fn sub(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

pub trait Model {
    fn get_vertices(&self) -> &Vec<Vec3>;
    fn get_edges(&self) -> &Vec<(usize, usize)>;
    fn get_triangles(&self) -> &Vec<(usize, usize, usize)>;
}