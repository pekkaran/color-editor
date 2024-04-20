#[macro_use] mod macros;

mod all;
mod color_file;

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
      // source_path: args.source_path,
      color_file,
      test_color: Color32::RED,
    })}),
  ).unwrap();

  Ok(())
}

#[allow(dead_code)]
struct ColorEditor {
  // pub source_path: PathBuf,
  pub color_file: ColorFile,
  pub test_color: Color32,
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
