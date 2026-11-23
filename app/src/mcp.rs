//! MCP (Model Context Protocol) JSON-RPC 2.0 Server & Headless CLI engine for FireCraft.
//!
//! Extracted and adapted from `C:\Repos\FireCraft\crates\app-cli\src\mcp.rs` and `main.rs`:
//! - Headless CLI verbs:
//!   - `firecraft info <file>` — JSON summary of document dimensions, pages, states, layers, and objects
//!   - `firecraft export <in> <out.(png|jpg|gif|webp|tiff|bmp|ico|svg|html)>` — headless conversion/export
//!   - `firecraft batch <dir> --format <ext> [--out <out-dir>]` — batch-convert a directory of files
//!   - `firecraft list-commands` — list every registered editor/automation command ID
//!   - `firecraft mcp [file]` — start the stdio JSON-RPC 2.0 MCP server (`2025-06-18` protocol)
//!
//! - MCP tools exposed over stdio & in-app MCP Console:
//!   - `firecraft.info`
//!   - `firecraft.apply` (`{command, args}`)
//!   - `firecraft.export` (`{path}`)
//!   - `firecraft.undo`
//!   - `firecraft.redo`
//!   - `firecraft.listCommands`
//! - MCP resource:
//!   - `document://current` (`application/json`)

use crate::app::FireCraft;
use crate::doc::{AutoShapeKind, Document, LiveEffect};
use crate::formats;
use eframe::egui::Color32;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};

pub const PROTOCOL_VERSION: &str = "2025-06-18";

