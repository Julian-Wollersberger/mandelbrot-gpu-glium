use glium::uniforms::{UniformValue, AsUniformValue};

/// Representation of the coordinate space of the mandelbrot set.
/// Has upper and lower real and imaginary boundaries.
#[derive(Clone)]
pub struct ComplexPlane {
    min_re: f32,
    min_im: f32,
    max_re: f32,
    max_im: f32,
    // image size in pixel
    width: u32,
    height: u32,
}

impl ComplexPlane {
    
    // --- Math ---
    
    /// Adjust the ratio to 1:1.
    fn pixel_size(&self) -> f32 {
        let pixel_width = (self.max_re - self.min_re) / self.width as f32;
        let pixel_height = (self.max_im - self.min_im) / self.height as f32;
        f32::max(pixel_width, pixel_height)
    }
    
    // --- Events ---
    
    /// Resize the plane to fit into the window without
    /// stretching it in one direction or the other.
    pub fn fit_to_screen(&self, width: u32, height: u32) -> (ComplexPlane, f32) {
        let resized = ComplexPlane {
            height,
            width,
            .. self.clone() // rest is the same.
        };
        let pixel_size = resized.pixel_size();
        
        let center_re = (self.min_re + self.max_re) / 2.0;
        let center_im = (self.min_im + self.max_im) / 2.0;
        // because how pixel_size is adjusted, one axis is adjusted and the other stays the same.
        let radius_re = width as f32 * pixel_size / 2.0;
        let radius_im = height as f32 * pixel_size / 2.0;
        
        let new_plane = ComplexPlane {
            min_re: center_re - radius_re,
            min_im: center_im - radius_im,
            max_re: center_re + radius_re,
            max_im: center_im + radius_im,
            width,
            height,
        };
        
        (new_plane, pixel_size)
    }
    
    //TODO propper calculation
    pub fn zoom(&self, factor: f32) -> ComplexPlane {
        ComplexPlane {
            min_re: self.min_re * factor,
            min_im: self.min_im * factor,
            max_re: self.max_re * factor,
            max_im: self.max_im * factor,
            width: self.width,
            height: self.height,
        }
    }
    
    /// Negative pixels move it to right.
    pub fn move_left(&self, pixels: f32) -> ComplexPlane {
        ComplexPlane {
            min_re: self.min_re - pixels * self.pixel_size(),
            max_re: self.max_re - pixels * self.pixel_size(),
            .. self.clone()
        }
    }
    
    /// Negative pixels move it upwards.
    pub fn move_down(&self, pixels: f32) -> ComplexPlane {
        ComplexPlane {
            min_im: self.min_im - pixels * self.pixel_size(),
            max_im: self.max_im - pixels * self.pixel_size(),
            .. self.clone()
        }
    }
}

impl Default for ComplexPlane {
    fn default() -> Self {
        ComplexPlane {
            min_re: -2.0,
            min_im: -1.2,
            max_re: 0.8,
            max_im: 1.2,
            width: 1000,
            height: 1000,
        }
    }
}

/// So glium can convert it to a vec4
impl AsUniformValue for ComplexPlane {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::Vec4([self.min_re, self.min_im, self.max_re, self.max_im])
    }
}
