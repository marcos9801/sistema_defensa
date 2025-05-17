use eframe::egui;
use std::sync::{Arc, Mutex};
use crate::metrics::Metrics;

pub struct PanelAlertas;

impl PanelAlertas {
    pub fn show(
        ui: &mut egui::Ui,
        logs: &[String],
        cpu_alerta_limite: &mut f64,
        mem_alerta_limite: &mut f64,
        metrics: &Arc<Mutex<Metrics>>,
    ) {
        ui.heading("Panel de Alertas");
        ui.label("Alertas:");

        ui.horizontal(|ui| {
            ui.label("CPU uso alerta");
            ui.add(egui::Slider::new(cpu_alerta_limite, 0.0..=100.0).suffix("%"));
        });

        ui.horizontal(|ui| {
            ui.label("Memoria uso alerta");
            ui.add(egui::Slider::new(mem_alerta_limite, 0.0..=100.0).suffix("%"));
        });

        if let Ok(metrics_guard) = metrics.try_lock() {
            if let Some(metric) = metrics_guard.latest() {
                // CPU alert
                let mut sum = 0.0;
                for usage in metric.cpu.get_uso_nucleos() {
                    sum += usage;
                }
                let cpu_usage = (sum / metric.cpu.get_cantidad_nucleos() as f32) as f64;
                if cpu_usage > *cpu_alerta_limite {
                    ui.colored_label(
                        egui::Color32::RED,
                        format!("ALERTA: CPU usada {:.1}% excede el limite", cpu_usage),
                    );
                }

                // Memory alert
                let mem_used = metric.memoria.get_memoria_usada() as f64;
                let mem_total = metric.memoria.get_memoria_total() as f64;
                if mem_total > 0.0 {
                    let mem_usage = (mem_used / mem_total) * 100.0;
                    if mem_usage > *mem_alerta_limite {
                        ui.colored_label(
                            egui::Color32::RED,
                            format!("ALERTA: Memoria usada {:.1}% excede el limite", mem_usage),
                        );
                    }
                }
            }
        }

        for log in logs {
            ui.label(log);
        }
    }
}