use crate::all::*;

const SPACE_AROUND_COLOR_BOX: f32 = 5.;

pub fn render_center(ui: &mut egui::Ui, c: &mut ColorEditor) {
  egui::ScrollArea::vertical().show(ui, |ui| {
    ui.horizontal_wrapped(|ui| {
      ui.spacing_mut().item_spacing.x = 0.0;
      for (i, token) in c.color_file.tokens.iter().enumerate() {
        match token {
          Token::Text(s) => {
            ui.label(&*s);
          },
          Token::Color(color_kind, color32) => {
            ui.add_space(SPACE_AROUND_COLOR_BOX);
            let box_response = render_color_box(ui, *color32);
            ui.add_space(SPACE_AROUND_COLOR_BOX);

            let s = color_to_text(*color_kind, *color32);
            let link_response = ui.link(&*s);

            if link_response.clicked() || box_response.clicked() {
              c.selected_token = Some(i);
            }
          },
        }
      }
    });
  });
}

fn render_color_box(ui: &mut egui::Ui, color32: Color32) -> Response {
  let (rect, response) = ui.allocate_at_least(egui::Vec2::splat(10.), egui::Sense::click());
  ui.painter().rect(
    rect,
    0., // rounding
    color32,
    egui::Stroke::new(1., Color32::WHITE),
  );
  response
}

pub fn render_left(ui: &mut egui::Ui, c: &mut ColorEditor) {
  if let Some(selected_token) = c.selected_token {
    if let Token::Color(_color_kind, ref mut color32) = c.color_file.tokens[selected_token] {
      render_color_picker(ui, color32);
    }
  }
}

fn render_color_picker(ui: &mut egui::Ui, color32: &mut Color32) {
  use egui::widgets::color_picker::Alpha;
  egui::widgets::color_picker::color_picker_color32(ui, color32, Alpha::Opaque);
}
