use cgmath::Vector3;

pub trait AnyVertex {
    fn new(point: [f32; 3]) -> Self;
    fn get_position(&self) -> Vector3<f32>;
}

#[derive(Copy, Clone)]
pub struct VertexPosition {
    position: [f32; 3],
}

#[derive(Copy, Clone)]
pub struct Normal{
    pub normal: [f32; 3],
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