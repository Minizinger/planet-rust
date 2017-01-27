use vertex::*;
use cgmath::{Vector3, InnerSpace};
use std::cell::RefCell;
use std::rc::Rc;

pub enum TriangleContent<T : AnyVertex + Clone, N : AnyVertex + Clone>{
    Triangles([Rc<RefCell<Triangle<T, N>>>; 4]),
    Verticies([T; 3]),
}

pub struct Triangle<T : AnyVertex + Clone, N : AnyVertex + Clone>{
    pub subdivided: bool,
    pub level_of_subdivision: f32,
    pub contents: TriangleContent<T, N>
}

impl<T : AnyVertex + Clone, N : AnyVertex + Clone> Triangle<T, N> {
     pub fn new(pos: [Vector3<f32>; 3], los: f32) -> Triangle<T, N>{
         let verts : [T; 3] = [T::new(pos[0].into()), T::new(pos[1].into()), T::new(pos[2].into())];
         Triangle{
             subdivided: false,
             level_of_subdivision: los,
             contents: TriangleContent::Verticies(verts),
         }
     }

     pub fn subdivide(&mut self){
         let tris : [Rc<RefCell<Triangle<T, N>>>; 4] = [Rc::new(RefCell::new(Triangle::new([Vector3::new(0.,0.,0.); 3], 0.))), 
                                                    Rc::new(RefCell::new(Triangle::new([Vector3::new(0.,0.,0.); 3], 0.))), 
                                                    Rc::new(RefCell::new(Triangle::new([Vector3::new(0.,0.,0.); 3], 0.))), 
                                                    Rc::new(RefCell::new(Triangle::new([Vector3::new(0.,0.,0.); 3], 0.)))];
         let mut triangles_temp : TriangleContent<T, N> = TriangleContent::Triangles(tris);

         match self.contents {
             TriangleContent::Triangles(ref t) => {
                for value in t.iter(){
                    value.borrow_mut().subdivide();
                }
             },
             TriangleContent::Verticies(ref v) => {
                let a = Triangle::<T, N>::middle_point(v[0].get_position(), v[1].get_position());
                let b = Triangle::<T, N>::middle_point(v[1].get_position(), v[2].get_position());
                let c = Triangle::<T, N>::middle_point(v[2].get_position(), v[0].get_position());

                let triangles : [Rc<RefCell<Triangle<T, N>>>; 4] = [Rc::new(RefCell::new(Triangle::new([v[0].get_position(), a, c], self.level_of_subdivision + 1.))),
                    Rc::new(RefCell::new(Triangle::new([v[1].get_position(), b, a], self.level_of_subdivision + 1.))),
                    Rc::new(RefCell::new(Triangle::new([v[2].get_position(), c, b], self.level_of_subdivision + 1.))),
                    Rc::new(RefCell::new(Triangle::new([a, b, c], self.level_of_subdivision + 1.)))];
                triangles_temp = TriangleContent::Triangles(triangles);
            },
         }
         if !self.subdivided
            {self.contents = triangles_temp;
            self.subdivided = true;}
     }

     pub fn middle_point(p1 : Vector3<f32>, p2: Vector3<f32>) -> Vector3<f32>{
         Vector3::<f32>::new((p1.x + p2.x) / 2., (p1.y + p2.y) / 2., (p1.z + p2.z) / 2.).normalize()
     }

     pub fn get_verticies(&self) -> Vec<T>{
         let mut out : Vec<T> = Vec::new();

         match self.contents{
             TriangleContent::Triangles(ref t) =>
             {
                 for value in t.iter(){
                     out.append(&mut value.borrow_mut().get_verticies());
                 }
             },
             TriangleContent::Verticies(ref v) =>
             {
                 out = v.to_vec();
             }
         }

         out
     }

     pub fn get_normal(&self) -> Vec<N>{
         let mut out : Vec<N> = Vec::new();
         match self.contents{
             TriangleContent::Triangles(ref t) =>
             {
                 for value in t.iter(){
                     out.append(&mut value.borrow_mut().get_normal());
                 }
             },
             TriangleContent::Verticies(ref v) =>
             {
                 let v1 = v[2].get_position() - v[0].get_position();
                 let v2 = v[1].get_position() - v[0].get_position();
                 let n = v1.cross(v2);
                 let norm = N::new([n.x, n.y, n.z]);
                 out = vec![norm, norm, norm];
             }
         }
         out
     }
}