#[macro_use] mod macros;

mod all;
mod color_file;
mod render;

use all::*;

use clap::Parser;

#[derive(Parser)]
struct Args {
  /// Path to the file to edit.
  pub source_path: PathBuf,

  /// Show the edited file in monospace font.
  #[arg(short, long)]
  pub monospace: Option<bool>,

  /// Save the edited file on every change.
  #[arg(short, long)]
  pub autosave: Option<bool>,

  /// UI scale. Use eg 1.5 to make everything bigger.
  #[arg(short, long, default_value_t = 1.0)]
  pub scale: f32,
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
    Box::new(move |creation_context| {
      creation_context.egui_ctx.set_pixels_per_point(args.scale);

      Box::new(ColorEditor {
        color_file,
        selected_token: None,
        old_color32: Color32::BLACK,
        should_save: false,
        monospace: args.monospace.unwrap_or(true),
        autosave: args.autosave.unwrap_or(true),
      })
    }),
  ).unwrap();

  Ok(())
}

#[allow(dead_code)]
pub struct ColorEditor {
  pub color_file: ColorFile,
  pub selected_token: Option<usize>,
  pub old_color32: Color32,
  pub should_save: bool,

  // Options.
  pub monospace: bool,
  pub autosave: bool,
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

    if self.should_save {
      self.color_file.save().unwrap();
      self.should_save = false;
    }
  }
}
