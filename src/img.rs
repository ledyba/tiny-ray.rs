use std::fs::File;
use std::io::BufWriter;
use palette::{Srgb, LinSrgb};
use png::SrgbRenderingIntent;

pub struct Image {
  width: usize,
  height: usize,
  colors: Vec<LinSrgb>,
}

impl Image {
  pub fn new(
    width: usize,
    height: usize,
  ) -> Self {
    Self {
      width,
      height,
      colors: vec![LinSrgb::new(0.0, 0.0, 0.0); width * height],
    }
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut LinSrgb {
    &mut self.colors[y * self.width + x]
  }

  pub fn pixel(&self, x: usize, y: usize) -> &LinSrgb {
    &self.colors[y * self.width + x]
  }

  pub fn save(&self, path: impl AsRef<std::path::Path> + Sized) -> std::io::Result<()> {
    let file = File::create(path)?;
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_srgb(SrgbRenderingIntent::Perceptual);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut data: Vec<u8> = vec![0; self.width * self.height * 3];
    {
      use rayon::prelude::*;
      self.colors
        .par_chunks_exact(self.width)
        .zip(data.par_chunks_exact_mut(3 * self.width))
        .for_each(|(color, dat)| {
          for x in 0..self.width {
            let c = Srgb::from_linear(color[x]).into_format::<u8>();
            dat[x * 3 + 0] = c.red;
            dat[x * 3 + 1] = c.green;
            dat[x * 3 + 2] = c.blue;
          }
        });
    }
    writer.write_image_data(&data).unwrap();
    Ok(())
  }

  pub fn fill_from<F>(&mut self, f: F)
  where
    F: Fn(usize, usize) -> LinSrgb,
    F: Sync + Send,
  {
    use rayon::prelude::*;
    self.colors
      .par_chunks_exact_mut(self.width)
      .enumerate()
      .for_each(|(y, pixels)| {
        for x in 0..self.width {
          pixels[x] = f(x, y);
        }
      });
  }

}
