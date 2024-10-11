use egui::Vec2;
use holiday::get_holidays;

mod holiday;
mod my_error;
mod ui;

fn main() {
    let holiday_data = get_holidays(2024).unwrap();
    if holiday_data.code == 0 {
        let native_options = eframe::NativeOptions {
            centered: true,
            viewport: egui::ViewportBuilder::default()
                .with_resizable(true)
                .with_inner_size(Vec2::new(240.0, 250.0)),
            ..Default::default()
        };
        let _ = eframe::run_native(
            "buling_calendar",
            native_options,
            Box::new(|_cc| Ok(Box::new(ui::MyApp::new(holiday_data)))),
        );
    } else {
        eprintln!("error when get holidays from web");
        panic!();
    }
}
