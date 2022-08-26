mod render;
mod util;

fn main() -> anyhow::Result<()> {
  use log::info;
  use util::img::Image;
  use util::math::Vec3;
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
  let world = {
    use std::sync::Arc;
    use palette::LinSrgb;
    use render::entity;
    use render::material;
    let mut world = render::World::new();
    let lambert = Arc::new(material::Lambert::new(LinSrgb::new(0.5, 0.5, 0.5)));
    world.push(
      entity::Sphere::new(Vec3::new(0.0, -100.5, 0.0), 100.0, lambert.clone())
    );
    world.push(
      entity::Sphere::new(Vec3::new(0.0, 0.0, 0.0), 0.5, lambert.clone())
    );
    world.push(
      entity::Sphere::new(
        Vec3::new(-1.2, 0.0, 0.0), 0.5,
        Arc::new(material::Metal::new(LinSrgb::new(0.5, 0.0, 0.0), 0.1)))
    );
    world.push(
      entity::Sphere::new(
        Vec3::new(1.2, 0.0, 0.0), -0.49,
        Arc::new(material::Dielectric::new(1.5)))
    );
    world.push(
      entity::Sphere::new(
        Vec3::new(1.2, 0.0, 0.0), 0.5,
        Arc::new(material::Dielectric::new(1.5)))
    );
    world.build()
  };
  let engine = Renderer::new(camera, world);

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
