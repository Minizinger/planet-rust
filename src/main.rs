#[macro_use]
extern crate glium;
extern crate cgmath;

mod vertex;
mod camera;
mod planet;
mod triangle;

fn main() {
    use glium::{DisplayBuild, Surface};
    use vertex::{AnyVertex, VertexPosition};
    use camera::Camera;
    use cgmath::Vector3;
    use planet::Planet;

    let display = glium::glutin::WindowBuilder::new().with_depth_buffer(24).build_glium().unwrap();
    let mut target = display.draw();
    let (width, height) = target.get_dimensions();
    target.finish();

    let mut cam : Camera = Camera::new(Vector3::new(0.0, -5.0, 0.0), width as f32 / height as f32, 5.);
    let planet : Planet<VertexPosition> = Planet::new(3);

    let verts = glium::VertexBuffer::new(&display, &planet.verticies).unwrap();
    let normals = glium::VertexBuffer::new(&display, &planet.normals).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader = format!("{}{}", include_str!("../assets/shaders/noise4d.glsl"), include_str!("../assets/shaders/shader_150.glslv"));
    //let vertex_shader = include_str!("../assets/shaders/shader_150.glslv");

    let program = glium::Program::from_source(&display, &vertex_shader, include_str!("../assets/shaders/shader_150.glslf"), None).unwrap();

    let mut vertical_position : f32 = 0.;
    let mut horisontal_angle : f32 = 0.;

    let light = [-1.0, 0.4, 0.9f32];

    loop {
        let mut target = display.draw();     
        
        target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
        .. Default::default()
        };

        target.draw((&verts, &normals), &indices, &program, 
                    &uniform!{view: cam.view, projection: cam.projection, u_light: light},
                    //&glium::uniforms::EmptyUniforms,
                    &params).unwrap();
        target.finish().unwrap();


        let mut cam_pos = cam.get_position();
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Escape)) |
                glium::glutin::Event::Closed => return,

                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Up)) => vertical_position += 0.1,
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Down)) => vertical_position -= 0.1,

                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Right)) => horisontal_angle += 0.1,
                glium::glutin::Event::KeyboardInput(_, _, Some(glium::glutin::VirtualKeyCode::Left)) => horisontal_angle -= 0.1,

                _ => ()
            }
        }

        if vertical_position < -1.{
            vertical_position = -1.;
        }else if vertical_position > 1.{
            vertical_position = 1.;}

        cam_pos += Vector3::new(f32::cos(horisontal_angle), f32::sin(horisontal_angle), vertical_position);
        cam.set_position(cam_pos);
    }
}