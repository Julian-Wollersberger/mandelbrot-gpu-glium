use glium::uniforms::{UniformValue, AsUniformValue};

/// Representation of the coordinate space of the mandelbrot set.
/// Has upper and lower real and imaginary boundaries.
#[derive(Clone)]
pub struct ComplexPlane {
    min_re: f32,
    min_im: f32,
    max_re: f32,
    max_im: f32,
}

impl ComplexPlane {
    /// Resize the plane to fit into the window without
    /// stretching it in one direction or the other.
    pub fn fit_to_screen(&self, width: u32, height: u32) -> (ComplexPlane, f32) {
        let pixel_width = (self.max_re - self.min_re) / width as f32;
        let pixel_height = (self.max_im - self.min_im) / height as f32;
        
        let pixel_size = f32::max(pixel_width, pixel_height);
        
        //let width_difference;
        
        return (self.clone(), pixel_size);
    }
    
    pub fn zoom(&self, factor: f32) -> ComplexPlane {
        ComplexPlane {
            min_re: self.min_re * factor,
            min_im: self.min_im * factor,
            max_re: self.max_re * factor,
            max_im: self.max_im * factor,
        }
    }
}

impl Default for ComplexPlane {
    fn default() -> Self {
        ComplexPlane {
            min_re: -2.0,
            min_im: -1.2,
            max_re: 0.8,
            max_im: 1.2
        }
    }
}

/// So glium can convert it to a vec4
impl AsUniformValue for ComplexPlane {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec4([self.min_re, self.min_im, self.max_re, self.max_im])
    }
}
