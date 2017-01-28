use vertex::AnyVertex;
use super::glium::backend::Facade;
use super::glium::{Frame, DrawParameters};
use cgmath::*;
use triangle::*;
use std::f32;
use renderable::{RenderableVertexNormal, Renderable};
use rand::distributions::Range;
use rand::distributions::IndependentSample;
use rand::random;
use rand;

pub struct Planet<T: AnyVertex, N: AnyVertex> {
    triangles: Vec<Triangle<T, N>>,
    pub verticies: Vec<T>,
    pub normals: Vec<N>,
    pub renderable: RenderableVertexNormal<T, N>,
    f_seed: f32,
    f_persistance: f32,
    f_lacunarity: f32,
    i_octaves: i32,
    u_color: [f32; 3],
}

impl<T: AnyVertex, N: AnyVertex> Planet<T, N> {
    pub fn new(display: &Facade, subdivisions: i32) -> Self {
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

        let mut verts: Vec<T> = Vec::with_capacity(tris.len());
        for tri in tris.iter_mut() {
            verts.append(&mut tri.get_verticies());
        }

        let mut nrm: Vec<N> = Vec::with_capacity(tris.len());
        for t in tris.iter_mut() {
            nrm.append(&mut t.get_normal());
        }
        let vertex_shader = format!("{}{}",
                                    include_str!("../assets/shaders/noise4d.glsl"),
                                    include_str!("../assets/shaders/shader_150.glslv"));
        let fragment_shader = include_str!("../assets/shaders/shader_150.glslf");
        let rnd =
            RenderableVertexNormal::new(display, &verts, &nrm, &vertex_shader, &fragment_shader);

        let mut rng = rand::thread_rng();
        let random_pers = Range::new(0.0f32, 1.0f32);
        let random_lac = Range::new(1.0f32, 3.0f32);
        let random_step = Range::new(1i32, 10i32);

        Planet {
            triangles: tris,
            verticies: verts,
            normals: nrm,
            renderable: rnd,
            f_seed: random::<f32>(),
            f_persistance: random_pers.ind_sample(&mut rng),
            f_lacunarity: random_lac.ind_sample(&mut rng),
            i_octaves: random_step.ind_sample(&mut rng),
            u_color: [random_pers.ind_sample(&mut rng),
                      random_pers.ind_sample(&mut rng),
                      random_pers.ind_sample(&mut rng)],
        }
    }

    pub fn draw(&mut self,
                target: &mut Frame,
                params: &DrawParameters,
                m_view: [[f32; 4]; 4],
                m_proj: [[f32; 4]; 4],
                light: [f32; 3]) {
        let uni = uniform!{view : m_view, projection: m_proj, u_light: light, f_seed: self.f_seed, f_persistance: self.f_persistance, f_lacunarity: self.f_lacunarity, i_octaves: self.i_octaves, u_color: self.u_color};
        self.renderable.render(target, params, &uni);
    }
}
