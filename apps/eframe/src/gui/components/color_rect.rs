use egui::{Response, Sense, Ui, epaint::Color32};

pub fn color_rect(ui: &mut Ui, color: Color32) -> Response {
    let size = ui.spacing().interact_size;
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());

    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);

        let rect = rect.expand(visuals.expansion + if response.hovered() { 2.0 } else { 0.0 });

        let rounding = visuals.corner_radius.at_most(2);

        ui.painter().rect_filled(rect, rounding, color);
        ui.painter().rect_stroke(
            rect,
            rounding,
            (2.0, visuals.bg_fill),
            egui::StrokeKind::Inside,
        );
    }

    response
}
