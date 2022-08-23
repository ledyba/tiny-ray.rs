mod engine;
mod img;
mod math;

use log::info;

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
  fern::Dispatch::new()
    .format(|out, message, record| {
      out.finish(format_args!(
        "{}[{}][{}] {}",
        chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
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

  let mut image = img::Image::new(800, 800);
  let mut engine = engine::Engine::new();

  info!("Rendering...");
  engine.render(&mut image);
  info!("Done.");

  image.save("out.png")?;
  Ok(())
}
