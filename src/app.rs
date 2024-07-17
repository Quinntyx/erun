use eframe::CreationContext;

use crate::gui::windows::Window;
use crate::model::context::Context;

/// The main application struct for `erun`. 
///
/// * `ctx`: The application context. 
/// * `layout`: The loaded Window layout currently being displayed in this window. 
pub struct App {
    ctx: Context,
    layout: Window,
}

impl App {
    /// Constructor. 
    ///
    /// * `layout`: The layout of the window to construct the App with. 
    pub fn new(layout: Window) -> Self {
        Self {
            ctx: Context::new(),
            layout,
        }
    }

    /// Setup function. Called in main and passed to eframe::run_native. 
    ///
    /// * `cc`: 
    /// * `window`: 
    pub fn setup(cc: &CreationContext, window: Window) -> Box<dyn eframe::App> {
        if let Some(style) = &window.style {
            let estyle: egui::Style = style.clone().into();
            if let Some(zoom) = &window.zoom_factor {
                cc.egui_ctx.set_zoom_factor(*zoom);
            }
            cc.egui_ctx.set_style(std::sync::Arc::new(estyle));
        }

        Box::new(Self::new(window))
    }
}

impl eframe::App for App {
    /// Implementation of update. 
    ///
    /// * `ctx`: egui Context. 
    /// * `_frame`: eframe Frame. Unused. 
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let app_ctx = &mut self.ctx;

        app_ctx.update().expect("Update should not fail");

        if let Some(title) = &self.layout.title {
            egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
                ui.label(title);
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            self.layout
                .content
                .clone()
                .expect("Root window should have content")
                .render(app_ctx, ui);
        });
    }
}
