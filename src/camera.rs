use std::f32::consts::PI;
use cgmath::{perspective, Matrix4, Vector3, Point3, Rad, InnerSpace, EuclideanSpace};
use cgmath::conv::array4x4;

pub struct Camera{
    pub view: [[f32;4];4],
    pub projection: [[f32;4];4],

    position: Vector3<f32>,
    fov: Rad<f32>,
    asp: f32,
    camera_orbit: f32
}

impl Camera {
     pub fn new(pos: Vector3<f32>/*, fov_deg: f32*/, aspect: f32, orbit: f32) -> Camera{
         let norm_pos = pos.normalize_to(orbit);
         let view_martix = Matrix4::<f32>::look_at(Point3::from_vec(norm_pos), Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.));
         Camera{
             view: array4x4(view_martix),
             projection: array4x4(perspective(Rad(PI / 4.), aspect, 0.1, 500.)),
             //projection: Matrix4::identity(),
             //projection:  cgmath::ortho(-1., 1., -1., 1., 0.1, 500.),

             position: norm_pos,
             fov: Rad(PI / 4.),
             asp: aspect,
             camera_orbit: orbit
         }
     }

     pub fn update_matricies(&mut self){
         self.view = array4x4(Matrix4::<f32>::look_at(Point3::from_vec(self.position), Point3::new(0., 0., 0.), Vector3::new(0., 0., 1.)));
         self.projection = array4x4(perspective(self.fov, self.asp, 0.1, 100.));
     }

     pub fn get_position(&self) -> Vector3<f32>{self.position}
     pub fn set_position(&mut self, new_pos: Vector3<f32>){
         self.position = new_pos.normalize_to(self.camera_orbit);
         self.update_matricies();
     }
     pub fn get_orbit(&self) -> f32{self.camera_orbit}
     pub fn set_orbit(&mut self, new_orbit: f32){
         self.position = self.position.normalize_to(new_orbit);
         self.update_matricies();
     }
}