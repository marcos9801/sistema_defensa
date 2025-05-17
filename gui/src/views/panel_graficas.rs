use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use crate::metrics::{Metrics, Metric};

pub struct PanelGraficas {
    historico_cpu: VecDeque<f64>,
    historico_memoria: VecDeque<f64>,
    historico_red: VecDeque<f64>,
    historico_disco: VecDeque<f64>,
    max_puntos: usize,
}

impl PanelGraficas {
    pub fn new() -> Self {
        Self {
            historico_cpu: VecDeque::with_capacity(100),
            historico_memoria: VecDeque::with_capacity(100),
            historico_red: VecDeque::with_capacity(100),
            historico_disco: VecDeque::with_capacity(100),
            max_puntos: 100,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, metrics: &Arc<Mutex<Metrics>>) {
        ui.heading("Panel de Gráficas");

        if let Ok(metrics_guard) = metrics.lock() {
            if let Some(metric) = metrics_guard.latest() {
                // Actualizar datos históricos
                self.actualizar_historicos(metric);

                // Gráfica CPU
                ui.collapsing("CPU", |ui| {
                    self.mostrar_grafica_cpu(ui);
                });

                // Gráfica Memoria
                ui.collapsing("Memoria", |ui| {
                    self.mostrar_grafica_memoria(ui);
                });

                // Gráfica Red
                ui.collapsing("Red", |ui| {
                    self.mostrar_grafica_red(ui);
                });

                // Gráfica Disco
                ui.collapsing("Disco", |ui| {
                    self.mostrar_grafica_disco(ui);
                });
            }
        }
    }

    fn actualizar_historicos(&mut self, metric: &Metric) {
        //generar variable suma
        let mut suma = 0.0;


        for uso in metric.cpu.get_uso_nucleos() {
            suma += uso;
        }
        let promedio = (suma as f64 / metric.cpu.get_cantidad_nucleos() as f64) as f64;
        self.historico_cpu.push_back(promedio);
        let porcentaje = (metric.memoria.get_memoria_usada() as f64 / metric.memoria.get_memoria_total() as f64) * 100.0;
        self.historico_memoria.push_back(porcentaje);
        self.historico_red.push_back(metric.red.get_bytes_recibidos() as f64 / 1000000.0);
        let disco = metric.disco.get_espacio_total() - metric.disco.get_espacio_usado();
        let porcentaje_disco = (disco as f64 / metric.disco.get_espacio_total() as f64) * 100.0;
        self.historico_disco.push_back(porcentaje_disco);

        while self.historico_cpu.len() > self.max_puntos {
            self.historico_cpu.pop_front();
        }
        // Repeat for other VecDeque fields as needed
    }

    fn mostrar_grafica_cpu(&self, ui: &mut egui::Ui) {
        let points: PlotPoints = self.historico_cpu
            .iter()
            .enumerate()
            .map(|(i, &v)| [i as f64, v])
            .collect();

        let line = Line::new(points)
            .color(egui::Color32::from_rgb(100, 200, 100))
            .name("CPU Usage");

        Plot::new("cpu_plot")
            .view_aspect(2.0)
            .include_y(0.0)
            .include_y(100.0)
            .label_formatter(|name, value| {
                format!("{}: {:.1}%", name, value.y)
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }

    fn mostrar_grafica_memoria(&self, ui: &mut egui::Ui) {
        let points: PlotPoints = self.historico_memoria
            .iter()
            .enumerate()
            .map(|(i, &v)| [i as f64, v])
            .collect();

        let line = Line::new(points)
            .color(egui::Color32::from_rgb(100, 100, 200))
            .name("Memoria Usage");

        Plot::new("memoria_plot")
            .view_aspect(2.0)
            .include_y(0.0)
            .include_y(100.0)
            .label_formatter(|name, value| {
                format!("{}: {:.1}%", name, value.y)
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }

    fn mostrar_grafica_red(&self, ui: &mut egui::Ui) {
        let points: PlotPoints = self.historico_red
            .iter()
            .enumerate()
            .map(|(i, &v)| [i as f64, v])
            .collect();

        let line = Line::new(points)
            .color(egui::Color32::from_rgb(200, 100, 100))
            .name("Red Usage");

        Plot::new("red_plot")
            .view_aspect(2.0)
            .include_y(0.0)
            .label_formatter(|name, value| {
                format!("{}: {:.1}", name, value.y)
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }

    fn mostrar_grafica_disco(&self, ui: &mut egui::Ui) {
        let points: PlotPoints = self.historico_disco
            .iter()
            .enumerate()
            .map(|(i, &v)| [i as f64, v])
            .collect();

        let line = Line::new(points)
            .color(egui::Color32::from_rgb(100, 200, 200))
            .name("Disco Usage");

        Plot::new("disco_plot")
            .view_aspect(2.0)
            .include_y(0.0)
            .include_y(100.0)
            .label_formatter(|name, value| {
                format!("{}: {:.1}%", name, value.y)
            })
            .show(ui, |plot_ui| {
                plot_ui.line(line);
            });
    }
}