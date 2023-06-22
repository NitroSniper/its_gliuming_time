#![allow(unused_variables)]
use glium::{self, glutin, implement_vertex, Surface};

#[path ="assets/models/teapot.rs"]
mod teapot;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    println!("Hello, world!");
    let events_loop = glutin::event_loop::EventLoop::new();
    let cb = glutin::ContextBuilder::new();
    let wb = glutin::window::WindowBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).expect("Display could not be created.");

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 0.0],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
        tex_coords: [0.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
        tex_coords: [1.0, 0.0],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    // (Theory)[https://glium.github.io/glium/book/tuto-02-triangle.html#program]
    // let vertex_buffer =
    //    glium::VertexBuffer::new(&display, &shape).expect("shape to be put into memory");
    // let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).expect("shape to be put into memory");
    let normals = glium::VertexBuffer::new(&display, &teapot::VERTICES).expect("shape to be put into memory");
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).expect("This to be read into memory");

    let vertex_shader_src = r#"
    #version 140
    in vec3 position;
    in vec3 normal;

    uniform mat4 matrix;

    void main() {
        gl_Position = matrix * vec4(position, 1.0); 
    }
    "#;
    // ideas of using the 4th coordinate as scaling. look into the complex shape part of it.
    // [x, y, z, 1.0] 
    // This 1.0 constant is useful for mapping an addition to a constant.

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

    let mut t: f32 = -0.5;

    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("assets/textures/opengl.png")),image::ImageFormat::Png).expect("couldn't open image file").to_rgba8();
    let image_dimenions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimenions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).expect(" to form a texture with it");
    

    events_loop.run(move |ev, _, control_flow| {
        // this is the event loop
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
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_millis(100);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        // Game updates

        let uniforms = glium::uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32]
            ],
        };

        // let uniforms = glium::uniform! {
        //     matrix: [
        //         [1.0, 0.0, 0.0, 0.0],
        //         [0.0, 1.0, 0.0, 0.0],
        //         [0.0, 0.0, 1.0, 0.0],
        //         [ t , 0.0, 0.0, 1.0f32]  *   [x, y, z, 1.0] = [x+t, y, z, 1.0]

        //     ]
        // };

        // Drawing to the Screen
        let mut target = display.draw();
        target.clear_color(0., 0., 1., 1.);
        target
            .draw(
                (&positions, &normals),
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .expect("to draw complex shape");
        target.finish().expect("to be drawn on a screen");
    });
}

