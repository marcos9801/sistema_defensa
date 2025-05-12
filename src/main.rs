use gui::SistemaDefensaApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sistema de Defensa",
        native_options,
        Box::new(|_cc| Ok(Box::new(SistemaDefensaApp::default()))),
    )
}
