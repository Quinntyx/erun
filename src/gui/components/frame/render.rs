use egui::Ui;

use crate::gui::components::Component;
use crate::model::context::Context;

pub fn render(c: &Component, app_ctx: &mut Context, ui: &mut Ui) {
    let Component::Frame {
        padding,
        margin,
        content,
    } = c
    else {
        unreachable!()
    };

    egui::Frame::none()
        .inner_margin(padding.clone())
        .outer_margin(margin.clone())
        .show(ui, |ui| {
            content.render(app_ctx, ui);
        });
}