/// Canonical command catalog shared by the MCP server, the CLI, and the Ctrl+K Command Palette.
pub const COMMAND_CATALOG: &[(&str, &str)] = &[
    ("document.new", "Create a new blank document (800×600)"),
    (
        "document.demoShowcase",
        "Populate canvas with CS6 Vector + Live Filters + Web Slice showcase",
    ),
    (
        "document.fitCanvas",
        "Fit canvas bounds tightly to all visible objects",
    ),
    ("document.rotateCanvas90", "Rotate canvas 90° clockwise"),
    (
        "document.toggleTransparent",
        "Toggle transparent canvas background",
    ),
    (
        "document.addThirdsGuides",
        "Add 3×3 rule-of-thirds cyan guides",
    ),
    ("page.add", "Add a new page to the document"),
    ("layer.add", "Add a new layer above the active layer"),
    ("state.add", "Add a new animation / rollover state"),
    ("state.duplicate", "Duplicate the current animation state"),
    (
        "state.tween3x",
        "Generate 3 motion-tween animation states from selection",
    ),
    ("edit.undo", "Undo last step (Ctrl+Z)"),
    ("edit.redo", "Redo last undone step (Ctrl+Y)"),
    ("edit.cut", "Cut selection to clipboard (Ctrl+X)"),
    ("edit.copy", "Copy selection to clipboard (Ctrl+C)"),
    ("edit.paste", "Paste clipboard onto active layer (Ctrl+V)"),
    ("edit.duplicate", "Duplicate selection (Ctrl+D)"),
    ("edit.clone", "Clone selection in place (Ctrl+Shift+D)"),
    ("edit.delete", "Delete selected object(s) (Del)"),
    (
        "edit.selectAll",
        "Select all objects on active layer (Ctrl+A)",
    ),
    ("edit.deselect", "Clear selection (Ctrl+Shift+D)"),
    (
        "object.group",
        "Group selected objects into a single compound object (Ctrl+G)",
    ),
    (
        "object.ungroup",
        "Ungroup selected group into individual objects (Ctrl+Shift+G)",
    ),
    ("object.bringToFront", "Bring selection to front of layer"),
    ("object.sendToBack", "Send selection to back of layer"),
    ("object.flipHorizontal", "Flip selection horizontally"),
    ("object.flipVertical", "Flip selection vertically"),
    ("object.rotate90", "Rotate selection 90° clockwise"),
    ("object.scale125", "Scale selection to 125%"),
    ("object.scale80", "Scale selection to 80%"),
    (
        "object.convertToPath",
        "Convert geometric primitive or text to editable Vector Path",
    ),
    (
        "object.convertToSymbol",
        "Convert selection to reusable Library Symbol (F8)",
    ),
    (
        "object.flattenToBitmap",
        "Rasterize selection into an editable Bitmap object",
    ),
    ("align.left", "Align selection to left edge"),
    (
        "align.centerHorizontal",
        "Align selection to horizontal center",
    ),
    ("align.right", "Align selection to right edge"),
    ("align.top", "Align selection to top edge"),
    ("align.centerVertical", "Align selection to vertical center"),
    ("align.bottom", "Align selection to bottom edge"),
    (
        "autoshape.roundedRect",
        "Insert Rounded Rectangle Auto Shape",
    ),
    ("autoshape.star", "Insert 5-Point Star Auto Shape"),
    ("autoshape.polygon", "Insert Smart Polygon Auto Shape"),
    ("autoshape.arrow", "Insert Arrow Auto Shape"),
    (
        "autoshape.callout",
        "Insert Talk Bubble / Callout Auto Shape",
    ),
    ("autoshape.gear", "Insert Cog / Gear Auto Shape"),
    ("autoshape.spiral", "Insert Spiral Auto Shape"),
    ("filter.dropShadow", "Add Live Filter: Drop Shadow"),
    ("filter.innerShadow", "Add Live Filter: Inner Shadow"),
    ("filter.outerGlow", "Add Live Filter: Outer Glow"),
    ("filter.innerBevel", "Add Live Filter: Inner Bevel"),
    ("filter.gaussianBlur", "Add Live Filter: Gaussian Blur"),
    ("filter.brighten", "Adjust Color: Brightness +"),
    ("filter.darken", "Adjust Color: Darken -"),
    ("filter.invert", "Adjust Color: Invert Colors"),
    ("filter.grayscale", "Adjust Color: Convert to Grayscale"),
    ("filter.sepia", "Adjust Color: Sepia Tone"),
    ("filter.clearAll", "Remove all Live Filters from selection"),
    ("web.insertSlice", "Insert Web Slice overlay on Web Layer"),
    ("web.insertHotspot", "Insert Web Hotspot on Web Layer"),
    (
        "web.copyCss",
        "Generate and copy CSS3 properties for selection",
    ),
    ("view.fitInWindow", "Zoom to fit page in viewport (Ctrl+0)"),
    ("view.zoom100", "Set magnification to 100%"),
    ("view.toggleGrid", "Toggle document grid overlay"),
    ("view.toggleGuides", "Toggle cyan guides overlay"),
    ("view.toggleRulers", "Toggle pixel rulers"),
    ("theme.dark", "Switch to Dark (CS6 Charcoal) theme"),
    (
        "theme.light",
        "Switch to Light (CS6 Classic Platinum) theme",
    ),
    ("theme.system", "Follow OS system theme"),
];

