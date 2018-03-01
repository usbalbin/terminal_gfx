use std::f32::NEG_INFINITY;

extern crate terminal_size;

use terminal_size::{ Width, Height };

pub mod types;

use types::*;

pub struct TerminalGraphics
{
    draw_buffer: Vec<Vec<u8>>,
    depth_buffer: Vec<Vec<f32>>,
    grey_to_utf8: fn(u8) -> char,
}

impl TerminalGraphics {
    pub fn new(width: usize, height: usize, grey_to_utf8: fn(u8) -> char) -> Self {
        TerminalGraphics {
            grey_to_utf8,
            draw_buffer: vec![vec![0; width]; height],
            depth_buffer: vec![vec![NEG_INFINITY; width]; height],
        }
    }

    //TODO rename me
    pub fn example_gfx(width: usize, height: usize) -> TerminalGraphics {
        let grey_to_utf8 = |color: u8|{
            match color {
                0...41 => ' ',
                42...83 => '.',
                84...125 => ':',
                126...167 => '*',
                168...209 => '8',
                210...255 => '#',
                _ => unreachable!()
            }
        };

        TerminalGraphics::new(width, height, grey_to_utf8)
    }

    pub fn clear(&mut self, auto_resize: bool) {
        if auto_resize {
            if let Some((Width(width), Height(height))) = terminal_size::terminal_size() {
                self.resize(width as usize - 1, height as usize - 1);
            }
        }

        for row in self.draw_buffer.iter_mut() {
            for pixel in row.iter_mut() {
                *pixel = 0;
            }
        }

        for row in self.depth_buffer.iter_mut() {
            for depth in row.iter_mut() {
                *depth = NEG_INFINITY;
            }
        }
    }

    pub fn draw<T: Vertex>(&mut self, vertices: &[T], vertex_scratch: &mut [T],
                           indices: &[(usize, usize, usize)],
                           pixel_shader: fn(triangle: &Triangle<T>, uv: Vector2<f32>) -> u8,
                           vertex_shader: fn(vertex: &T) -> T
    ) {

        for (transformed, vertex) in vertex_scratch.iter_mut().zip(vertices) {
            *transformed = vertex_shader(vertex);
        }

        //TODO: this is very naive
        for tri in indices.iter() {
            let triangle = Triangle::new(tri, vertex_scratch);
            let (width, height) = self.get_dimensions();

            for y in 0..height {
                for x in 0..width {

                    if let Some(hit) = self.is_inside(Vector2::new(x, y), &triangle) {
                        let y_index = height - y - 1;
                        if hit.depth >= self.depth_buffer[y_index][x] {
                            self.depth_buffer[y_index][x] = hit.depth;
                            self.draw_buffer[y_index][x] = pixel_shader(&triangle, hit.uv);
                        }
                    }

                }
            }
        }
    }

    pub fn pixel_shader(triangle: &Triangle<SimpleVertex>, uv: Vector2<f32>) -> u8 {
        let u = uv.x;
        let v = uv.y;

        let color = (
            triangle.b.color * u +
                triangle.c.color * v +
                triangle.a.color * (1.0 - u - v)
        ) * 255.0;
        if color < 0.0 { 0 } else if color > 255.0 { 255 } else { color as u8 }
    }

    pub fn vertex_shader(vertex: &SimpleVertex) -> SimpleVertex {
        *vertex
    }

    pub fn flush(&self) {
        let (_, height) = self.get_dimensions();

        print!("{}", "\n".repeat((height / 10).min(5)));
        for row in self.draw_buffer.iter() {
            for pixel in row.iter() {
                print!("{}", (self.grey_to_utf8)(*pixel));
            }
            println!();
        }
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.draw_buffer.resize(height, vec![0; width]);
        for row in self.draw_buffer.iter_mut() {
            row.resize(width, 0);
        }

        self.depth_buffer.resize(height, vec![NEG_INFINITY; width]);
        for row in self.depth_buffer.iter_mut() {
            row.resize(width, NEG_INFINITY);
        }
    }




    // ----------------- Collision stuff below -----------------

    fn get_dimensions(&self) -> (usize, usize) {
        //Width, height
        (self.draw_buffer[0].len(), self.draw_buffer.len())
    }

    /// Map point from (0, 0)..(width, heigth) to (-1, -1, 0)..(1, 1, 0)
    fn map_to_screen(&self, point: Vector2<usize>) -> Vector3<f32> {
        let (width, height) = self.get_dimensions();
        let (width, height) = (width as f32, height as f32);
        Vector3 {
            x: 2.0 * point.x as f32 / width - 1.0,
            y: 2.0 * point.y as f32 / height - 1.0,
            z: 0.0
        }
    }

    // http://blackpawn.com/texts/pointinpoly/
    fn is_inside<T: Vertex>(&self, point: Vector2<usize>, triangle: &Triangle<T>) -> Option<Intersection> {
        let p = self.map_to_screen(point);

        // Compute vectors
        let v0 = triangle.c.get_pos() - triangle.a.get_pos();
        let v1 = triangle.b.get_pos() - triangle.a.get_pos();
        let v2 = p - triangle.a.get_pos();

        // Compute dot products
        let dot00 = dot(v0, v0);
        let dot01 = dot(v0, v1);
        let dot02 = dot(v0, v2);
        let dot11 = dot(v1, v1);
        let dot12 = dot(v1, v2);

        // Compute barycentric coordinates
        let inv_denominator = 1.0 / (dot00 * dot11 - dot01 * dot01);
        let u = (dot11 * dot02 - dot01 * dot12) * inv_denominator;
        let v = (dot00 * dot12 - dot01 * dot02) * inv_denominator;

        // Check if point is in triangle
        if !((u >= 0.0) && (v >= 0.0) && (u + v < 1.0)) {
            return None;
        }

        let depth = triangle.b.get_pos().z * u +
            triangle.c.get_pos().z * v +
            triangle.a.get_pos().z * (1.0 - u - v);

        Some(Intersection{
            depth,
            uv: Vector2::new(u, v)
        })
    }
}