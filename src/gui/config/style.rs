use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Style {
    #[serde(default)]
    pub faint_bg_color: Option<egui::Color32>,
    #[serde(default)]
    pub code_bg_color: Option<egui::Color32>,
    #[serde(default)]
    pub extreme_bg_color: Option<egui::Color32>,
    #[serde(default)]
    pub warn_fg_color: Option<egui::Color32>,
    #[serde(default)]
    pub error_fg_color: Option<egui::Color32>,
    #[serde(default)]
    pub window_rounding: Option<egui::Rounding>,
    #[serde(default)]
    pub panel_fill: Option<egui::Color32>,
    #[serde(default)]
    pub zoom_factor: Option<f32>,
}

impl Into<egui::Style> for Style {
    fn into(self) -> egui::Style {
        let mut visuals = egui::Visuals::default();
        visuals.faint_bg_color = self.faint_bg_color.unwrap_or(visuals.faint_bg_color);
        visuals.code_bg_color = self.code_bg_color.unwrap_or(visuals.code_bg_color);
        visuals.extreme_bg_color = self.extreme_bg_color.unwrap_or(visuals.extreme_bg_color);
        visuals.warn_fg_color = self.warn_fg_color.unwrap_or(visuals.warn_fg_color);
        visuals.error_fg_color = self.error_fg_color.unwrap_or(visuals.error_fg_color);
        visuals.window_rounding = self.window_rounding.unwrap_or(visuals.window_rounding);
        visuals.panel_fill = self.panel_fill.unwrap_or(visuals.panel_fill);

        // TODO: implement configurable widgets
        let widgets = egui::style::Widgets::default();

        visuals.widgets = widgets;

        let style = egui::Style {
            visuals,
            ..Default::default()
        };
        style
    }
}
