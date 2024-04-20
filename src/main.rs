use eframe::egui;

fn main() -> Result<(), eframe::Error> {
  env_logger::Builder::new().filter_level(log::LevelFilter::Info).init();
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
}

impl Default for ColorEditor {
  fn default() -> Self {
    Self {
    }
  }
}

impl eframe::App for ColorEditor {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |_ui| {
    });
  }
}
