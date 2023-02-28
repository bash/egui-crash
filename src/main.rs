fn main() -> Result<(), eframe::Error>  {
    let options = eframe::NativeOptions {
        // If I disable run_and_return, there's no "crash".
        // run_and_return: false,
        ..Default::default()
    };
    eframe::run_native(
        "egui_crash",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}


#[derive(Default)]
struct App;

impl eframe::App for App {
    fn persist_native_window(&self) -> bool {
        // If I disable persist_native_window, I get no "crash" either.
        true
    }

    fn update(&mut self, _ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
    }
}
