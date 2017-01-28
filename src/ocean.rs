use vertex::{VertexPosition, AnyVertex};
use super::glium::backend::Facade;
use super::glium::{Frame, DrawParameters};
use cgmath::*;
use triangle::*;
use renderable::{RenderableVertexNormal, Renderable};
use rand::distributions::Range;
use rand::distributions::IndependentSample;
use rand;

pub struct Ocean<T: AnyVertex, N: AnyVertex> {
    triangles: Vec<Triangle<T, N>>,
    pub verticies: Vec<T>,
    pub normals: Vec<N>,
    pub renderable: RenderableVertexNormal<T, N>,
    scale: [[f32; 4]; 4],
    u_color: [f32; 3],
}

macro_rules! vec3 {
    [$x:expr, $y:expr, $z:expr] => ( Vector3::new($x, $y, $z) )
}

impl<T, N: AnyVertex> Ocean<T, N>
    where T: AnyVertex,
          N: AnyVertex
{
    pub fn new(display: &Facade, subdivisions: i32, size: f32) -> Self {
        let t: f32 = (1. + (5.0f32).sqrt()) / 2.;

        let p0 = vec3![-1., t, 0.].normalize();
        let p1 = vec3![1., t, 0.].normalize();
        let p2 = vec3![-1., -t, 0.].normalize();
        let p3 = vec3![1., -t, 0.].normalize();

        let p4 = vec3![0., -1., t].normalize();
        let p5 = vec3![0., 1., t].normalize();
        let p6 = vec3![0., -1., -t].normalize();
        let p7 = vec3![0., 1., -t].normalize();

        let p8 = vec3![t, 0., -1.].normalize();
        let p9 = vec3![t, 0., 1.].normalize();
        let p10 = vec3![-t, 0., -1.].normalize();
        let p11 = vec3![-t, 0., 1.].normalize();

        let mut tris = vec![Triangle::new([p0, p11, p5], 0.),
                            Triangle::new([p0, p5, p1], 0.),
                            Triangle::new([p0, p1, p7], 0.),
                            Triangle::new([p0, p7, p10], 0.),
                            Triangle::new([p0, p10, p11], 0.),

                            Triangle::new([p1, p5, p9], 0.),
                            Triangle::new([p5, p11, p4], 0.),
                            Triangle::new([p11, p10, p2], 0.),
                            Triangle::new([p10, p7, p6], 0.),
                            Triangle::new([p7, p1, p8], 0.),

                            Triangle::new([p3, p9, p4], 0.),
                            Triangle::new([p3, p4, p2], 0.),
                            Triangle::new([p3, p2, p6], 0.),
                            Triangle::new([p3, p6, p8], 0.),
                            Triangle::new([p3, p8, p9], 0.),

                            Triangle::new([p4, p9, p5], 0.),
                            Triangle::new([p2, p4, p11], 0.),
                            Triangle::new([p6, p2, p10], 0.),
                            Triangle::new([p8, p6, p7], 0.),
                            Triangle::new([p9, p8, p1], 0.)];

        for _ in 0..subdivisions {
            for tri in tris.iter_mut() {
                (*tri).subdivide();
            }
        }

        let mut verts: Vec<T> = Vec::new();
        for tri in tris.iter_mut() {
            verts.append(&mut tri.get_verticies());
        }

        let mut nrm: Vec<N> = Vec::new();
        for t in tris.iter_mut() {
            nrm.append(&mut t.get_normal());
        }
        let vertex_shader = include_str!("../assets/shaders/ocean.glslv");
        let fragment_shader = include_str!("../assets/shaders/ocean.glslf");
        let rnd =
            RenderableVertexNormal::new(display, &verts, &nrm, &vertex_shader, &fragment_shader);

        let scal = [[size, 0.0, 0.0, 0.0],
                    [0.0, size, 0.0, 0.0],
                    [0.0, 0.0, size, 0.0],
                    [0.0, 0.0, 0.0, 1.0]];

        let mut rng = rand::thread_rng();
        let col = Range::new(0.0f32, 1.0f32);

        Ocean {
            triangles: tris,
            verticies: verts,
            normals: nrm,
            renderable: rnd,
            scale: scal,
            u_color: [col.ind_sample(&mut rng), col.ind_sample(&mut rng), col.ind_sample(&mut rng)],
        }
    }

    pub fn draw(&mut self,
                target: &mut Frame,
                params: &DrawParameters,
                m_view: [[f32; 4]; 4],
                m_proj: [[f32; 4]; 4],
                light: [f32; 3],
                time: f32) {
        let uni = uniform!{view : m_view, projection: m_proj, u_light: light, f_time : time, scale: self.scale, u_color : self.u_color};
        self.renderable.render(target, params, &uni);
    }
}
