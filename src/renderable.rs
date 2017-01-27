use super::glium::{Surface, Vertex, VertexBuffer, Program, Frame, DrawParameters};
use super::glium::index::NoIndices;
use super::glium::backend::Facade;

pub trait Renderable{
    fn render(&mut self, target : &mut Frame, params : DrawParameters, m_view : [[f32;4];4], m_proj : [[f32;4];4], light : [f32;3]);
}

pub struct RenderableVertexNormal<V : Copy + Vertex, N : Copy + Vertex> {
    vertex_buffer : VertexBuffer<V>,
    normal_buffer : VertexBuffer<N>,
    index_buffer : NoIndices,
    shader : Program,
}
pub struct RenderableVertex<V : Copy + Vertex> {
    vertex_buffer : VertexBuffer<V>,
    index_buffer : NoIndices,
    shader : Program,
}

impl<V : Copy + Vertex, N : Copy + Vertex> RenderableVertexNormal<V, N> {
    pub fn new(display : &Facade, verts : &Vec<V>, normals : &Vec<N>, vshader : &str, fshader : &str) -> RenderableVertexNormal<V, N>{
        use super::glium::index::PrimitiveType;
        
        let vb = VertexBuffer::new(display, verts).unwrap();
        let nb = VertexBuffer::new(display, normals).unwrap();
        let ib = NoIndices(PrimitiveType::TrianglesList);

        let program = Program::from_source(display, vshader, fshader, None).unwrap();

        RenderableVertexNormal{
            vertex_buffer: vb,
            normal_buffer: nb,
            index_buffer: ib,
            shader : program,
        }
    }
}
impl<V : Copy + Vertex> RenderableVertex<V> {
    pub fn new(display : &Facade, verts : &Vec<V>, vshader : &str, fshader : &str) -> RenderableVertex<V>{
        use super::glium::index::PrimitiveType;
        
        let vb = VertexBuffer::new(display, verts).unwrap();
        let ib = NoIndices(PrimitiveType::TrianglesList);

        let program = Program::from_source(display, vshader, fshader, None).unwrap();

        RenderableVertex{
            vertex_buffer: vb,
            index_buffer: ib,
            shader : program,
        }
    }
}

impl<V : Vertex, N : Vertex> Renderable for RenderableVertexNormal<V, N> {
    fn render(&mut self, target : &mut Frame, params : DrawParameters, m_view : [[f32;4];4], m_proj : [[f32;4];4], light : [f32;3]){
        target.draw((&self.vertex_buffer, &self.normal_buffer), &self.index_buffer, &self.shader, 
                    &uniform!{view: m_view, projection: m_proj, u_light: light},
                    &params).unwrap();
    }
}
impl<V : Vertex> Renderable for RenderableVertex<V> {
    fn render(&mut self, target : &mut Frame, params : DrawParameters, m_view : [[f32;4];4], m_proj : [[f32;4];4], light : [f32;3]){
        target.draw(&self.vertex_buffer, &self.index_buffer, &self.shader, 
                    &uniform!{view: m_view, projection: m_proj, u_light: light},
                    &params).unwrap();
    }
}