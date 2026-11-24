#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! FireCraft — a modern, dark-first raster and vector editor in the shape of
//! Adobe Fireworks CS6, with an embedded MCP JSON-RPC 2.0 server and headless CLI.

mod app;
mod canvas;
mod colors;
mod constants;
mod dialogs;
mod doc;
mod formats;
mod icons;
mod layout;
mod mcp;
mod menus;
mod page;
mod panels;
#[cfg(windows)]
mod shell_windows;
mod theme;

#[cfg(not(target_arch = "wasm32"))]
fn load_window_icon() -> Option<eframe::egui::IconData> {
    let bytes = include_bytes!("logo.png");
    let img = image::load_from_memory(bytes).ok()?.into_rgba8();
    let (width, height) = img.dimensions();
    Some(eframe::egui::IconData {
        rgba: img.into_raw(),
        width,
        height,
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    // If a headless CLI verb or `mcp` server mode was invoked, attach to parent console
    // (on Windows release builds) and execute immediately without opening a GUI window.
    if args.len() > 1
        && !args[1].starts_with("--screenshot")
        && args[1] != "--demo"
        && args[1] != "--tab"
        && args[1] != "--splash"
        && let Some(code) = mcp::try_run_cli(&args)
    {
        std::process::exit(code);
    }

    let mut screenshot_path: Option<String> = None;
    let mut load_demo = false;
    let mut show_splash = false;
    let mut initial_top_tab: Option<usize> = None;
    let mut initial_path: Option<String> = None;
    let mut new_document = false;
    let mut show_about = false;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--screenshot" if i + 1 < args.len() => {
                screenshot_path = Some(args[i + 1].clone());
                i += 2;
            }
            "--tab" if i + 1 < args.len() => {
                initial_top_tab = args[i + 1].parse::<usize>().ok();
                i += 2;
            }
            "--demo" => {
                load_demo = true;
                i += 1;
            }
            "--splash" => {
                show_splash = true;
                i += 1;
            }
            "--new" => {
                new_document = true;
                i += 1;
            }
            "--about" => {
                show_about = true;
                i += 1;
            }
            arg if !arg.starts_with('-') && initial_path.is_none() => {
                initial_path = Some(arg.to_string());
                i += 1;
            }
            _ => i += 1,
        }
    }

    #[cfg(windows)]
    shell_windows::init_app_id();
    #[cfg(windows)]
    shell_windows::update_jump_list(&[]);

    let mut viewport = eframe::egui::ViewportBuilder::default()
        .with_inner_size([1360.0, 860.0])
        .with_min_inner_size([960.0, 640.0])
        .with_title("FireCraft Studio - Open Source bitmap and vector Graphics Editor")
        .with_app_id("firecraft.editor")
        .with_visible(true)
        .with_active(screenshot_path.is_none());

    if let Some(icon) = load_window_icon() {
        viewport = viewport.with_icon(std::sync::Arc::new(icon));
    }

    if screenshot_path.is_some() {
        viewport = viewport.with_window_level(eframe::egui::WindowLevel::AlwaysOnBottom);
    }

    let native_options = eframe::NativeOptions {
        viewport,
        centered: true,
        persist_window: true,
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
            if show_splash {
                fc.show_welcome = true;
            }
            if let Some(t) = initial_top_tab {
                fc.top_right_tab = t.min(3);
            }
            if let Some(path) = &initial_path
                && let Err(err) = fc.open_path(std::path::Path::new(path))
            {
                fc.status = err;
            }
            if new_document {
                fc.show_welcome = false;
            }
            fc.show_about_dialog = show_about;
            Ok(Box::new(fc))
        }),
    )
    .expect("FireCraft could not start");
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast as _;

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("firecraft-canvas")
            .expect("Failed to find #firecraft-canvas")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("#firecraft-canvas was not a HtmlCanvasElement");

        let web_options = eframe::WebOptions::default();
        let _ = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|_cc| Ok(Box::new(app::FireCraft::new(None)))),
            )
            .await;
    });
}
