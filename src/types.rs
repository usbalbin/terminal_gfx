
use ::std::ops::{ Add, Sub, Mul };

#[derive(Copy, Clone)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }

    pub fn extend(self, z: T) -> Vector3<T> {
        Vector3{
            x: self.x,
            y: self.y,
            z
        }
    }
}

#[derive(Copy, Clone)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }
}

impl<T> Add<Vector3<T>> for Vector3<T>
    where T: Add<T, Output=T>
{
    type Output = Vector3<T>;
    fn add(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<T> Sub<Vector3<T>> for Vector3<T>
    where T: Sub<T, Output=T>
{
    type Output = Vector3<T>;
    fn sub(self, other: Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

pub fn dot<T>(a: Vector3<T>, b: Vector3<T>) -> T
    where T: Add<T, Output=T> + Mul<T, Output=T>
{
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub trait Vertex: Copy {
    fn default() -> Self;
    fn get_pos(&self) -> Vector3<f32>;
}

#[derive(Copy, Clone)]
pub struct SimpleVertex {
    pub pos: Vector3<f32>,
    pub color: f32,
}

impl Vertex for SimpleVertex {
    fn default() -> Self {
        SimpleVertex {
            pos: Vector3{ x: 0.0, y: 0.0, z: 0.0 },
            color: 0.0
        }
    }
    fn get_pos(&self) -> Vector3<f32> {
        self.pos
    }
}

#[derive(Copy, Clone)]
pub struct Triangle<T: Vertex> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T: Vertex> Triangle<T> {
    pub fn new(indices: &(usize, usize, usize), vertices: &[T]) -> Self {
        Triangle {
            a: vertices[indices.0],
            b: vertices[indices.1],
            c: vertices[indices.2],
        }
    }
}

#[derive(Copy, Clone)]
pub struct Intersection{
    pub depth: f32,
    pub uv: Vector2<f32>,
}