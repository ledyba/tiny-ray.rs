use png::Writer;
use std::fs::File;
use std::io::BufWriter;
use palette::{Pixel, Srgb, LinSrgb};

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

  pub fn pixel_mut(&mut self, x: usize, y: usize) -> &mut LinSrgb {
    &mut self.colors[y * self.width + x]
  }

  pub fn pixel(&self, x: usize, y: usize) -> &LinSrgb {
    &self.colors[y * self.width + x]
  }

  pub fn save(&self, path: impl AsRef<std::path::Path> + Sized) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, self.width as u32, self.height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;
    let mut data: Vec<u8> = vec![0; self.width * self.height * 3];
    {
      use rayon::prelude::*;
      data
        .par_chunks_exact_mut(3 * self.width)
        .enumerate()
        .into_par_iter()
        .for_each(|(y, dat)| {
          for x in 0..self.width {
            let srgb = Srgb::from_linear(*self.pixel(x, y)).into_format::<u8>();
            dat[x * 3 + 0] = srgb.red;
            dat[x * 3 + 1] = srgb.green;
            dat[x * 3 + 2] = srgb.blue;
          }
        });

    }
    writer.write_image_data(&data).unwrap();
    Ok(())
  }
}