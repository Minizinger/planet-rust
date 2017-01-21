extern crate cgmath;

use vertex::*;
use cgmath::*;
use triangle::*;
use std::f32;

pub struct Planet<T : AnyVertex + Clone> {
    triangles : Vec<Triangle<T>>,
    pub verticies : Vec<T>,
    pub normals : Vec<Normal>,
}

impl<T : AnyVertex + Clone> Planet<T> {
     pub fn new(subdivisions : i32) -> Self{
         let t : f32 = (1. + (5.0f32).sqrt()) / 2.;

         let mut p0 : Vector3<f32> = Vector3::new(-1., t, 0.);
         let mut p1 : Vector3<f32> = Vector3::new(1., t, 0.);
         let mut p2 : Vector3<f32> = Vector3::new(-1., -t, 0.);
         let mut p3 : Vector3<f32> = Vector3::new(1., -t, 0.);

         let mut p4 : Vector3<f32> = Vector3::new(0., -1., t);
         let mut p5 : Vector3<f32> = Vector3::new(0., 1., t);
         let mut p6 : Vector3<f32> = Vector3::new(0., -1., -t);
         let mut p7 : Vector3<f32> = Vector3::new(0., 1., -t);

         let mut p8 : Vector3<f32> = Vector3::new(t, 0., -1.);
         let mut p9 : Vector3<f32> = Vector3::new(t, 0., 1.);
         let mut p10 : Vector3<f32> = Vector3::new(-t, 0., -1.);
         let mut p11 : Vector3<f32> = Vector3::new(-t, 0., 1.);

         p0 = p0.normalize();
         p1 = p1.normalize();
         p2 = p2.normalize();
         p3 = p3.normalize();
         p4 = p4.normalize();
         p5 = p5.normalize();
         p6 = p6.normalize();
         p7 = p7.normalize();
         p8 = p8.normalize();
         p9 = p9.normalize();
         p10 = p10.normalize();
         p11 = p11.normalize();

         let mut tris : Vec<Triangle<T>> = Vec::new();

         tris.push(Triangle::new([p0, p11, p5], 0.));
         tris.push(Triangle::new([p0, p5, p1], 0.));
         tris.push(Triangle::new([p0, p1, p7], 0.));
         tris.push(Triangle::new([p0, p7, p10], 0.));
         tris.push(Triangle::new([p0, p10, p11], 0.));

         tris.push(Triangle::new([p1, p5, p9], 0.));
         tris.push(Triangle::new([p5, p11, p4], 0.));
         tris.push(Triangle::new([p11, p10, p2], 0.));
         tris.push(Triangle::new([p10, p7, p6], 0.));
         tris.push(Triangle::new([p7, p1, p8], 0.));

         tris.push(Triangle::new([p3, p9, p4], 0.));
         tris.push(Triangle::new([p3, p4, p2], 0.));
         tris.push(Triangle::new([p3, p2, p6], 0.));
         tris.push(Triangle::new([p3, p6, p8], 0.));
         tris.push(Triangle::new([p3, p8, p9], 0.));

         tris.push(Triangle::new([p4, p9, p5], 0.));
         tris.push(Triangle::new([p2, p4, p11], 0.));
         tris.push(Triangle::new([p6, p2, p10], 0.));
         tris.push(Triangle::new([p8, p6, p7], 0.));
         tris.push(Triangle::new([p9, p8, p1], 0.));

         for i in 0..subdivisions{
             for tri in tris.iter_mut(){
                 (*tri).subdivide();
             }
         }

         let mut verts : Vec<T> = Vec::new();
         for tri in tris.iter_mut(){
             verts.append(&mut tri.get_verticies());
         }

         let mut nrm : Vec<Normal> = Vec::new();
         for t in tris.iter_mut(){
             nrm.append(&mut t.get_normal());
         }

        println!("{}", verts.len());

         Planet{
             triangles : tris,
             verticies : verts,
             normals : nrm,
         }
     }
}