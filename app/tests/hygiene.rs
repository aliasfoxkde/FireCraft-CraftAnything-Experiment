//! Repository guard rails that a unit test cannot express by exercising a function.
//!
//! These run against the source tree itself, so a regression fails the suite even if every
//! behavior test still passes.

use std::path::{Path, PathBuf};

fn src_files() -> Vec<PathBuf> {
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).expect("read src dir") {
            let path = entry.expect("dir entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|e| e == "rs") {
                out.push(path);
            }
        }
    }
    let mut files = Vec::new();
    walk(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    files
}

#[test]
fn no_placeholders_or_unimplemented_paths() {
    let mut offenders = Vec::new();
    for file in src_files() {
        let text = std::fs::read_to_string(&file).expect("read source");
        for (line_no, line) in text.lines().enumerate() {
            for needle in ["todo!(", "unimplemented!(", "TODO", "FIXME", "XXX:"] {
                if line.contains(needle) {
                    offenders.push(format!("{}:{}: {needle}", file.display(), line_no + 1));
                }
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "placeholders found:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn unsafe_is_confined_to_the_windows_shell_module() {
    let mut offenders = Vec::new();
    for file in src_files() {
        if file.file_name().is_some_and(|n| n == "shell_windows.rs") {
            continue;
        }
        let text = std::fs::read_to_string(&file).expect("read source");
        for (line_no, line) in text.lines().enumerate() {
            if line.contains("unsafe ") || line.contains("unsafe{") {
                offenders.push(format!(
                    "{}:{}: {}",
                    file.display(),
                    line_no + 1,
                    line.trim()
                ));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "unsafe outside shell_windows.rs:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn no_hardcoded_release_version_in_sources() {
    let mut offenders = Vec::new();
    for file in src_files() {
        let text = std::fs::read_to_string(&file).expect("read source");
        for (line_no, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with("//!") {
                continue;
            }
            // A literal "0.2.5"-style release string is a second version source.
            if trimmed.contains("\"0.2.") || trimmed.contains("\"0.3.") {
                offenders.push(format!("{}:{}: {trimmed}", file.display(), line_no + 1));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "hardcoded version literals (use env!(\"CARGO_PKG_VERSION\")):\n{}",
        offenders.join("\n")
    );
}

#[test]
fn tests_do_not_reference_real_user_directories() {
    let mut offenders = Vec::new();
    for file in src_files() {
        let text = std::fs::read_to_string(&file).expect("read source");
        for (line_no, line) in text.lines().enumerate() {
            if line.contains("dirs::home_dir")
                || line.contains("\"/home/")
                || line.contains("\"/Users/")
            {
                offenders.push(format!(
                    "{}:{}: {}",
                    file.display(),
                    line_no + 1,
                    line.trim()
                ));
            }
        }
    }
    assert!(
        offenders.is_empty(),
        "tests and code must use temp dirs, not real user paths:\n{}",
        offenders.join("\n")
    );
}
