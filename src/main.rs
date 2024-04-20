#[macro_use] mod macros;

mod all;
mod color_file;
mod render;

use all::*;

use clap::Parser;

#[derive(Parser)]
struct Args {
  source_path: PathBuf,
}

fn main() -> Result<()> {
  env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
  let args = Args::parse();

  let color_file = ColorFile::new(&args.source_path)?;

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_maximized(true),
    ..Default::default()
  };
  eframe::run_native(
    "Color editor",
    options,
    Box::new(|_creation_context| { Box::new(ColorEditor {
      color_file,
      selected_token: None,
      old_color: Color32::BLACK,
      monospace: true,
    })}),
  ).unwrap();

  Ok(())
}

#[allow(dead_code)]
pub struct ColorEditor {
  pub color_file: ColorFile,
  pub selected_token: Option<usize>,
  pub old_color: Color32,
  pub monospace: bool,
}

impl eframe::App for ColorEditor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
      ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    egui::SidePanel::left("left").default_width(1000.).show(ctx, |ui| {
      render_left(ui, self);
    });

    egui::CentralPanel::default().show(ctx, |ui| {
      render_center(ui, self);
    });
  }
}
