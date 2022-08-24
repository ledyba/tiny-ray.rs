mod engine;
mod img;
mod math;

use log::info;
use crate::math::Vec3;

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        chrono::Local::now().format("[%Y-%m-%d %H:%M:%S]"),
        record.target(),
        record.level(),
        message
      ))
    })
    .level(level)
    .chain(std::io::stdout())
    //.chain(fern::log_file("output.log")?)
    .apply()?;
  Ok(())
}

fn main() -> anyhow::Result<()> {
  setup_logger(log::LevelFilter::Info)?;
  info!("Initialized.");

  let mut image = img::Image::new(1600, 900);
  let camera = engine::Camera::new(
    Vec3::new(3.0, 2.0, 1.0),
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    45.0,
    (image.width() as f32, image.height() as f32),
    0.0,
  );
  let engine = engine::Engine::new(camera);

  info!("Rendering...");
  engine.render(&mut image, 64);
  info!("Done.");

  image.save("output.png")?;
  Ok(())
}