/// Execute a catalog command ID against a `FireCraft` instance.
pub fn execute_catalog_command(app: &mut FireCraft, cmd: &str) -> Result<String, String> {
    match cmd {
        "document.new" => {
            app.new_document(800.0, 600.0);
            Ok("Created 800×600 document".to_string())
        }
        "document.demoShowcase" => {
            app.load_demo_showcase();
            Ok("Loaded Fireworks CS6 Demo Showcase".to_string())
        }
        "document.fitCanvas" => {
            app.fit_canvas_to_contents();
            Ok(app.status.clone())
        }
        "document.rotateCanvas90" => {
            app.rotate_canvas_90();
            Ok(app.status.clone())
        }
        "document.toggleTransparent" => {
            let p = app.doc.page_mut();
            p.transparent_bg = !p.transparent_bg;
            Ok(format!("Transparent canvas: {}", p.transparent_bg))
        }
        "document.addThirdsGuides" => {
            app.command_add_thirds_guides();
            Ok(app.status.clone())
        }
        "page.add" => {
            app.add_page();
            Ok(app.status.clone())
        }
        "layer.add" => {
            app.add_layer();
            Ok(app.status.clone())
        }
        "state.add" => {
            app.add_state();
            Ok(app.status.clone())
        }
        "state.duplicate" => {
            app.duplicate_state();
            Ok(app.status.clone())
        }
        "state.tween3x" => {
            app.tween_selected_across_states();
            Ok(app.status.clone())
        }
        "edit.undo" => {
            app.undo_action();
            Ok(app.status.clone())
        }
        "edit.redo" => {
            app.redo_action();
            Ok(app.status.clone())
        }
        "edit.cut" => {
            app.cut_selected();
            Ok(app.status.clone())
        }
        "edit.copy" => {
            app.copy_selected();
            Ok(app.status.clone())
        }
        "edit.paste" => {
            app.paste();
            Ok(app.status.clone())
        }
        "edit.duplicate" => {
            app.duplicate_selected();
            Ok(app.status.clone())
        }
        "edit.clone" => {
            app.clone_selected();
            Ok(app.status.clone())
        }
        "edit.delete" => {
            app.delete_selected();
            Ok(app.status.clone())
        }
        "edit.selectAll" => {
            app.select_all();
            Ok(app.status.clone())
        }
        "edit.deselect" => {
            app.clear_selection();
            Ok("Deselected".to_string())
        }
        "object.group" => {
            app.group_selected();
            Ok(app.status.clone())
        }
        "object.ungroup" => {
            app.ungroup_selected();
            Ok(app.status.clone())
        }
        "object.bringToFront" => {
            app.reorder_selected(true);
            Ok(app.status.clone())
        }
        "object.sendToBack" => {
            app.reorder_selected(false);
            Ok(app.status.clone())
        }
        "object.flipHorizontal" => {
            app.flip_horizontal();
            Ok(app.status.clone())
        }
        "object.flipVertical" => {
            app.flip_vertical();
            Ok(app.status.clone())
        }
        "object.rotate90" => {
            app.rotate_90();
            Ok(app.status.clone())
        }
        "object.scale125" => {
            app.scale_selected(1.25);
            Ok(app.status.clone())
        }
        "object.scale80" => {
            app.scale_selected(0.80);
            Ok(app.status.clone())
        }
        "object.convertToPath" => {
            app.convert_selected_to_path();
            Ok(app.status.clone())
        }
        "object.convertToSymbol" => {
            app.convert_selection_to_symbol();
            Ok(app.status.clone())
        }
        "object.flattenToBitmap" => {
            app.flatten_selected_to_bitmap();
            Ok(app.status.clone())
        }
        "align.left" => {
            app.align_selected_to_canvas("left");
            Ok(app.status.clone())
        }
        "align.centerHorizontal" => {
            app.align_selected_to_canvas("center_x");
            Ok(app.status.clone())
        }
        "align.right" => {
            app.align_selected_to_canvas("right");
            Ok(app.status.clone())
        }
        "align.top" => {
            app.align_selected_to_canvas("top");
            Ok(app.status.clone())
        }
        "align.centerVertical" => {
            app.align_selected_to_canvas("center_y");
            Ok(app.status.clone())
        }
        "align.bottom" => {
            app.align_selected_to_canvas("bottom");
            Ok(app.status.clone())
        }
        "autoshape.roundedRect" => {
            app.insert_auto_shape(AutoShapeKind::RoundedRect);
            Ok(app.status.clone())
        }
        "autoshape.star" => {
            app.insert_auto_shape(AutoShapeKind::Star);
            Ok(app.status.clone())
        }
        "autoshape.polygon" => {
            app.insert_auto_shape(AutoShapeKind::Polygon);
            Ok(app.status.clone())
        }
        "autoshape.arrow" => {
            app.insert_auto_shape(AutoShapeKind::Arrow);
            Ok(app.status.clone())
        }
        "autoshape.callout" => {
            app.insert_auto_shape(AutoShapeKind::Callout);
            Ok(app.status.clone())
        }
        "autoshape.gear" => {
            app.insert_auto_shape(AutoShapeKind::Gear);
            Ok(app.status.clone())
        }
        "autoshape.spiral" => {
            app.insert_auto_shape(AutoShapeKind::Spiral);
            Ok(app.status.clone())
        }
        "filter.dropShadow" => {
            app.add_live_effect(LiveEffect::DropShadow {
                distance: 5.0,
                angle_deg: 45.0,
                blur: 5.0,
                color: Color32::from_black_alpha(150),
                enabled: true,
            });
            Ok(app.status.clone())
        }
        "filter.innerShadow" => {
            app.add_live_effect(LiveEffect::InnerShadow {
                distance: 3.0,
                blur: 4.0,
                color: Color32::from_black_alpha(140),
                enabled: true,
            });
            Ok(app.status.clone())
        }
        "filter.outerGlow" => {
            app.add_live_effect(LiveEffect::Glow {
                radius: 6.0,
                color: Color32::from_rgba_unmultiplied(232, 163, 61, 180),
                inner: false,
                enabled: true,
            });
            Ok(app.status.clone())
        }
        "filter.innerBevel" => {
            app.add_live_effect(LiveEffect::Bevel {
                width: 2.0,
                highlight: Color32::from_white_alpha(190),
                shadow: Color32::from_black_alpha(160),
                enabled: true,
            });
            Ok(app.status.clone())
        }
        "filter.gaussianBlur" => {
            app.add_live_effect(LiveEffect::GaussianBlur {
                radius: 4.0,
                enabled: true,
            });
            Ok(app.status.clone())
        }
        "filter.brighten" => {
            app.apply_color_command("brighten");
            Ok(app.status.clone())
        }
        "filter.darken" => {
            app.apply_color_command("darken");
            Ok(app.status.clone())
        }
        "filter.invert" => {
            app.apply_color_command("invert");
            Ok(app.status.clone())
        }
        "filter.grayscale" => {
            app.apply_color_command("grayscale");
            Ok(app.status.clone())
        }
        "filter.sepia" => {
            app.apply_color_command("sepia");
            Ok(app.status.clone())
        }
        "filter.clearAll" => {
            app.clear_live_effects();
            Ok(app.status.clone())
        }
        "web.insertSlice" => {
            app.insert_slice(false);
            Ok(app.status.clone())
        }
        "web.insertHotspot" => {
            app.insert_slice(true);
            Ok(app.status.clone())
        }
        "web.copyCss" => {
            app.copy_selected_css();
            Ok(app.status.clone())
        }
        "view.fitInWindow" => {
            app.fit_in_window();
            Ok(app.status.clone())
        }
        "view.zoom100" => {
            app.set_zoom(1.0);
            Ok(app.status.clone())
        }
        "view.toggleGrid" => {
            app.show_grid = !app.show_grid;
            Ok(format!("Grid: {}", app.show_grid))
        }
        "view.toggleGuides" => {
            app.show_guides = !app.show_guides;
            Ok(format!("Guides: {}", app.show_guides))
        }
        "view.toggleRulers" => {
            app.show_rulers = !app.show_rulers;
            Ok(format!("Rulers: {}", app.show_rulers))
        }
        "theme.dark" => {
            app.set_theme(crate::theme::ThemeMode::Dark);
            Ok(app.status.clone())
        }
        "theme.light" => {
            app.set_theme(crate::theme::ThemeMode::Light);
            Ok(app.status.clone())
        }
        "theme.system" => {
            app.set_theme(crate::theme::ThemeMode::System);
            Ok(app.status.clone())
        }
        other => Err(format!("Unknown command: {other}")),
    }
}

