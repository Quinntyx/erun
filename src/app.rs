use eframe::CreationContext;

use crate::gui::definitions::*;
use crate::model::context::Context;

pub struct App {
    ctx: Context,
    layout: Window,
}

impl App {
    pub fn new(layout: Window) -> Self {
        Self {
            ctx: Context::new(),
            layout,
        }
    }

    pub fn setup(_cc: &CreationContext, window: Window) -> Box<dyn eframe::App> {
        Box::new(Self::new(window))
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let app_ctx = &mut self.ctx;

        app_ctx.update().expect("Update should not fail");

        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            if let Some(title) = &self.layout.title {
                ui.label(title);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.layout
                .content
                .clone()
                .expect("Root window should have content")
                .render(app_ctx, ui);
        });
    }
}
