use std::io::{self, Write};

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    buffer: Vec<Option<(f32, f32)>>, // Store (brightness, depth)
    scale: usize, // Higher resolution rendering scale
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        let scale = 2;
        Self {
            width,
            height,
            buffer: vec![None; width * height * scale * scale],
            scale,
        }
    }

    pub fn resize(&mut self, new_width: usize, new_height: usize) {
        if self.width != new_width || self.height != new_height {
            self.width = new_width;
            self.height = new_height;
            self.buffer = vec![None; new_width * new_height * self.scale * self.scale];
            print!("\x1b[2J");
        }
    }
    
    pub fn clear(&mut self) {
        self.buffer.fill(None);
    }

    pub fn draw_pixel_direct(&mut self, x: i32, y: i32, brightness: f32, depth: f32) {
        // Direct pixel drawing without screen space transformation
        let buf_width = self.width * self.scale;
        let buf_height = self.height * self.scale;
        
        // Center the coordinates more precisely
        let screen_x = x + (buf_width as i32 / 2);
        let screen_y = y + (buf_height as i32 / 2);

        if screen_x >= 0 && screen_x < buf_width as i32 &&
            screen_y >= 0 && screen_y < buf_height as i32 {
            let idx = screen_y as usize * buf_width + screen_x as usize;
            if let Some((_existing_brightness, existing_depth)) = self.buffer[idx] {
                if depth < existing_depth {
                    self.buffer[idx] = Some((brightness, depth));
                }
            } else {
                self.buffer[idx] = Some((brightness, depth));
            }
        }
    }

    pub fn draw_pixel_direct_aa(&mut self, x: f32, y: f32, brightness: f32, depth: f32) {
        // Anti-aliased pixel drawing with sub-pixel precision
        let buf_width = self.width * self.scale;
        let buf_height = self.height * self.scale;
        
        // Get the 4 surrounding pixels for anti-aliasing
        let base_x = x.floor() as i32;
        let base_y = y.floor() as i32;
        let frac_x = x - x.floor();
        let frac_y = y - y.floor();
        
        // Calculate coverage for each of the 4 pixels
        let coverage_tl = (1.0 - frac_x) * (1.0 - frac_y);
        let coverage_tr = frac_x * (1.0 - frac_y);
        let coverage_bl = (1.0 - frac_x) * frac_y;
        let coverage_br = frac_x * frac_y;
        
        // Draw each pixel with appropriate coverage
        self.draw_pixel_direct_coverage(base_x, base_y, brightness * coverage_tl, depth);
        self.draw_pixel_direct_coverage(base_x + 1, base_y, brightness * coverage_tr, depth);
        self.draw_pixel_direct_coverage(base_x, base_y + 1, brightness * coverage_bl, depth);
        self.draw_pixel_direct_coverage(base_x + 1, base_y + 1, brightness * coverage_br, depth);
    }
    
    fn draw_pixel_direct_coverage(&mut self, x: i32, y: i32, brightness: f32, depth: f32) {
        // Direct pixel drawing with coverage blending
        let buf_width = self.width * self.scale;
        let buf_height = self.height * self.scale;
        
        // Center the coordinates more precisely
        let screen_x = x + (buf_width as i32 / 2);
        let screen_y = y + (buf_height as i32 / 2);

        if screen_x >= 0 && screen_x < buf_width as i32 &&
            screen_y >= 0 && screen_y < buf_height as i32 {
            let idx = screen_y as usize * buf_width + screen_x as usize;
            if let Some((existing_brightness, existing_depth)) = self.buffer[idx] {
                if depth < existing_depth {
                    // Blend with existing brightness
                    let blended_brightness = existing_brightness + brightness * (1.0 - existing_brightness);
                    self.buffer[idx] = Some((blended_brightness, depth));
                } else if (depth - existing_depth).abs() < 0.001 {
                    // Same depth, blend brightness
                    let blended_brightness = existing_brightness + brightness * (1.0 - existing_brightness);
                    self.buffer[idx] = Some((blended_brightness.min(1.0), depth));
                }
            } else {
                self.buffer[idx] = Some((brightness, depth));
            }
        }
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, brightness: f32, depth: f32) {
        let screen_x = x * self.scale as i32 + (self.width as i32 * self.scale as i32 / 2);
        let screen_y = y * self.scale as i32 + (self.height as i32 * self.scale as i32 / 2);
        let buf_width = self.width * self.scale;
        let buf_height = self.height * self.scale;

        if screen_x >= 0 && screen_x < buf_width as i32 &&
            screen_y >= 0 && screen_y < buf_height as i32 {
            let idx = screen_y as usize * buf_width + screen_x as usize;
            if let Some((_existing_brightness, existing_depth)) = self.buffer[idx] {
                if depth < existing_depth {
                    self.buffer[idx] = Some((brightness, depth));
                }
            } else {
                self.buffer[idx] = Some((brightness, depth));
            }
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, brightness: f32, depth: f32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut curr_x = x0;
        let mut curr_y = y0;

        loop {
            self.draw_pixel(curr_x, curr_y, brightness, depth);
            if curr_x == x1 && curr_y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; curr_x += sx; }
            if e2 <= dx { err += dx; curr_y += sy; }
        }
    }

    pub fn draw_line_direct(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, brightness: f32, depth: f32) {
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut curr_x = x0;
        let mut curr_y = y0;

        loop {
            self.draw_pixel_direct(curr_x, curr_y, brightness, depth);
            if curr_x == x1 && curr_y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; curr_x += sx; }
            if e2 <= dx { err += dx; curr_y += sy; }
        }
    }

    pub fn draw_line_direct_aa(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, brightness: f32, depth: f32) {
        // Anti-aliased line drawing using Xiaolin Wu's algorithm
        let dx = x1 - x0;
        let dy = y1 - y0;
        
        if dx.abs() >= dy.abs() {
            // Draw horizontally
            if x0 > x1 {
                self.draw_line_direct_aa(x1, y1, x0, y0, brightness, depth);
                return;
            }
            
            let gradient = dy / dx;
            let mut x = x0.floor();
            let mut y = y0;
            
            // Draw first endpoint
            self.draw_pixel_direct_aa(x0, y0, brightness * (1.0 - (x0 - x)), depth);
            
            // Draw main line
            for x in x0.floor() as i32..x1.floor() as i32 {
                let alpha = y - y.floor();
                self.draw_pixel_direct_coverage(x as i32, y.floor() as i32, brightness * (1.0 - alpha), depth);
                self.draw_pixel_direct_coverage(x as i32, y.floor() as i32 + 1, brightness * alpha, depth);
                y += gradient;
            }
            
            // Draw last endpoint
            self.draw_pixel_direct_aa(x1, y1, brightness * (x1 - x1.floor()), depth);
        } else {
            // Draw vertically
            if y0 > y1 {
                self.draw_line_direct_aa(x1, y1, x0, y0, brightness, depth);
                return;
            }
            
            let gradient = dx / dy;
            let mut y = y0.floor();
            let mut x = x0;
            
            // Draw first endpoint
            self.draw_pixel_direct_aa(x0, y0, brightness * (1.0 - (y0 - y)), depth);
            
            // Draw main line
            for y in y0.floor() as i32..y1.floor() as i32 {
                let alpha = x - x.floor();
                self.draw_pixel_direct_coverage(x.floor() as i32, y as i32, brightness * (1.0 - alpha), depth);
                self.draw_pixel_direct_coverage(x.floor() as i32 + 1, y as i32, brightness * alpha, depth);
                x += gradient;
            }
            
            // Draw last endpoint
            self.draw_pixel_direct_aa(x1, y1, brightness * (y1 - y1.floor()), depth);
        }
    }
    
    pub fn draw_triangle(&mut self, v0: (f32, f32), v1: (f32, f32), v2: (f32, f32), brightness: f32, depth: f32) {
        // Use anti-aliased wireframe for smoother lines
        self.draw_line_direct_aa(v0.0, v0.1, v1.0, v1.1, brightness, depth);
        self.draw_line_direct_aa(v1.0, v1.1, v2.0, v2.1, brightness, depth);
        self.draw_line_direct_aa(v2.0, v2.1, v0.0, v0.1, brightness, depth);
        
        /*
        // Find bounding box with some padding
        let min_x = v0.0.floor().min(v1.0.floor()).min(v2.0.floor()) as i32 - 1;
        let max_x = v0.0.ceil().max(v1.0.ceil()).max(v2.0.ceil()) as i32 + 1;
        let min_y = v0.1.floor().min(v1.1.floor()).min(v2.1.floor()) as i32 - 1;
        let max_y = v0.1.ceil().max(v1.1.ceil()).max(v2.1.ceil()) as i32 + 1;
        
        // Rasterize filled triangle
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.point_in_triangle((x as f32, y as f32), v0, v1, v2) {
                    self.draw_pixel_direct(x, y, brightness, depth);
                }
            }
        }
        */
    }
    
    fn point_in_triangle(&self, p: (f32, f32), a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> bool {
        let (px, py) = p;
        let (ax, ay) = a;
        let (bx, by) = b;
        let (cx, cy) = c;
        
        let v0x = cx - ax;
        let v0y = cy - ay;
        let v1x = bx - ax;
        let v1y = by - ay;
        let v2x = px - ax;
        let v2y = py - ay;
        
        let dot00 = v0x * v0x + v0y * v0y;
        let dot01 = v0x * v1x + v0y * v1y;
        let dot02 = v0x * v2x + v0y * v2y;
        let dot11 = v1x * v1x + v1y * v1y;
        let dot12 = v1x * v2x + v1y * v2y;
        
        let denom = dot00 * dot11 - dot01 * dot01;
        if denom.abs() < 0.0001 {
            return false;
        }
        
        let inv_denom = 1.0 / denom;
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
        
        (u >= -0.01) && (v >= -0.01) && (u + v <= 1.01)
    }

    pub fn render(&self) {
        let mut output = String::with_capacity(self.width * self.height + 100);
        output.push_str("\x1b[H");
        let buf_width = self.width * self.scale;
        
        for y in 0..self.height {
            for x in 0..self.width {
                // Get 2x2 quad of pixels and map to Unicode block character
                let get_brightness = |px: usize, py: usize| -> f32 {
                    if py < self.height * self.scale && px < buf_width {
                        match self.buffer[py * buf_width + px] {
                            Some((brightness, _depth)) => brightness,
                            None => 0.0,
                        }
                    } else {
                        0.0
                    }
                };
                
                let tl = get_brightness(x * self.scale, y * self.scale);
                let tr = get_brightness(x * self.scale + 1, y * self.scale);
                let bl = get_brightness(x * self.scale, y * self.scale + 1);
                let br = get_brightness(x * self.scale + 1, y * self.scale + 1);
                
                // Average brightness across the 2x2 block
                let avg = (tl + tr + bl + br) / 4.0;
                
                // Clamp brightness to [0, 1] and map to shading
                let brightness = avg.max(0.0).min(1.0);
                
                // Map brightness to shade character (0-4)
                let shades = [' ', '░', '▒', '▓', '█'];
                let shade_idx = (brightness * 4.9) as usize;
                let ch = shades[shade_idx.min(4)];
                
                output.push(ch);
            }
            if y < self.height - 1 { output.push('\n'); }
        }
        print!("{}", output);
        io::stdout().flush().unwrap();
    }
}