use crate::all::*;

use egui::TextFormat;

pub fn render_left(ui: &mut egui::Ui, c: &ColorEditor) {
  let mut job = egui::text::LayoutJob::default();
  for token in &c.color_file.tokens {
    match token {
      Token::Text(s) => {
        job.append(&s, 0.0, TextFormat::default());
      },
      Token::Color(_color_kind, color32) => {
        let s = format!("{:?}", color32);
        let text_format = TextFormat {
          color: Color32::WHITE,
          ..Default::default()
        };
        job.append(&s, 0.0, text_format);
      },
    }
  }
  ui.label(egui::WidgetText::from(job));
}
