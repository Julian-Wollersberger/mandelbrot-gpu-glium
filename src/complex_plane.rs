use glium::uniforms::{UniformValue, AsUniformValue};

/// Representation of the coordinate space of the mandelbrot set.
/// Has upper and lower real and imaginary boundaries.
#[derive(Clone, Debug)]
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
    
    /// Computes how big one pixel is in the complex plane.
    /// Adjust the aspect ratio to 1:1.
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
        // math is the same as noop-zoom. pixel_size() does the magic.
        let new_plane = resized.zoom(1.0);
        let pixel_size = new_plane.pixel_size();
        
        (new_plane, pixel_size)
    }
    
    /// Zooms in (`factor < 1.0`) or out (`factor > 1.0`).
    /// The center point remains stationary.
    pub fn zoom(&self, factor: f32) -> ComplexPlane {
        let new_pixel_size = factor * self.pixel_size();
    
        let center_re = (self.min_re + self.max_re) / 2.0;
        let center_im = (self.min_im + self.max_im) / 2.0;
        // because how pixel_size is adjusted, one axis is adjusted and the other stays the same.
        let radius_re = self.width as f32 * new_pixel_size / 2.0;
        let radius_im = self.height as f32 * new_pixel_size / 2.0;
    
        ComplexPlane {
            min_re: center_re - radius_re,
            min_im: center_im - radius_im,
            max_re: center_re + radius_re,
            max_im: center_im + radius_im,
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