/// Produce a JSON summary of a `Document` (used by `firecraft info` and `firecraft.info` MCP tool).
pub fn summarize_document_json(doc: &Document) -> String {
    let page = doc.page();
    let total_objects = doc.total_objects_on_page();
    let slices = page
        .layers
        .iter()
        .flat_map(|l| &l.objects)
        .filter(|o| matches!(o.kind, crate::doc::Shape::Slice { .. }))
        .count();
    format!(
        "{{\"name\":\"{}\",\"width\":{:.0},\"height\":{:.0},\"dpi\":{},\"pages\":{},\"states\":{},\"layers\":{},\"objects\":{},\"slices\":{},\"symbols\":{},\"exportFormat\":\"{}\"}}",
        doc.name.replace('"', "\\\""),
        page.width,
        page.height,
        doc.dpi,
        doc.pages.len(),
        page.states.len(),
        page.layers.len(),
        total_objects,
        slices,
        doc.symbols.len(),
        doc.optimize.format
    )
}

/// Handle a single JSON-RPC 2.0 MCP request line against `app`, returning the JSON-RPC reply line.
pub fn handle_mcp_jsonrpc(app: &mut FireCraft, line: &str) -> Option<String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.contains("\"notifications/") {
        return None;
    }
    let id = extract_json_id(trimmed);
    let Some(method) = extract_json_str(trimmed, "method") else {
        return Some(format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"error\":{{\"code\":-32600,\"message\":\"Missing method\"}}}}"
        ));
    };

    match method.as_str() {
        "initialize" => Some(format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{\"protocolVersion\":\"{PROTOCOL_VERSION}\",\"capabilities\":{{\"tools\":{{}},\"resources\":{{}}}},\"serverInfo\":{{\"name\":\"firecraft\",\"version\":\"0.2.5\"}}}}}}"
        )),
        "ping" => Some(format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{}}}}"
        )),
        "tools/list" => Some(format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{\"tools\":[\
            {{\"name\":\"firecraft.info\",\"description\":\"Summarize the open FireCraft document as JSON.\",\"inputSchema\":{{\"type\":\"object\",\"properties\":{{}}}}}},\
            {{\"name\":\"firecraft.apply\",\"description\":\"Apply a registered FireCraft command ID to the active document.\",\"inputSchema\":{{\"type\":\"object\",\"properties\":{{\"command\":{{\"type\":\"string\"}}}},\"required\":[\"command\"]}}}},\
            {{\"name\":\"firecraft.runScript\",\"description\":\"Run a FireCraft automation script file (path) or inline semicolon/newline command batch (script).\",\"inputSchema\":{{\"type\":\"object\",\"properties\":{{\"path\":{{\"type\":\"string\"}},\"script\":{{\"type\":\"string\"}}}}}}}},\
            {{\"name\":\"firecraft.export\",\"description\":\"Render and export the active document to a file path.\",\"inputSchema\":{{\"type\":\"object\",\"properties\":{{\"path\":{{\"type\":\"string\"}}}},\"required\":[\"path\"]}}}},\
            {{\"name\":\"firecraft.undo\",\"description\":\"Step back one undo level.\",\"inputSchema\":{{\"type\":\"object\",\"properties\":{{}}}}}},\
            {{\"name\":\"firecraft.redo\",\"description\":\"Step forward one redo level.\",\"inputSchema\":{{\"type\":\"object\",\"properties\":{{}}}}}},\
            {{\"name\":\"firecraft.listCommands\",\"description\":\"List all {} registered command IDs.\",\"inputSchema\":{{\"type\":\"object\",\"properties\":{{}}}}}}\
            ]}}}}",
            COMMAND_CATALOG.len()
        )),
        "resources/list" => Some(format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{\"resources\":[{{\"uri\":\"document://current\",\"name\":\"Current document\",\"mimeType\":\"application/json\"}}]}}}}"
        )),
        "resources/read" => {
            let summary = summarize_document_json(&app.doc).replace('"', "\\\"");
            Some(format!(
                "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{\"contents\":[{{\"uri\":\"document://current\",\"mimeType\":\"application/json\",\"text\":\"{summary}\"}}]}}}}"
            ))
        }
        "tools/call" => {
            let tool_name = extract_json_str(trimmed, "name").unwrap_or_default();
            match tool_name.as_str() {
                "firecraft.info" => {
                    let info = summarize_document_json(&app.doc).replace('"', "\\\"");
                    Some(tool_ok(&id, &info))
                }
                "firecraft.listCommands" => {
                    let list = COMMAND_CATALOG
                        .iter()
                        .map(|(k, _)| *k)
                        .collect::<Vec<_>>()
                        .join("\\n");
                    Some(tool_ok(&id, &list))
                }
                "firecraft.undo" => {
                    app.undo_action();
                    Some(tool_ok(&id, &app.status))
                }
                "firecraft.redo" => {
                    app.redo_action();
                    Some(tool_ok(&id, &app.status))
                }
                "firecraft.apply" => {
                    let cmd = extract_json_str(trimmed, "command").unwrap_or_default();
                    match execute_catalog_command(app, &cmd) {
                        Ok(msg) => Some(tool_ok(&id, &msg)),
                        Err(err) => Some(tool_err(&id, &err)),
                    }
                }
                "firecraft.runScript" => {
                    let script_body = if let Some(p) = extract_json_str(trimmed, "path")
                        && !p.trim().is_empty()
                    {
                        match std::fs::read_to_string(&p) {
                            Ok(s) => s,
                            Err(e) => {
                                return Some(tool_err(&id, &format!("Failed to read {p}: {e}")));
                            }
                        }
                    } else if let Some(s) = extract_json_str(trimmed, "script") {
                        s
                    } else {
                        return Some(tool_err(
                            &id,
                            "firecraft.runScript requires a path or script argument",
                        ));
                    };
                    let mut ran = 0usize;
                    for raw_line in script_body.split(['\n', ';']) {
                        let cmd = raw_line.trim();
                        if cmd.is_empty() || cmd.starts_with('#') || cmd.starts_with("//") {
                            continue;
                        }
                        if let Err(err) = execute_catalog_command(app, cmd) {
                            return Some(tool_err(
                                &id,
                                &format!("Script failed at step {}: {err}", ran + 1),
                            ));
                        }
                        ran += 1;
                    }
                    Some(tool_ok(&id, &format!("Executed {ran} script command(s)")))
                }
                "firecraft.export" => {
                    let path_str = extract_json_str(trimmed, "path").unwrap_or_default();
                    if path_str.is_empty() {
                        return Some(tool_err(&id, "firecraft.export requires a path argument"));
                    }
                    match app.export_to(Path::new(&path_str)) {
                        Ok(()) => Some(tool_ok(&id, &format!("Exported {path_str}"))),
                        Err(e) => Some(tool_err(&id, &e)),
                    }
                }
                other => Some(format!(
                    "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"error\":{{\"code\":-32602,\"message\":\"Unknown tool: {other}\"}}}}"
                )),
            }
        }
        other => Some(format!(
            "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"error\":{{\"code\":-32601,\"message\":\"Unknown method: {other}\"}}}}"
        )),
    }
}

