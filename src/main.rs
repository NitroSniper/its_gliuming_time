#![allow(unused_variables)]
use glium::{self, glutin, implement_vertex, Surface};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn main() {
    println!("Hello, world!");
    let events_loop = glutin::event_loop::EventLoop::new();
    let cb = glutin::ContextBuilder::new();
    let wb = glutin::window::WindowBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).expect("Display could not be created.");

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.4, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    // (Theory)[https://glium.github.io/glium/book/tuto-02-triangle.html#program]
    let vertex_buffer =
        glium::VertexBuffer::new(&display, &shape).expect("shape to be put into memory");
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
    "#;

    // void main is called for each vertex
    // dafluffy potato covers some theory of this.

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .expect("this to read the source");

    events_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0., 0., 1., 1.);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms, // What the fuck is a uniform
                &Default::default(),
            )
            .expect("to draw triangle");
        target.finish().expect("to be drawn on a screen");

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => (),
            },
            _ => (),
        }
    });
}
