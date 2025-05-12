use eframe::egui;

pub struct PanelAlertas;

impl PanelAlertas {
    pub fn new() -> Self {
        Self {}
    }
    pub fn show(ui: &mut egui::Ui, logs: &[String]) {
        ui.heading("Panel de Alertas");
        ui.label("Alertas:");
        for log in logs {
            ui.label(log);
        }
    }
}