use egui::Vec2;

pub fn integer_scaling_size(available_size: Vec2, image_size: Vec2) -> Vec2 {
    let screen_width = available_size.x;
    let screen_height = available_size.y;

    let image_width = image_size.x;
    let image_height = image_size.y;

    let width_ratio = (screen_width / image_width).max(1.0);
    let height_ratio = (screen_height / image_height).max(1.0);

    let scale = width_ratio.clamp(1.0, height_ratio).floor();

    let scaled_width = image_width * scale;
    let scaled_height = image_height * scale;

    Vec2::new(scaled_width, scaled_height)
}
