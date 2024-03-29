mod render;
mod util;
mod scene;

fn app() -> clap::Command {
  let scenes = [
    "spheres",
    "many-boxes",
    "many-spheres",
    "lighted-spheres",
    "cornell-box",
    "fog",
  ];

  use clap::{Command, Arg, ArgAction, value_parser};
  Command::new("tiny-ray")
    .bin_name("tiny-ray")
    .author("Kaede Fujisaki <kaede@hexe.net>")
    .arg(Arg::new("verbose")
      .long("verbose")
      .short('v')
      .action(ArgAction::Count)
      .required(false))
    .arg(Arg::new("output")
      .long("output")
      .short('o')
      .value_parser(value_parser!(String))
      .default_value("output.png")
      .action(ArgAction::Set)
      .required(false))
    .arg(Arg::new("animation")
      .long("animation")
      .action(ArgAction::SetTrue)
      .required(false))
    .arg(Arg::new("num-rays")
      .long("num-rays")
      .value_parser(value_parser!(usize))
      .default_value("64")
      .action(ArgAction::Set)
      .required(false))
    .arg(Arg::new("num-reflections")
      .long("num-reflections")
      .value_parser(value_parser!(usize))
      .default_value("64")
      .action(ArgAction::Set)
      .required(false))
    .arg(Arg::new("width")
      .long("width")
      .value_parser(value_parser!(usize))
      .default_value("1600")
      .action(ArgAction::Set)
      .required(false)
      .requires("height"))
    .arg(Arg::new("height")
      .long("height")
      .value_parser(value_parser!(usize))
      .default_value("900")
      .action(ArgAction::Set)
      .required(false)
      .requires("width"))
    .arg(Arg::new("SCENE")
      .required(true)
      .action(ArgAction::Set)
      .value_parser(scenes)
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
  info!("Available cores: {} (Physical core: {})", num_cpus::get(), num_cpus::get_physical());
  let animation = *m.get_one::<bool>("animation").expect("[BUG] No animation");
  let width = *m.get_one::<usize>("width").expect("[BUG] No width");
  let height = *m.get_one::<usize>("height").expect("[BUG] No height");
  let num_rays = *m.get_one::<usize>("num-rays").expect("[BUG] No num-rays");
  let num_reflections = *m.get_one::<usize>("num-reflections").expect("[BUG] No num-reflections");
  let output_path = m.get_one::<String>("output").expect("[BUG] No output");

  let mut canvas = Image::new(width, height);

  let scene =
    match m.get_one::<String>("SCENE").map(|it| it.as_str()) {
      Some("spheres") => scene::spheres(&canvas),
      Some("many-boxes") => scene::many_boxes(&canvas),
      Some("many-spheres") => scene::many_spheres(&canvas),
      Some("lighted-spheres") => scene::lighted_spheres(&canvas),
      Some("cornell-box") => scene::cornell_box(&canvas),
      Some("fog") => scene::fog(&canvas),
      _ => unreachable!(),
    };
  let engine = Renderer::new(scene);

  info!("Rendering...");
  let beg = std::time::Instant::now();
  if animation {
    std::fs::create_dir_all(output_path)?;
    let mut output = Image::new(width, height);
    output.save(std::path::Path::new(output_path).join("0.png"))?;
    for frame in 1..=num_rays {
      canvas.update_by(|x, y, pix| {
        *pix += engine.throw_ray_to(x, width, y, height, num_reflections);
      });
      output.fill_by(|x, y| {
        canvas.pixel_at(x, y) / frame as f32
      });
      let path = std::path::Path::new(output_path).join(format!("{}.png", frame));
      output.save(path)?;
      info!("Ray {}/{} done.", frame, num_rays);
    }
  } else {
    engine.render(&mut canvas, num_rays, num_reflections);
    canvas.save(output_path)?;
  }
  info!("Done in {:.2} sec.", beg.elapsed().as_secs_f32());
  Ok(())
}
