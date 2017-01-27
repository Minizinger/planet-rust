use cgmath::Vector3;
use super::glium::vertex::Vertex;

pub trait AnyVertex : Vertex {
    fn new(point: [f32; 3]) -> Self;
    fn get_position(&self) -> Vector3<f32>;
}

#[derive(Copy, Clone)]
pub struct VertexPosition {
    position: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Normal{
    normal: [f32; 3],
}

implement_vertex!(VertexPosition, position);
implement_vertex!(Normal, normal);

impl AnyVertex for VertexPosition{
    fn new(point: [f32; 3]) -> VertexPosition{
        VertexPosition{
            position: point,
        }
    }

    fn get_position(&self) -> Vector3<f32>{
        Vector3::new(self.position[0], self.position[1], self.position[2])
    }
}
impl AnyVertex for Normal{
    fn new(point: [f32; 3]) -> Normal{
        Normal{
            normal: point,
        }
    }

    fn get_position(&self) -> Vector3<f32>{
        Vector3::new(self.normal[0], self.normal[1], self.normal[2])
    }
}