//! End-to-end tests for the headless CLI: real process, real files, real exit codes.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn bin() -> Command {
    Command::new(env!("CARGO_BIN_EXE_firecraft"))
}

fn run(args: &[&str]) -> Output {
    bin()
        .args(args)
        .output()
        .unwrap_or_else(|e| panic!("failed to run firecraft {args:?}: {e}"))
}

fn stdout(out: &Output) -> String {
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        let mut path = std::env::temp_dir();
        path.push(format!(
            "firecraft-test-{tag}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("create temp dir");
        Self(path)
    }

    fn join(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn assert_magic(path: &Path, magic: &[u8]) {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("read {path:?}: {e}"));
    assert!(
        bytes.starts_with(magic),
        "{path:?} does not start with {magic:?} (got {:?})",
        &bytes[..bytes.len().min(8)]
    );
}

#[test]
fn version_reports_the_crate_version() {
    let out = run(&["--version"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    assert!(
        text.contains(env!("CARGO_PKG_VERSION")),
        "version output {text:?} does not contain {}",
        env!("CARGO_PKG_VERSION")
    );
    assert!(!text.contains("0.2.5"), "stale hardcoded version: {text:?}");
}

#[test]
fn help_lists_every_cli_verb() {
    let out = run(&["--help"]);
    assert!(out.status.success());
    let text = stdout(&out);
    for verb in [
        "info",
        "export",
        "convert",
        "batch",
        "apply",
        "list-commands",
        "mcp",
        "--version",
    ] {
        assert!(text.contains(verb), "help does not mention {verb}");
    }
}

#[test]
fn list_commands_prints_a_unique_catalog() {
    let out = run(&["list-commands"]);
    assert!(out.status.success());
    let text = stdout(&out);
    let ids: Vec<&str> = text
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| l.split_whitespace().next().unwrap_or(""))
        .collect();
    assert!(ids.len() >= 60, "catalog has only {} entries", ids.len());
    let mut unique = ids.clone();
    unique.sort_unstable();
    unique.dedup();
    assert_eq!(unique.len(), ids.len(), "catalog contains duplicate ids");
}

#[test]
fn info_demo_prints_the_showcase_geometry() {
    let out = run(&["info", "--demo"]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    let text = stdout(&out);
    assert!(
        text.starts_with('{') && text.trim_end().ends_with('}'),
        "{text}"
    );
    assert!(text.contains("\"width\":1920"), "{text}");
    assert!(text.contains("\"height\":1080"), "{text}");
    assert!(text.contains("\"dpi\":144"), "{text}");
    assert!(text.contains("\"objects\":"), "{text}");
}

#[test]
fn info_missing_file_fails_with_a_message() {
    let dir = TempDir::new("info");
    let out = run(&["info", dir.join("nope.fw.png").to_str().unwrap()]);
    assert!(!out.status.success());
    assert!(stderr(&out).contains("firecraft info"));
}

#[test]
fn every_export_format_produces_a_real_file() {
    let dir = TempDir::new("export");
    let cases: &[(&str, &[u8])] = &[
        ("png", b"\x89PNG\r\n\x1a\n"),
        ("jpg", b"\xff\xd8\xff"),
        ("gif", b"GIF8"),
        ("webp", b"RIFF"),
        ("bmp", b"BM"),
        ("tiff", b"II"),
        ("ico", b"\x00\x00\x01\x00"),
    ];
    for (ext, magic) in cases {
        let path = dir.join(&format!("showcase.{ext}"));
        let out = run(&["export", "--demo", path.to_str().unwrap()]);
        assert!(
            out.status.success(),
            "{ext} export failed: {}",
            stderr(&out)
        );
        assert_magic(&path, magic);
    }

    let svg = dir.join("showcase.svg");
    assert!(
        run(&["export", "--demo", svg.to_str().unwrap()])
            .status
            .success()
    );
    let svg_text = std::fs::read_to_string(&svg).unwrap();
    assert!(svg_text.contains("<svg"), "{svg_text:.200}");
    assert!(svg_text.contains("viewBox"), "SVG has no viewBox");

    let html = dir.join("showcase.html");
    assert!(
        run(&["export", "--demo", html.to_str().unwrap()])
            .status
            .success()
    );
    let html_text = std::fs::read_to_string(&html).unwrap();
    assert!(html_text.contains("<html"), "{html_text:.200}");
}

#[test]
fn unsupported_output_extension_fails() {
    let dir = TempDir::new("bad-ext");
    let out = run(&[
        "export",
        "--demo",
        dir.join("showcase.xyz").to_str().unwrap(),
    ]);
    assert!(!out.status.success(), "unsupported extension was accepted");
    assert!(!dir.join("showcase.xyz").exists());
}

#[test]
fn export_without_arguments_fails() {
    let out = run(&["export"]);
    assert!(!out.status.success());
    assert!(stderr(&out).contains("usage"));
}

#[test]
fn apply_unknown_command_fails_and_known_command_succeeds() {
    let dir = TempDir::new("apply");
    let out = run(&["apply", "no-such-command"]);
    assert!(!out.status.success());
    assert!(stderr(&out).contains("firecraft apply"));

    let list = stdout(&run(&["list-commands"]));
    let first = list
        .lines()
        .find(|l| !l.trim().is_empty())
        .and_then(|l| l.split_whitespace().next())
        .expect("catalog is not empty");
    let out = run(&["apply", first]);
    assert!(
        out.status.success(),
        "apply {first} failed: {}",
        stderr(&out)
    );
    assert!(!stdout(&out).trim().is_empty());

    let saved = dir.join("applied.png");
    let out = run(&["apply", first, "--out", saved.to_str().unwrap()]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert_magic(&saved, b"\x89PNG");
}

#[test]
fn convert_png_to_svg_and_back_keeps_a_document() {
    let dir = TempDir::new("convert");
    let png = dir.join("in.png");
    assert!(
        run(&["export", "--demo", png.to_str().unwrap()])
            .status
            .success()
    );
    let svg = dir.join("out.svg");
    let out = run(&["convert", png.to_str().unwrap(), svg.to_str().unwrap()]);
    assert!(out.status.success(), "convert failed: {}", stderr(&out));
    assert!(std::fs::read_to_string(&svg).unwrap().contains("<svg"));

    let info = run(&["info", svg.to_str().unwrap()]);
    assert!(
        info.status.success(),
        "info on converted svg failed: {}",
        stderr(&info)
    );
    assert!(stdout(&info).contains("\"width\""));
}

#[test]
fn batch_converts_a_directory_and_reports_the_count() {
    let dir = TempDir::new("batch");
    let src = dir.join("src");
    let dst = dir.join("dst");
    std::fs::create_dir_all(&src).unwrap();
    for name in ["a.png", "b.png"] {
        assert!(
            run(&["export", "--demo", src.join(name).to_str().unwrap()])
                .status
                .success()
        );
    }
    let out = run(&[
        "batch",
        src.to_str().unwrap(),
        "--format",
        "webp",
        "--out",
        dst.to_str().unwrap(),
    ]);
    assert!(out.status.success(), "stderr: {}", stderr(&out));
    assert!(stdout(&out).contains("Batch processed 2 file(s)"));
    assert_magic(&dst.join("a.webp"), b"RIFF");
    assert_magic(&dst.join("b.webp"), b"RIFF");
}

#[test]
fn batch_on_a_missing_directory_fails() {
    let dir = TempDir::new("batch-missing");
    let out = run(&["batch", dir.join("absent").to_str().unwrap()]);
    assert!(!out.status.success());
    assert!(stderr(&out).contains("cannot read"));
}
