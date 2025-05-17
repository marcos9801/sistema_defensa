use std::sync::{Arc, Mutex};
use eframe::egui;

use crate::{metrics, views};
use views::panel_metricas::PanelMetricas;
use views::panel_alertas::PanelAlertas;
use views::panel_graficas::PanelGraficas;

use metrics::Metrics;

pub struct SistemaDefensaApp {
    pub monitoring: bool,
    pub metrics: Arc<Mutex<Metrics>>,
    pub logs: Vec<String>,
    pub vista_actual: Vista,
    pub panel_graficas: PanelGraficas,
}

enum Monitoreado {
    Activado,
    Desactivado,
}
enum Vista {
    PanelMetricas,
    PanelAlertas,
    PanelGraficas,
}

impl Default for SistemaDefensaApp {
    fn default() -> Self {
        Self {
            monitoring: false,
            metrics: Arc::new(Mutex::new(Metrics::new())),
            logs: Vec::new(),
            vista_actual: Vista::PanelMetricas,
            panel_graficas: PanelGraficas::new(),
            // panel_graficas: PanelGraficas::new(),
        }
    }
}

impl eframe::App for SistemaDefensaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Sistema de Defensa");

                if ui.button("Panel de Metricas").clicked() {
                    self.vista_actual = Vista::PanelMetricas;
                    ctx.request_repaint();
                }
                if ui.button("Panel de alertas").clicked() {
                    self.vista_actual = Vista::PanelAlertas;
                    ctx.request_repaint();
                }
                if ui.button("Mostrar Graficas").clicked() {
                    self.vista_actual = Vista::PanelGraficas;
                    ctx.request_repaint();
                }
                if ui.button(if self.monitoring { "Parar Monitoreo" } else { "Iniciar Monitoreo" }).clicked() {
                    if !self.monitoring {
                        self.monitoring = true;
                        let metrics_clone = Arc::clone(&self.metrics);
                        self.metrics.lock().unwrap().start_monitoring(metrics_clone);
                    } else {
                        self.monitoring = false;
                        self.metrics.lock().unwrap().stop_monitoring();
                    }
                    ctx.request_repaint();
                }
                if ui.button("Salir").clicked() {
                    ctx.request_repaint();
                    std::process::exit(0);
                }
            });
        });

        match self.vista_actual {
            Vista::PanelMetricas => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    PanelMetricas::show(ui, &self.metrics);
                });
            }
            Vista::PanelAlertas => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    PanelAlertas::show(ui, &self.logs);
                });
            }
            Vista::PanelGraficas => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    self.panel_graficas.show(ui, &self.metrics);
                });
            }
        }
    }
}