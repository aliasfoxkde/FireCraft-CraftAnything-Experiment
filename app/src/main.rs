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
    let mut load_demo = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot" if i + 1 < args.len() => {
                screenshot_path = Some(args[i + 1].clone());
                i += 2;
            }
            "--demo" => {
                load_demo = true;
                i += 1;
            }
            _ => i += 1,
        }
    }

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([1280.0, 820.0])
        .with_min_inner_size([800.0, 600.0])
        .with_title("FireCraft Studio — Adobe Fireworks CS6 Edition")
        .with_app_id("firecraft.editor")
        .with_visible(true)
        .with_active(screenshot_path.is_none());

    if screenshot_path.is_some() {
        viewport = viewport.with_window_level(eframe::egui::WindowLevel::AlwaysOnBottom);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    let path_for_closure = screenshot_path.clone();
    eframe::run_native(
        "FireCraft",
        native_options,
        Box::new(move |_cc| {
            let mut fc = app::FireCraft::new(path_for_closure.clone());
            if load_demo {
                fc.load_demo_showcase();
            }
            Ok(Box::new(fc))
        }),
    )
    .expect("FireCraft could not start");
}
