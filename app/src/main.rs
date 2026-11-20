//! FireCraft — a modern, dark-first raster and vector editor in the shape of
//! Adobe Fireworks CS6.

mod app;
mod canvas;
mod colors;
mod constants;
mod dialogs;
mod doc;
mod formats;
mod icons;
mod layout;
mod menus;
mod page;
mod panels;
mod theme;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut screenshot_path: Option<String> = None;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot" if i + 1 < args.len() => {
                screenshot_path = Some(args[i + 1].clone());
                i += 2;
            }
            _ => i += 1,
        }
    }

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_title("FireCraft")
        .with_app_id("firecraft.editor")
        .with_visible(true)
        .with_active(true);

    if screenshot_path.is_some() {
        viewport = viewport.with_window_level(eframe::egui::WindowLevel::Normal);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let path_for_closure = screenshot_path.clone();
    eframe::run_native(
        "FireCraft",
        native_options,
        Box::new(move |_cc| Ok(Box::new(app::FireCraft::new(path_for_closure.clone())))),
    )
    .expect("FireCraft could not start");
}
