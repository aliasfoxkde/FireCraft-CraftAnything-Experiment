#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! FireCraft — a modern, dark-first raster and vector editor in the shape of
//! Adobe Fireworks CS6, with an embedded MCP JSON-RPC 2.0 server and headless CLI.

mod app;
mod canvas;
mod colors;
mod constants;
mod demo;
mod dialogs;
mod doc;
mod formats;
#[cfg(test)]
mod harness;
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
    let bytes = include_bytes!("../assets/logo.png");
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
    let mut interactive_state: Option<String> = None;
    let mut window_size = (1360.0_f32, 860.0_f32);
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
            "--size" if i + 1 < args.len() => {
                if let Some((w, h)) = args[i + 1].split_once('x')
                    && let (Ok(w), Ok(h)) = (w.parse::<f32>(), h.parse::<f32>())
                {
                    window_size = (w.max(960.0), h.max(640.0));
                }
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
            "--state" if i + 1 < args.len() => {
                interactive_state = Some(args[i + 1].clone());
                i += 2;
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
        .with_inner_size([window_size.0, window_size.1])
        .with_min_inner_size([960.0, 640.0])
        .with_title("FireCraft - Open Source Raster and Vector Graphics Editor")
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
        // Screenshot mode must not read persisted window state: a stored
        // inner_size overrides --size and every captured layout would be the
        // last interactive window size. (persist_window alone only stops
        // saving, not loading.) 18.3.
        persistence_path: screenshot_path
            .as_ref()
            .map(|_| std::env::temp_dir().join("firecraft-screenshot-persistence-unused")),
        // A persisted size would override --size in screenshot mode, making
        // every captured layout the last interactive window size.
        persist_window: screenshot_path.is_none(),
        ..Default::default()
    };

    let path_for_closure = screenshot_path.clone();
    eframe::run_native(
        "FireCraft",
        native_options,
        Box::new(move |cc| {
            icons::install_phosphor_font(&cc.egui_ctx);
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
            match interactive_state.as_deref() {
                // G-5 interactive states for scripted capture: an open menu,
                // the iconic flyout, the find-and-replace dialog.
                Some("menu") => {
                    fc.show_welcome = false;
                    fc.active_menu = Some(0);
                }
                Some("flyout") => {
                    fc.show_welcome = false;
                    fc.apply_workspace_preset("Iconic Mode");
                    if let Some(&pid) = fc.panel_groups[0].tabs.first() {
                        fc.iconic_flyout_panel = Some(pid);
                    }
                }
                Some("find-replace") => {
                    fc.show_welcome = false;
                    fc.show_find_replace = true;
                }
                other => {
                    if let Some(s) = other {
                        fc.status = format!("Unknown --state '{s}' (menu|flyout|find-replace)");
                    }
                }
            }
            Ok(Box::new(fc))
        }),
    )
    .expect("FireCraft could not start");
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    // WebRunner owns the resize observer and animation frame. Keep it alive for the page lifetime.
    static WEB_RUNNER: std::cell::RefCell<Option<eframe::WebRunner>> = const { std::cell::RefCell::new(None) };
    static BROWSER_PASTE_QUEUE: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

/// Queue an asynchronous browser clipboard payload for consumption during the next UI frame.
#[cfg(target_arch = "wasm32")]
pub(crate) fn queue_browser_paste(payload: String) {
    BROWSER_PASTE_QUEUE.with(|queue| queue.borrow_mut().push(payload));
}

/// Drain image payloads collected by the browser clipboard listener.
#[cfg(target_arch = "wasm32")]
pub(crate) fn take_browser_pastes() -> Vec<String> {
    BROWSER_PASTE_QUEUE.with(|queue| std::mem::take(&mut *queue.borrow_mut()))
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
        let runner = eframe::WebRunner::new();
        let start_result = runner
            .start(
                canvas,
                web_options,
                Box::new(|cc| {
                    icons::install_phosphor_font(&cc.egui_ctx);
                    install_browser_image_paste(cc.egui_ctx.clone());
                    Ok(Box::new(app::FireCraft::new(None)))
                }),
            )
            .await;
        WEB_RUNNER.with(|slot| slot.replace(Some(runner)));
        if let Err(error) = start_result {
            web_sys::console::error_1(&error);
            if let Some(message) = document.get_element_by_id("firecraft-startup-error") {
                let detail = error.as_string().unwrap_or_else(|| {
                    "The browser could not initialize the graphics renderer.".to_string()
                });
                message.set_text_content(Some(&format!(
                    "FireCraft could not start. Try refreshing the page or using a browser with WebGL2 enabled. Details: {detail}"
                )));
                if let Ok(message) = message.dyn_into::<web_sys::HtmlElement>() {
                    message.set_hidden(false);
                }
            }
        }
    });
}

/// Browser clipboard paste events expose image blobs through DataTransfer rather than
/// egui's text-only `Event::Paste`. Read those blobs asynchronously and feed the bytes
/// into the same paste action path as text clipboard events.
#[cfg(target_arch = "wasm32")]
fn install_browser_image_paste(ctx: eframe::egui::Context) {
    use wasm_bindgen::{JsCast as _, closure::Closure};

    let Some(document) = web_sys::window().and_then(|window| window.document()) else {
        return;
    };
    let on_paste = Closure::<dyn FnMut(web_sys::ClipboardEvent)>::new(
        move |event: web_sys::ClipboardEvent| {
            let Some(items) = event.clipboard_data().map(|data| data.items()) else {
                return;
            };
            for index in 0..items.length() {
                let Some(item) = items.get(index) else {
                    continue;
                };
                if !item.type_().starts_with("image/") {
                    continue;
                }
                let Ok(Some(file)) = item.get_as_file() else {
                    continue;
                };
                event.prevent_default();
                let Ok(reader) = web_sys::FileReader::new() else {
                    continue;
                };
                let callback_ctx = ctx.clone();
                let on_load = Closure::once_into_js(move |load_event: web_sys::Event| {
                    let Some(reader) = load_event
                        .target()
                        .and_then(|target| target.dyn_into::<web_sys::FileReader>().ok())
                    else {
                        return;
                    };
                    let Ok(result) = reader.result() else {
                        return;
                    };
                    let bytes = js_sys::Uint8Array::new(&result).to_vec();
                    if bytes.is_empty() {
                        return;
                    }
                    let payload =
                        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, bytes);
                    queue_browser_paste(format!("FIRECRAFT_IMAGE:{payload}"));
                    callback_ctx.request_repaint();
                });
                reader.set_onload(Some(on_load.unchecked_ref()));
                if reader.read_as_array_buffer(&file).is_ok() {
                    break;
                }
            }
        },
    );
    if document
        .add_event_listener_with_callback("paste", on_paste.as_ref().unchecked_ref())
        .is_ok()
    {
        on_paste.forget();
    }
}