fn tool_ok(id: &str, text: &str) -> String {
    let clean = text.replace('"', "\\\"");
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{\"content\":[{{\"type\":\"text\",\"text\":\"{clean}\"}}],\"isError\":false}}}}"
    )
}

fn tool_err(id: &str, text: &str) -> String {
    let clean = text.replace('"', "\\\"");
    format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":{{\"content\":[{{\"type\":\"text\",\"text\":\"{clean}\"}}],\"isError\":true}}}}"
    )
}

fn extract_json_id(json: &str) -> String {
    if let Some(pos) = json.find("\"id\"") {
        let rest = &json[pos + 4..];
        if let Some(colon) = rest.find(':') {
            let val = rest[colon + 1..].trim_start();
            if let Some(stripped) = val.strip_prefix('"') {
                if let Some(end) = stripped.find('"') {
                    return format!("\"{}\"", &stripped[..end]);
                }
            } else {
                let digits: String = val
                    .chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '-')
                    .collect();
                if !digits.is_empty() {
                    return digits;
                }
            }
        }
    }
    "null".to_string()
}

pub fn extract_json_str(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\"");
    let mut search_from = 0;
    while let Some(pos) = json[search_from..].find(&pattern) {
        let abs = search_from + pos + pattern.len();
        let rest = json[abs..].trim_start();
        if let Some(after_colon) = rest.strip_prefix(':') {
            let val = after_colon.trim_start();
            if let Some(inner) = val.strip_prefix('"')
                && let Some(end) = inner.find('"')
            {
                return Some(inner[..end].to_string());
            }
        }
        search_from = abs;
    }
    None
}

