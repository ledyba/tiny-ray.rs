mod render;
mod img;
mod math;

fn main() -> anyhow::Result<()> {
  use log::info;
  use math::Vec3;
  use img::Image;
  use render::{Camera, Renderer};

  setup_logger(log::LevelFilter::Info)?;
  info!("Initialized.");

  let mut canvas = Image::new(1600, 900);
  let camera = Camera::new(
    Vec3::new(3.0, 2.0, 1.0),
    Vec3::new(0.0, 0.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    45.0,
    canvas.aspect_ratio(),
    0.0,
  );
  let engine = Renderer::new(camera);

  info!("Rendering...");
  engine.render(&mut canvas, 64);
  info!("Done.");

  canvas.save("output.png")?;
  Ok(())
}

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
