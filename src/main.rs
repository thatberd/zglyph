mod math;
mod renderer;
mod models;

use crate::renderer::Renderer;
use crate::math::Model;
use models::cube::Cube;
use models::stl::StlModel;
use terminal_size::{terminal_size, Height, Width};
use std::{thread, time::Duration, env};
use crossterm::{
    event::{self, Event, KeyCode, MouseEventKind, EnableMouseCapture, DisableMouseCapture},
    execute,
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::stdout;

fn get_terminal_dimensions() -> (usize, usize) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w as usize, h as usize)
    } else {
        (80, 40) // Fallback
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    let result = if args.len() > 1 {
        // Try to load STL file
        match StlModel::from_file(&args[1]) {
            Ok(stl) => {
                eprintln!("✓ Loaded STL: {} vertices, {} triangles", 
                         stl.vertices.len(), stl.triangles.len());
                run_app_with_stl(&stl)
            }
            Err(e) => {
                eprintln!("✗ Failed to load STL: {}", e);
                run_app_with_cube(&Cube::new())
            }
        }
    } else {
        eprintln!("✓ Using default cube: 8 vertices, 12 triangles");
        run_app_with_cube(&Cube::new())
    };
    
    // Cleanup
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    
    result
}

fn run_app_with_cube(model: &Cube) -> Result<(), Box<dyn std::error::Error>> {
    run_app(model.get_vertices(), model.get_triangles())
}

fn run_app_with_stl(model: &StlModel) -> Result<(), Box<dyn std::error::Error>> {
    run_app(model.get_vertices(), model.get_triangles())
}

fn run_app(vertices: &Vec<crate::math::Vec3>, triangles: &Vec<(usize, usize, usize)>) -> Result<(), Box<dyn std::error::Error>> {
    let (w, h) = get_terminal_dimensions();
    let mut renderer = Renderer::new(w, h);
    let mut angle_x: f32 = 0.0;
    let mut angle_y: f32 = 0.0;
    let mut angle_z: f32 = 0.0;
    let mut focal_length: f32 = (h as f32) * 0.8;
    
    let mut last_mouse_x: f32 = 0.0;
    let mut last_mouse_y: f32 = 0.0;
    let mut is_dragging = false;
    
    // Spinning control
    let mut spinning_enabled = true;
    
    // Inertia control
    let mut inertia_enabled = false;
    let mut velocity_x: f32 = 0.0;
    let mut velocity_y: f32 = 0.0;
    
    // Hide cursor and clear
    print!("\x1b[?25l\x1b[2J");

    loop {
        // Handle Resize
        let (current_w, current_h) = get_terminal_dimensions();
        renderer.resize(current_w, current_h);

        renderer.clear();
        
        // Handle events (non-blocking)
        if event::poll(Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(key_event) => {
                    match key_event.code {
                        KeyCode::Esc | KeyCode::Char('q') => break,
                        KeyCode::Char('s') => {
                            spinning_enabled = !spinning_enabled;
                        }
                        KeyCode::Char('m') => {
                            inertia_enabled = !inertia_enabled;
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::Down(_) => {
                            last_mouse_x = mouse_event.column as f32;
                            last_mouse_y = mouse_event.row as f32;
                        }
                        MouseEventKind::Up(_) => {
                            is_dragging = false;
                        }
                        MouseEventKind::Drag(_) => {
                            is_dragging = true;
                            let dx = (mouse_event.column as f32 - last_mouse_x) * 0.02;
                            let dy = (mouse_event.row as f32 - last_mouse_y) * 0.02;
                            
                            angle_y += dx;
                            angle_x += dy;
                            
                            // Update velocity for inertia
                            if inertia_enabled {
                                velocity_x = dy;
                                velocity_y = dx;
                            }
                            
                            last_mouse_x = mouse_event.column as f32;
                            last_mouse_y = mouse_event.row as f32;
                        }
                        MouseEventKind::Moved => {
                            if is_dragging {
                                let dx = (mouse_event.column as f32 - last_mouse_x) * 0.02;
                                let dy = (mouse_event.row as f32 - last_mouse_y) * 0.02;
                                
                                angle_y += dx;
                                angle_x += dy;
                                
                                // Update velocity for inertia
                                if inertia_enabled {
                                    velocity_x = dy;
                                    velocity_y = dx;
                                }
                                
                                last_mouse_x = mouse_event.column as f32;
                                last_mouse_y = mouse_event.row as f32;
                            }
                        }
                        MouseEventKind::ScrollUp => {
                            focal_length *= 1.1; // Zoom in
                        }
                        MouseEventKind::ScrollDown => {
                            focal_length *= 0.9; // Zoom out
                        }
                        _ => {}
                    }
                }
                Event::Resize(cols, rows) => {
                    renderer.resize(cols as usize, rows as usize);
                }
                _ => {}
            }
        }

        // Apply rotation based on mode
        if !is_dragging {
            if spinning_enabled {
                angle_z += 0.02;
            }
            
            // Apply inertia if enabled
            if inertia_enabled {
                angle_x += velocity_x;
                angle_y += velocity_y;
                
                // Apply friction to gradually slow down
                velocity_x *= 0.98;
                velocity_y *= 0.98;
                
                // Stop very small velocities
                if velocity_x.abs() < 0.001 { velocity_x = 0.0; }
                if velocity_y.abs() < 0.001 { velocity_y = 0.0; }
            }
        }

        // Light direction (normalized)
        let light_dir = crate::math::Vec3::new(0.5, 0.7, 0.5).normalize();

        // Render triangles with shading
        for (v0_idx, v1_idx, v2_idx) in triangles {
            if *v0_idx >= vertices.len() || *v1_idx >= vertices.len() || *v2_idx >= vertices.len() {
                continue;
            }
            
            // Get rotated vertices
            let p0 = vertices[*v0_idx].rotate(angle_x, angle_y, angle_z);
            let p1 = vertices[*v1_idx].rotate(angle_x, angle_y, angle_z);
            let p2 = vertices[*v2_idx].rotate(angle_x, angle_y, angle_z);

            // Calculate normal
            let v1 = p1.sub(&p0);
            let v2 = p2.sub(&p0);
            let normal = v1.cross(&v2).normalize();
            
            // Remove shading and backface culling for debugging
            let brightness = 0.8; // Fixed brightness

            let z_offset = 4.0;

            // Project to screen with proper scaling and centering
            let base_scale_x = 15.0;
            let base_scale_y = 8.0;
            let x0 = (p0.x * focal_length / (p0.z + z_offset)) * base_scale_x;
            let y0 = (p0.y * focal_length / (p0.z + z_offset)) * base_scale_y;
            let x1 = (p1.x * focal_length / (p1.z + z_offset)) * base_scale_x;
            let y1 = (p1.y * focal_length / (p1.z + z_offset)) * base_scale_y;
            let x2 = (p2.x * focal_length / (p2.z + z_offset)) * base_scale_x;
            let y2 = (p2.y * focal_length / (p2.z + z_offset)) * base_scale_y;
            
            // Calculate average depth for the triangle
            let z0 = p0.z + z_offset;
            let z1 = p1.z + z_offset;
            let z2 = p2.z + z_offset;
            let avg_depth = (z0 + z1 + z2) / 3.0;

            renderer.draw_triangle((x0, y0), (x1, y1), (x2, y2), brightness, avg_depth);
        }

        renderer.render();
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }
    
    Ok(())
}