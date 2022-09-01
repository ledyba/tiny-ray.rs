mod render;
mod util;
mod scene;

fn app() -> clap::App<'static> {
  use clap::{App, Arg, ArgAction};
  App::new("tiny-ray")
    .bin_name("tiny-ray")
    .author("Kaede Fujisaki <kaede@hexe.net>")
    .arg(Arg::new("verbose")
      .long("verbose")
      .short('v')
      .action(ArgAction::Count)
      .takes_value(false))
    .arg(Arg::new("SCENE")
      .possible_values(["spheres", "many-spheres", "lighted-spheres"])
      .required(true)
      .multiple_values(false)
      .index(1))
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

fn main() -> anyhow::Result<()> {
  use log::{info, debug};
  use util::img::Image;
  use render::Renderer;

  let m = app().get_matches();
  let log_level = match m.get_one::<u8>("verbose") {
    None | Some(0) => log::LevelFilter::Info,
    Some(1) => log::LevelFilter::Debug,
    _ => log::LevelFilter::Trace,
  };

  setup_logger(log_level)?;
  debug!("Initialized.");

  let mut canvas = Image::new(1600, 900);

  let scene =
    match m.get_one::<String>("SCENE").map(|it| it.as_str()) {
      Some("spheres") => scene::spheres(&canvas),
      Some("many-spheres") => scene::many_spheres(&canvas),
      Some("lighted-spheres") => scene::lighted_spheres(&canvas),
      _ => unreachable!(),
    };
  let engine = Renderer::new(scene);

  info!("Rendering...");
  let beg = std::time::Instant::now();
  engine.render(&mut canvas, 64);
  info!("Done in {:.2} sec.", beg.elapsed().as_secs_f32());

  canvas.save("output.png")?;
  Ok(())
}
