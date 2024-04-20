mod all;
mod color_file;

use all::*;

use clap::Parser;
use eframe::egui;
use egui::Color32;

#[derive(Parser)]
struct Args {
  source_path: PathBuf,
}

fn main() -> Result<(), eframe::Error> {
  env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
  let args = Args::parse();

  let _s = std::fs::read_to_string(&args.source_path).unwrap();
    // .with_context(fformat!("read file `{}`.", path.display()))?;

  let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default().with_maximized(true),
    ..Default::default()
  };
  eframe::run_native(
    "Color editor",
    options,
    Box::new(|_| { Box::<ColorEditor>::default() }),
  )
}

struct ColorEditor {
  pub test_color: Color32,
}

impl Default for ColorEditor {
  fn default() -> Self {
    Self {
      test_color: Color32::RED,
    }
  }
}

impl eframe::App for ColorEditor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if ctx.input(|i| i.key_pressed(egui::Key::Q)) {
      ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    egui::CentralPanel::default().show(ctx, |ui| {
      use egui::widgets::color_picker::Alpha;
      egui::widgets::color_picker::color_picker_color32(ui, &mut self.test_color, Alpha::Opaque);
    });
  }
}
