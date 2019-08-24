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
    const WINDOW_SIZE: u32 = 1000;
    
    // Create the window
    let display = WindowBuilder::new()
        .with_title("Mandelbrot Set".to_string())
        .with_dimensions(WINDOW_SIZE, WINDOW_SIZE)
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
        Vertex { position: [-1.0, 1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [-1.0, -1.0] },
        
        // Bottom-right corner
        Vertex { position: [-1.0, -1.0] },
        Vertex { position: [1.0, 1.0] },
        Vertex { position: [1.0, -1.0] },
    ];
    
    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    
    let mut max_iterations: i32 = 0;
    loop {
        // Parameter for shader
        let uniforms = uniform! {
            maxIterations: max_iterations,
            windowHight: WINDOW_SIZE as f32
        };
        //max_iterations = (max_iterations + 1) % 50;
        max_iterations +=1;
        if max_iterations > 70 { max_iterations = 4 }
        
        let mut target = display.draw();
        // Draw the vertices
        target.draw(&vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program,
            &uniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();
        
        for event in display.poll_events() {
            match event {
                // the window has been closed by the user:
                Event::Closed => return,
                // Quit on Esc:
                Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                _ => ()
            }
        }
    }
}