/// Run the headless CLI or MCP stdio loop when a CLI subcommand is passed on the command line.
/// Returns `Some(exit_code)` if a CLI command handled execution, or `None` to launch the GUI.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
pub fn try_run_cli(args: &[String]) -> Option<i32> {
    let sub = args.get(1)?.as_str();
    match sub {
        "--help" | "-h" | "help" => {
            println!("FireCraft Studio v0.2.5 — Desktop GUI, Headless CLI & MCP Server");
            println!("Usage:");
            println!("  firecraft [--demo] [--screenshot <out.png>]     Launch Desktop GUI");
            println!(
                "  firecraft info <file>                           Print document JSON summary"
            );
            println!("  firecraft export <in> <out.(png|jpg|gif|webp|tiff|bmp|ico|svg|html)>");
            println!("  firecraft convert <in> <out>                    Convert between formats");
            println!("  firecraft batch <dir> --format <ext> [--out <dir>]");
            println!(
                "  firecraft apply <command-id> [--out <out-file>] Run a catalog command headless"
            );
            println!(
                "  firecraft list-commands                         List all {} catalog commands",
                COMMAND_CATALOG.len()
            );
            println!(
                "  firecraft mcp [file]                            Start MCP JSON-RPC 2.0 stdio server"
            );
            println!("  firecraft --version                             Print version");
            Some(0)
        }
        "--version" | "-V" | "version" => {
            println!("firecraft 0.2.5 (CS6 Parity + MCP + CLI)");
            Some(0)
        }
        "list-commands" => {
            for (id, desc) in COMMAND_CATALOG {
                println!("{id:<30} {desc}");
            }
            Some(0)
        }
        "info" => {
            let mut app = FireCraft::new(None);
            if let Some(path) = args.get(2) {
                if path == "--demo" {
                    app.load_demo_showcase();
                } else if let Err(e) = app.open_path(Path::new(path)) {
                    eprintln!("firecraft info: {e}");
                    return Some(1);
                }
            } else {
                app.load_demo_showcase();
            }
            println!("{}", summarize_document_json(&app.doc));
            Some(0)
        }
        "export" | "convert" => {
            let (Some(input), Some(output)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: firecraft {sub} <input|--demo> <output>");
                return Some(1);
            };
            let mut app = FireCraft::new(None);
            if input == "--demo" {
                app.load_demo_showcase();
            } else if let Err(e) = app.open_path(Path::new(input)) {
                eprintln!("firecraft {sub}: {e}");
                return Some(1);
            }
            match formats::save(&app.doc, Path::new(output)) {
                Ok(()) => {
                    println!("Exported {} -> {}", input, output);
                    Some(0)
                }
                Err(e) => {
                    eprintln!("firecraft {sub} failed: {e}");
                    Some(1)
                }
            }
        }
        "batch" => {
            let Some(dir) = args.get(2) else {
                eprintln!(
                    "usage: firecraft batch <dir> [--format <png|jpg|webp|svg|html>] [--out <out-dir>]"
                );
                return Some(1);
            };
            let mut fmt_ext = "png".to_string();
            let mut out_dir = PathBuf::from(dir);
            let mut i = 3;
            while i < args.len() {
                match args[i].as_str() {
                    "--format" if i + 1 < args.len() => {
                        fmt_ext = args[i + 1].trim_start_matches('.').to_ascii_lowercase();
                        i += 2;
                    }
                    "--out" if i + 1 < args.len() => {
                        out_dir = PathBuf::from(&args[i + 1]);
                        i += 2;
                    }
                    _ => i += 1,
                }
            }
            let _ = std::fs::create_dir_all(&out_dir);
            let entries = match std::fs::read_dir(dir) {
                Ok(rd) => rd,
                Err(e) => {
                    eprintln!("firecraft batch: cannot read {dir}: {e}");
                    return Some(1);
                }
            };
            let mut count = 0usize;
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file()
                    && let Ok(doc) = formats::open(&p)
                {
                    let stem = p
                        .file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("converted");
                    let dest = out_dir.join(format!("{stem}.{fmt_ext}"));
                    if formats::save(&doc, &dest).is_ok() {
                        count += 1;
                    }
                }
            }
            println!("Batch processed {count} file(s) into {}", out_dir.display());
            Some(0)
        }
        "apply" => {
            let Some(cmd) = args.get(2) else {
                eprintln!("usage: firecraft apply <command-id> [--out <out-file>]");
                return Some(1);
            };
            let mut app = FireCraft::new(None);
            app.load_demo_showcase();
            match execute_catalog_command(&mut app, cmd) {
                Ok(msg) => {
                    if let Some(pos) = args.iter().position(|a| a == "--out")
                        && let Some(out_path) = args.get(pos + 1)
                        && let Err(e) = formats::save(&app.doc, Path::new(out_path))
                    {
                        eprintln!("firecraft apply save failed: {e}");
                        return Some(1);
                    }
                    println!("{msg}");
                    Some(0)
                }
                Err(e) => {
                    eprintln!("firecraft apply: {e}");
                    Some(1)
                }
            }
        }
        "mcp" => {
            let mut app = FireCraft::new(None);
            if let Some(path) = args.get(2) {
                if path == "--demo" {
                    app.load_demo_showcase();
                } else {
                    let _ = app.open_path(Path::new(path));
                }
            } else {
                app.load_demo_showcase();
            }
            let stdin = std::io::stdin();
            let mut stdout = std::io::stdout();
            for line in stdin.lock().lines().map_while(Result::ok) {
                if let Some(reply) = handle_mcp_jsonrpc(&mut app, &line) {
                    let _ = writeln!(stdout, "{reply}");
                    let _ = stdout.flush();
                }
            }
            Some(0)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mcp_jsonrpc_initialize_tools_and_apply() {
        let mut app = FireCraft::new(None);
        let init = handle_mcp_jsonrpc(
            &mut app,
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#,
        )
        .unwrap();
        assert!(init.contains(PROTOCOL_VERSION));

        let tools = handle_mcp_jsonrpc(
            &mut app,
            r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
        )
        .unwrap();
        assert!(tools.contains("firecraft.apply"));
        assert!(tools.contains("firecraft.info"));

        let call = handle_mcp_jsonrpc(
            &mut app,
            r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"firecraft.apply","arguments":{"command":"document.demoShowcase"}}}"#,
        )
        .unwrap();
        assert!(call.contains("\"isError\":false"));
        assert!(app.doc.total_objects_on_page() >= 6);

        let info = handle_mcp_jsonrpc(
            &mut app,
            r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"firecraft.info"}}"#,
        )
        .unwrap();
        assert!(info.contains("width"));
    }
}
