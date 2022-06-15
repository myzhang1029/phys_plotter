#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Physics Plotter",
        native_options,
        Box::new(|cc| Box::new(phys_plotter::egui_ui::App::new(cc))),
    );
}
