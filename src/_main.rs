#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate cgmath;

pub mod camera;
pub mod triangle;
pub mod vertex;
pub mod planet;

use gfx::traits::FactoryExt;
use gfx::Device;
use cgmath::Vector3;
use vertex::*;
use triangle::Triangle;
use planet::*;

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.];

pub fn main() {
    let qb : Vec<VertexPositionColor> = vec![VertexPositionColor::new([-1., -1., 1.]), VertexPositionColor::new([-1., 1., 1.]), VertexPositionColor::new([1., 1., 1.]), VertexPositionColor::new([1., -1., 1.]) /*<-top*/, VertexPositionColor::new([-1., -1., -1.]), VertexPositionColor::new([-1., 1., -1.]), VertexPositionColor::new([1., 1., -1.]), VertexPositionColor::new([1., -1., -1.])];
    let tr : Vec<VertexPositionColor> = vec![VertexPositionColor::new([-0.5, -1., -0.5]), VertexPositionColor::new([0.5, -1., -0.5]), VertexPositionColor::new([0., 1., 1.]), VertexPositionColor::new([-0.5, -1., 0.5]), VertexPositionColor::new([0.5, -1., 0.5])];
    //let tr : Triangle<VertexPositionColor> = Triangle::new([Vector3::new(-0.5, -0.5, 0.), Vector3::new(0.5, -0.5, 0.), Vector3::new(0., 0.5, 0.)], 0.);
    let pl : Planet<VertexPositionColor> = Planet::new(0);
    let inds : Vec<u16> = vec![0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4];

    let builder = glutin::WindowBuilder::new()
        .with_title("Triangle example".to_string())
        //.with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let pso = factory.create_pipeline_simple(
        include_bytes!("../assets/shaders/shader_150.glslv"),
        include_bytes!("../assets/shaders/shader_150.glslf"),
        pipe::new()
    ).unwrap();

    let size = window.get_inner_size_pixels().unwrap();
    let mut cam = camera::Camera::new(Vector3::new(0., 0., 1.), 0.78, (size.0 as f32 / size.1 as f32), 5.);

    let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(qb.as_slice(), inds.as_slice());
    let data = pipe::Data {
        vbuf: vertex_buffer,
        transform: (cam.projection * cam.view).into(),
        locals: factory.create_constant_buffer(1),
        out_color: main_color,
        out_depth: main_depth
    };

    let mut horisontal_angle = 0.;
    let mut vertical_position = 0.;

    'main: loop {
        // loop over events
        let mut cam_pos = cam.get_position();

        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,

                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Up)) => vertical_position += 0.1,
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Down)) => vertical_position -= 0.1,

                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Right)) => horisontal_angle += 0.1,
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Left)) => horisontal_angle -= 0.1,
                
                _ => {},
            }
        }

        if vertical_position < -1.{
            vertical_position = -1.;
        }else if vertical_position > 1.{
            vertical_position = 1.;}

        cam_pos += Vector3::new(f32::cos(horisontal_angle), vertical_position, f32::sin(horisontal_angle));
        cam.set_position(cam_pos);

        // draw a frame
        let locals = Locals{transform: (cam.projection * cam.view).into()};
        encoder.update_constant_buffer(&data.locals, &locals);

        encoder.clear(&data.out_color, CLEAR_COLOR);
        encoder.clear_depth(&data.out_depth, 1.0);
        encoder.draw(&slice, &pso, &data);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}