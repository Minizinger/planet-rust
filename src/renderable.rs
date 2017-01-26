use super::glium::{Surface, Vertex, VertexBuffer, Program, Frame, DrawParameters};
use super::glium::index::NoIndices;
use super::glium::backend::Facade;
use vertex::VertexPosition;
use super::glium::vertex::{IntoVerticesSource, MultiVerticesSource};

pub struct Renderable<V : Copy + Vertex, N : Copy + Vertex> {
    vertexBuffer : VertexBuffer<V>,
    normalBuffer : VertexBuffer<N>,
    indexBuffer : NoIndices,
    shader : Program,
}

impl<V : Copy + Vertex, N : Copy + Vertex> Renderable<V, N> {
    pub fn new(display : &Facade, verts : &Vec<V>, normals : &Vec<N>, vshader : &str, fshader : &str) -> Renderable<V, N>{
        use super::glium::index::PrimitiveType;
        
        let vb = VertexBuffer::new(display, verts).unwrap();
        let nb = VertexBuffer::new(display, normals).unwrap();
        let ib = NoIndices(PrimitiveType::TrianglesList);

        let program = Program::from_source(display, vshader, fshader, None).unwrap();

        Renderable{
            vertexBuffer: vb,
            normalBuffer: nb,
            indexBuffer: ib,
            shader : program,
        }
    }

    pub fn draw(&mut self, target : &mut Frame, params : DrawParameters, m_view : [[f32;4];4], m_proj : [[f32;4];4], light : [f32;3]){
        target.draw((&self.vertexBuffer, &self.normalBuffer), &self.indexBuffer, &self.shader, 
                    &uniform!{view: m_view, projection: m_proj, u_light: light},
                    //&glium::uniforms::EmptyUniforms,
                    &params).unwrap();

    }
}