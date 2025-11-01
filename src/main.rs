use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Cellularity",
        options,
        Box::new(|cc| -> Result<
            Box<dyn eframe::App>,
            Box<dyn std::error::Error + Send + Sync + 'static>,
        > {
            Ok(Box::new(cellularity::ui::app::CellularityApp::new(cc)))
        }),
    )
}
