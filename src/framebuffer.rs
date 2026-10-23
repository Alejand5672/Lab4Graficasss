use std::path::Path;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pixels: Vec<u8>,
    depth: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::triangle::triangle;

    #[test]
    fn triangles_fill_pixels_and_resolve_depth_in_either_order() {
        let draw = |order: [f32; 2]| {
            let mut fb = Framebuffer::new(8, 8);
            fb.clear([0, 0, 0]);
            for z in order {
                let color = if z == 1.0 { [255, 0, 0] } else { [0, 255, 0] };
                triangle(&mut fb, [1.0, 1.0, z], [7.0, 1.0, z], [1.0, 7.0, z], color);
            }
            fb
        };
        let a = draw([1.0, 2.0]);
        let b = draw([2.0, 1.0]);
        let pixel = (2 * a.width + 2) * 3;
        assert_eq!(&a.pixels[pixel..pixel + 3], &[255, 0, 0]);
        assert_eq!(&a.pixels[0..3], &[0, 0, 0]);
        assert_eq!(a.pixels, b.pixels);
    }
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height * 3],
            depth: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self, color: [u8; 3]) {
        for pixel in self.pixels.chunks_exact_mut(3) {
            pixel.copy_from_slice(&color);
        }
        self.depth.fill(f32::INFINITY);
    }

    pub fn point(&mut self, x: usize, y: usize, z: f32, color: [u8; 3]) {
        if x >= self.width || y >= self.height {
            return;
        }
        let index = y * self.width + x;
        // Menor Z significa mas cerca: oculta las superficies de atras.
        if z < self.depth[index] {
            self.depth[index] = z;
            self.pixels[index * 3..index * 3 + 3].copy_from_slice(&color);
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), image::ImageError> {
        image::save_buffer(
            path,
            &self.pixels,
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
    }
}
