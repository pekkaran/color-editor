use crate::all::*;

const SPACE_AROUND_COLOR_BOX: f32 = 5.;
const COLOR_COMPARISON_BOX_SIZE: f32 = 100.;

pub fn render_center(ui: &mut egui::Ui, c: &mut ColorEditor) {
  egui::ScrollArea::vertical().show(ui, |ui| {
    if c.color_file.color_count == 0 {
      ui.label(egui::RichText::new("No colors found in the file!").size(20.0).underline().strong());
    }

    ui.horizontal_wrapped(|ui| {
      ui.spacing_mut().item_spacing.x = 0.0;
      for (i, token) in c.color_file.tokens.iter().enumerate() {
        match token {
          Token::Text(s) => {
            ui.label(rich_text(s, c.monospace));
          },
          Token::Color(color_kind, color32) => {
            ui.add_space(SPACE_AROUND_COLOR_BOX);
            let box_response = render_color_box(ui, *color32, 10.);
            ui.add_space(SPACE_AROUND_COLOR_BOX);

            let s = color_to_text(color_kind, *color32);
            let link_response = ui.link(rich_text(&s, c.monospace));

            if link_response.clicked() || box_response.clicked() {
              c.selected_token = Some(i);
              c.old_color32 = *color32;
            }
          },
        }
      }
    });
  });
}

fn rich_text(text: &str, monospace: bool) -> egui::RichText {
  let mut rich_text = egui::RichText::new(text);
  if monospace { rich_text = rich_text.monospace() }
  rich_text
}

fn render_color_box(ui: &mut egui::Ui, color32: Color32, size: f32) -> Response {
  let (rect, response) = ui.allocate_at_least(egui::Vec2::splat(size), egui::Sense::click());
  ui.painter().rect(
    rect,
    1., // rounding
    color32,
    egui::Stroke::new(1., Color32::WHITE),
  );
  response
}

pub fn render_left(ui: &mut egui::Ui, c: &mut ColorEditor) {
  if ui.add_enabled(!c.autosave, egui::Button::new("Save")).clicked() {
    c.should_save = true;
  }

  ui.checkbox(&mut c.monospace, "Monospace font");
  ui.checkbox(&mut c.autosave, "Autosave");

  ui.separator();

  if let Some(selected_token) = c.selected_token {
    if let Token::Color(_color_kind, ref mut color32) = &mut c.color_file.tokens[selected_token] {
      let color32_before_pick = color32.clone();
      render_color_picker(ui, color32);
      ui.separator();
      ui.label("Previous and current colors:");
      ui.horizontal_wrapped(|ui| {
        render_color_box(ui, c.old_color32, COLOR_COMPARISON_BOX_SIZE);
        render_color_box(ui, *color32, COLOR_COMPARISON_BOX_SIZE);
      });
      if c.autosave && *color32 != color32_before_pick {
        c.should_save = true;
      }

      if ui.button("Restore previous").clicked() {
        *color32 = c.old_color32;
        c.should_save = true;
      }
    }
  }
}

fn render_color_picker(ui: &mut egui::Ui, color32: &mut Color32) {
  use egui::widgets::color_picker::Alpha;
  egui::widgets::color_picker::color_picker_color32(ui, color32, Alpha::Opaque);
}
