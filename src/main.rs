#[macro_use]
extern crate glium;

mod complex_plane;

use glium::{DisplayBuild, Surface, Program, VertexBuffer};
use glium::glutin::{Event, VirtualKeyCode, WindowBuilder};
use glium::glutin::VirtualKeyCode::*;
use glium::backend::glutin_backend::GlutinFacade;

use crate::complex_plane::ComplexPlane;
use glium::backend::glutin_backend::glutin::ElementState;

/// Tutorial from https://aimlesslygoingforward.com/blog/2016/09/27/mandelbrot-using-shaders-rust/
fn main() {
    const WINDOW_SIZE: u32 = 1000;
    let (display, program, vertex_buffer) = gl_setup(WINDOW_SIZE);
    let mut max_iterations: i32 = 100;
    let mut complex_plane = ComplexPlane::default();
    
    // Render loop
    loop {
        let dim = display.get_framebuffer_dimensions();
        let (fitted_plane, pixel_size) = complex_plane.fit_to_screen(dim.0, dim.1);
        
        // These variables are passed
        // into the shader's corresponding
        // `uniform` variables.
        let uniforms = uniform! {
            max_iterations: max_iterations,
            complex_plane: fitted_plane,
            pixel_size: pixel_size,
        };
        
        // animation effect
        //max_iterations = (max_iterations + 1) % 50;
        //max_iterations +=1;
        //if max_iterations > 70 { max_iterations = 4 }
        
        let mut target = display.draw();
        // Draw the vertices
        target.draw(&vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
            &program,
            &uniforms,
            &Default::default()
        ).unwrap();
        target.finish().unwrap();
        
        for event in display.poll_events() {
            match event {
                // the window has been closed by the user:
                Event::Closed => return,
                // Quit on Esc:
                //Event::KeyboardInput(_, _, Some(VirtualKeyCode::Escape)) => return,
                
                // Other keys
                Event::KeyboardInput(ElementState::Pressed, _, Some(key)) => {
                    let new = match_input(key, &complex_plane, max_iterations);
                    complex_plane = new.0;
                    max_iterations = new.1;
                }
                Event::KeyboardInput(state, _, Some(key)) => {
                    eprintln!("{:?} key {:?}", state, key);
                },
                _ => ()
            }
        }
    }
}

fn match_input(key: VirtualKeyCode, plane: &ComplexPlane, max_iterations: i32) -> (ComplexPlane, i32) {
    // manipulate plane
    let new_plane = match key {
        // Zoom in
        Add | F => plane.zoom(0.8),
        // Zoom in very slowly
        Z => plane.zoom(0.98),
        // Zoom out
        Subtract | Space => plane.zoom(1.25),
        // Move
        Left | A => plane.move_left(100.),
        Right | D => plane.move_left(-100.),
        Up | W => plane.move_down(-100.),
        Down | S => plane.move_down(100.),
        _ => plane.clone(),
    };
    // manipulate iterations
    let new_max_iterations = match key {
        // increase iterations
        I | Q => max_iterations * 3 / 2,
        // decrease iterations, but minimum is 2.
        O | E => i32::max(max_iterations * 2 / 3, 2),
        _ => max_iterations,
    };
    
    eprintln!("iterations:{}, {:?}", new_max_iterations, &new_plane);
    (new_plane, new_max_iterations)
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

fn gl_setup(window_size: u32) -> (GlutinFacade, Program, VertexBuffer<Vertex>){
    // Create the window
    let display = WindowBuilder::new()
        .with_title("Mandelbrot Set".to_string())
        .with_dimensions(window_size, window_size)
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
    
    return (display, program, vertex_buffer);
}