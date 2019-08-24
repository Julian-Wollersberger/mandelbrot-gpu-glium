#[macro_use]
extern crate glium;

use glium::{DisplayBuild, Surface};
use glium::glutin::{Event, VirtualKeyCode, WindowBuilder};

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

/// Tutorial from https://aimlesslygoingforward.com/blog/2016/09/27/mandelbrot-using-shaders-rust/
fn main() {
    // Create the window
    let display = WindowBuilder::new()
        .with_title("Mandelbrot Set".to_string())
        .with_dimensions(1024, 768)
        .build_glium()
        .unwrap();
    
    // Compile the shaders
    let program = glium::Program::from_source(
        &display,
        include_str!("mandelbrot.glslv"),
        include_str!("mandelbrot.glslf"),
        None).unwrap();
    
    // Render 2 triangles covering the whole screen
    let vertices = [
        // Top-left corner
        Vertex{ position: [-1.0,  1.0] },
        Vertex{ position: [ 1.0,  1.0] },
        Vertex{ position: [-1.0, -1.0] },
        
        // Bottom-right corner
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [ 1.0,  1.0] },
        Vertex { position: [ 1.0, -1.0] },
    ];
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    
    loop {
        let mut target = display.draw();
        // Draw the vertices
        target.draw(&vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program,
            &uniform! {},
            &Default::default()).unwrap();
        target.finish().unwrap();
        
        for event in display.poll_events() {
            match event {
                // the window has been closed by the user:
                Event::Closed => return,
                // Quit on Esc:
                Event::KeyboardInput(_ , _, Some(VirtualKeyCode::Escape)) => return,
                _ => ()
            }
        }
    }
}