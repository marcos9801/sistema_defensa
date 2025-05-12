use std::sync::Mutex;
use eframe::egui;
use crate::metrics::Metrics;
use std::sync::Arc;

pub struct PanelMetricas;

impl PanelMetricas {
    pub fn show(ui: &mut egui::Ui, metrics: &Arc<Mutex<Metrics>>) {
        ui.heading("Panel de Métricas");

        let metrics_guard = metrics.lock().unwrap();
        if let Some(metric) = metrics_guard.latest() {
            ui.horizontal(|ui| {
                ui.label("CPU:");
                ui.label(format!("{:?}", metric.cpu));
            });
            ui.horizontal(|ui| {
                ui.label("Memoria:");
                ui.label(format!("{:?}", metric.memoria));
            });
            ui.horizontal(|ui| {
                ui.label("Red:");
                ui.label(format!("{:?}", metric.red));
            });
            ui.horizontal(|ui| {
                ui.label("Disco:");
                ui.label(format!("{:?}", metric.disco));
            });
        } else {
            ui.label("No metrics available.");
        }
    }
}
