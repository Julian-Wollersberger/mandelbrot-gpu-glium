use glium::uniforms::{UniformValue, AsUniformValue};

/// Representation of the coordinate space of the mandelbrot set.
///
pub struct ComplexPlane {
    min_re: f32,
    min_im: f32,
    max_re: f32,
    max_im: f32,
}

impl ComplexPlane {

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
