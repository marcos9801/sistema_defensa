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
            ui.label(format!("ID: {}", metric.id));

            ui.collapsing("CPU", |ui| {
                ui.label(format!("Brand: {}", metric.cpu.get_brand()));
                ui.label(format!("Cores: {}", metric.cpu.get_cantidad_nucleos()));
                ui.label(format!("Frequency: {} MHz", metric.cpu.get_frecuencia()));
                for (i, core) in metric.cpu.get_uso_nucleos().iter().enumerate() {
                    ui.label(format!("Core {} Usage: {:.2}%", i, core));
                }
            });

            ui.collapsing("Memoria", |ui| {
                ui.label(format!("Total: {} MB", metric.memoria.get_memoria_total()));
                ui.label(format!("Used: {} MB", metric.memoria.get_memoria_usada()));
                ui.label(format!("Free: {} MB", metric.memoria.get_memoria_libre()));
                ui.label(format!("RAM Total: {} MB", metric.memoria.get_total_ram()));
                ui.label(format!("RAM Used: {} MB", metric.memoria.get_usada_ram()));
                ui.label(format!("RAM Free: {} MB", metric.memoria.get_libre_ram()));
                ui.label(format!("Swap Total: {} MB", metric.memoria.get_swap_total()));
                ui.label(format!("Swap Used: {} MB", metric.memoria.get_swap_usada()));
                ui.label(format!("Swap Free: {} MB", metric.memoria.get_swap_libre()));
            });

            ui.collapsing("Red", |ui| {
                ui.label(format!("Interfaces: {}", metric.red.get_cantidad_interfaces()));
                ui.label(format!("Total RX: {} bytes", metric.red.get_bytes_recibidos()));
                ui.label(format!("Total TX: {} bytes", metric.red.get_bytes_enviados()));
                ui.collapsing("Ver interfaces...", |ui1| {
                    for iface in metric.red.get_interfaces() {
                        ui1.label(format!(
                            "{}: RX {} bytes, TX {} bytes, IPs: {:?}",
                            iface.get_nombre(), iface.get_bytes_recibidos(), iface.get_bytes_enviados(), iface.get_direccion_ip()
                        ));
                    }
                });
            });

            ui.collapsing("Disco", |ui| {
                ui.label(format!("Disks: {}", metric.disco.get_cantidad_discos()));
                ui.label(format!("Total: {:.1} GB", metric.disco.get_espacio_total()));
                ui.label(format!("Used: {:.1} GB", metric.disco.get_espacio_usado()));
                ui.label(format!("Free: {:.1} GB", metric.disco.get_espacio_libre()));
                ui.collapsing("Ver discos...", |ui2| {
                    for disk in metric.disco.get_discos() {
                        ui2.label(format!(
                            "{} ({}): Used {:.1} GB / {:.1} GB, Path: {}",
                            disk.get_nombre(), disk.get_sistema_archivos(), disk.get_espacio_usado(), disk.get_espacio_total(), disk.get_ruta()
                        ));
                    }
                });
            });

            ui.collapsing("Procesos", |ui| {
                ui.label(format!("Total: {}", metric.procesos.get_cantidad_procesos()));
                ui.collapsing("Top procesos uso CPU", |ui3| {
                    for proc in metric.procesos.get_top_procesos_uso_cpu() {
                        ui3.label(format!(
                            "PID: {}, Name: {}, CPU Usage: {:.2}%",
                            proc.get_pid(), proc.get_nombre(), proc.get_uso_cpu()
                        ));
                    }
                });
                ui.collapsing("Top procesos uso RAM", |ui4| {
                    for proc in metric.procesos.get_top_procesos_uso_memoria() {
                        ui4.label(format!(
                            "PID: {}, Name: {}, RAM Usage: {:.2}%",
                            proc.get_pid(), proc.get_nombre(), proc.get_uso_memoria()
                        ));
                    }
                });
                ui.collapsing("Top procesos tiempo de ejecucion", |ui5| {
                    for proc in metric.procesos.get_top_procesos_tiempo_ejecucion() {
                        ui5.label(format!(
                            "PID: {}, Name: {}, Execution Time: {}",
                            proc.get_pid(), proc.get_nombre(), proc.get_tiempo_ejecucion()
                        ));
                    }
                });
                ui.collapsing("Top procesos tiempò en CPU", |ui6| {
                    for proc in metric.procesos.get_top_procesos_tiempo_cpu() {
                        ui6.label(format!(
                            "PID: {}, Name: {}, CPU Time: {}",
                            proc.get_pid(), proc.get_nombre(), proc.get_tiempo_en_cpu()
                        ));
                    }
                });

            });
        } else {
            ui.label("No metrics available.");
        }
    }
}