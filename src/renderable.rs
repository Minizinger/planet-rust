use super::glium::{Surface, Vertex, VertexBuffer, Program, Frame, DrawParameters};
use super::glium::index::NoIndices;
use super::glium::backend::Facade;
use super::glium::uniforms::{Uniforms, AsUniformValue, UniformsStorage};

pub trait Renderable<'a, T: AsUniformValue, R: Uniforms> {
    fn render(&mut self,
              target: &mut Frame,
              params: &DrawParameters,
              uniforms: &UniformsStorage<'a, T, R>);
}

pub struct RenderableVertexNormal<V: Copy + Vertex, N: Copy + Vertex> {
    vertex_buffer: VertexBuffer<V>,
    normal_buffer: VertexBuffer<N>,
    index_buffer: NoIndices,
    shader: Program,
}
#[allow(dead_code)] //maybe will be useful later
pub struct RenderableVertex<V: Copy + Vertex> {
    vertex_buffer: VertexBuffer<V>,
    index_buffer: NoIndices,
    shader: Program,
}

impl<V: Copy + Vertex, N: Copy + Vertex> RenderableVertexNormal<V, N> {
    pub fn new(display: &Facade,
               verts: &Vec<V>,
               normals: &Vec<N>,
               vshader: &str,
               fshader: &str)
               -> RenderableVertexNormal<V, N> {
        use super::glium::index::PrimitiveType;

        let vb = VertexBuffer::new(display, verts).unwrap();
        let nb = VertexBuffer::new(display, normals).unwrap();
        let ib = NoIndices(PrimitiveType::TrianglesList);

        let program = Program::from_source(display, vshader, fshader, None).unwrap();

        RenderableVertexNormal {
            vertex_buffer: vb,
            normal_buffer: nb,
            index_buffer: ib,
            shader: program,
        }
    }
}
#[allow(dead_code)] //maybe will be useful later
impl<V: Copy + Vertex> RenderableVertex<V> {
    pub fn new(display: &Facade,
               verts: &Vec<V>,
               vshader: &str,
               fshader: &str)
               -> RenderableVertex<V> {
        use super::glium::index::PrimitiveType;

        let vb = VertexBuffer::new(display, verts).unwrap();
        let ib = NoIndices(PrimitiveType::TrianglesList);

        let program = Program::from_source(display, vshader, fshader, None).unwrap();

        RenderableVertex {
            vertex_buffer: vb,
            index_buffer: ib,
            shader: program,
        }
    }
}

impl<'a, T: AsUniformValue, R: Uniforms, V: Vertex, N: Vertex> Renderable<'a, T, R>
    for RenderableVertexNormal<V, N> {
    fn render(&mut self,
              target: &mut Frame,
              params: &DrawParameters,
              uniforms: &UniformsStorage<'a, T, R>) {
        target.draw((&self.vertex_buffer, &self.normal_buffer),
                  &self.index_buffer,
                  &self.shader,
                  uniforms,
                  params)
            .unwrap();
    }
}
impl<'a, T: AsUniformValue, R: Uniforms, V: Vertex> Renderable<'a, T, R> for RenderableVertex<V> {
    fn render(&mut self,
              target: &mut Frame,
              params: &DrawParameters,
              uniforms: &UniformsStorage<'a, T, R>) {
        target.draw(&self.vertex_buffer,
                  &self.index_buffer,
                  &self.shader,
                  uniforms,
                  params)
            .unwrap();
    }
}
