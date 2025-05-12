use eframe::egui;

pub struct PanelGraficas;

impl PanelGraficas {
    pub fn new() -> Self {
        Self {}
    }
    pub fn show(ui: &mut egui::Ui) {
        ui.heading("Panel de Gráficas");
        ui.label("Aquí se mostrarán las gráficas.");
        // Agrega aquí la lógica para mostrar gráficas
    }
}