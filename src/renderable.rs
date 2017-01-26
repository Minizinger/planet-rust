use super::glium::{Surface, Vertex, VertexBuffer, Program, Frame, DrawParameters};
use super::glium::index::NoIndices;
use super::glium::backend::Facade;
use vertex::VertexPosition;
use std::marker::PhantomData;

pub struct Renderable<V : Copy + Vertex, N : Copy + Vertex> {
    vertexBuffer : VertexBuffer<V>,
    normalBuffer : Option<VertexBuffer<N>>,
    indexBuffer : NoIndices,
    shader : Program,
}

impl<V : Copy + Vertex, N : Copy + Vertex> Renderable<V, N> {
    pub fn new(display : &Facade, verts : &Vec<V>, normals : Option<&Vec<N>>, vshader : &str, fshader : &str) -> Renderable<V, N>{
        use super::glium::index::PrimitiveType;
        
        let vb = VertexBuffer::new(display, verts).unwrap();
        let mut nb = None;
        match normals {
            Some(n) => nb = Some(VertexBuffer::new(display, n).unwrap()),
            None => nb = None,
        }
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
        use super::glium::vertex::MultiVerticesSource;
        let mut normals = false;
        /*match self.normalBuffer {
            Some(n) => normals = true,
            None => normals = false,
        }*/
        //if normals{
            let buffers = (self.vertexBuffer, self.normalBuffer.unwrap());
            target.draw(buffers, &self.indexBuffer, &self.shader, 
                        &uniform!{view: m_view, projection: m_proj, u_light: light},
                        //&glium::uniforms::EmptyUniforms,
                        &params).unwrap();
        //} else {*/
            /*target.draw(&self.vertexBuffer, &self.indexBuffer, &self.shader, 
                        &uniform!{view: m_view, projection: m_proj, u_light: light},
                        //&glium::uniforms::EmptyUniforms,
                        &params).unwrap();*/
        //}

    }
}