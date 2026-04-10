use crate::math::{Vec3, Model};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

pub struct StlModel {
    pub vertices: Vec<Vec3>,
    pub edges: Vec<(usize, usize)>,
    pub triangles: Vec<(usize, usize, usize)>,
}

impl StlModel {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        
        let mesh = stl_io::read_stl(&mut reader)?;
        
        let mut vertices = Vec::new();
        let mut edges = Vec::new();
        let mut triangles = Vec::new();
        let mut vertex_map = std::collections::HashMap::new();
        
        // Find min/max for centering
        let mut min = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
        let mut max = Vec3::new(f32::MIN, f32::MIN, f32::MIN);
        
        for facet in &mesh.faces {
            for vertex_idx in &facet.vertices {
                let v = mesh.vertices[*vertex_idx];
                min.x = min.x.min(v[0]);
                min.y = min.y.min(v[1]);
                min.z = min.z.min(v[2]);
                max.x = max.x.max(v[0]);
                max.y = max.y.max(v[1]);
                max.z = max.z.max(v[2]);
            }
        }
        
        // Normalize scale to [-1, 1]
        let scale_x = if max.x > min.x { 2.0 / (max.x - min.x) } else { 1.0 };
        let scale_y = if max.y > min.y { 2.0 / (max.y - min.y) } else { 1.0 };
        let scale_z = if max.z > min.z { 2.0 / (max.z - min.z) } else { 1.0 };
        let scale = scale_x.min(scale_y).min(scale_z);
        
        let center_x = (min.x + max.x) / 2.0;
        let center_y = (min.y + max.y) / 2.0;
        let center_z = (min.z + max.z) / 2.0;
        
        // Process triangles
        for facet in &mesh.faces {
            let mut triangle_indices = Vec::new();
            
            for vertex_idx in &facet.vertices {
                let v = mesh.vertices[*vertex_idx];
                let normalized = Vec3::new(
                    (v[0] - center_x) * scale,
                    (v[1] - center_y) * scale,
                    (v[2] - center_z) * scale,
                );
                
                let key = (
                    (normalized.x * 10000.0) as i32,
                    (normalized.y * 10000.0) as i32,
                    (normalized.z * 10000.0) as i32,
                );
                
                let idx = *vertex_map.entry(key).or_insert_with(|| {
                    let id = vertices.len();
                    vertices.push(normalized);
                    id
                });
                triangle_indices.push(idx);
            }
            
            // Create triangle and edges from it
            if triangle_indices.len() == 3 {
                triangles.push((triangle_indices[0], triangle_indices[1], triangle_indices[2]));
                edges.push((triangle_indices[0], triangle_indices[1]));
                edges.push((triangle_indices[1], triangle_indices[2]));
                edges.push((triangle_indices[2], triangle_indices[0]));
            }
        }
        
        Ok(StlModel { vertices, edges, triangles })
    }
}

impl Model for StlModel {
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
