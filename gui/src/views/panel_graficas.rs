use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};

pub struct PanelGraficas;

impl PanelGraficas {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(ui: &mut egui::Ui) {
        ui.heading("Panel de Gráficas");

        // Example data: y = x^2 for x in 0..100
        let points: PlotPoints = (0..100)
            .map(|x| [x as f64, (x as f64).powi(2)])
            .collect();

        let line = Line::new(points);

        Plot::new("example_plot")
            .view_aspect(2.0)
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }
}