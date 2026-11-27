//! Repository guard rails that a unit test cannot express by exercising a function.
//!
//! These run against the source tree itself, so a regression fails the suite even if every
//! behavior test still passes.

use std::path::{Path, PathBuf};

#[cfg(unix)]
#[test]
fn docs_gate_fails_when_git_cannot_list_tracked_markdown() {
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;

    let temp = std::env::temp_dir().join(format!(
        "firecraft-docs-git-failure-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos()
    ));
    let bin = temp.join("bin");
    std::fs::create_dir_all(&bin).expect("create fake git directory");
    let fake_git = bin.join("git");
    std::fs::write(&fake_git, "#!/bin/sh\nexit 128\n").expect("write fake git");
    let mut permissions = std::fs::metadata(&fake_git)
        .expect("stat fake git")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake_git, permissions).expect("make fake git executable");

    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = manifest.parent().expect("app has repository parent");
    let script = repo_root.join("scripts/check-docs.sh");
    let original_path = std::env::var_os("PATH").unwrap_or_default();
    let path =
        std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(&original_path)))
            .expect("construct fake git PATH");
    let output = Command::new("bash")
        .arg(script)
        .env("PATH", path)
        .output()
        .expect("run docs gate with failing git");

    std::fs::remove_dir_all(&temp).expect("remove fake git directory");
    assert!(
        !output.status.success(),
        "docs gate silently accepted an empty tracked-file list: {}",
        String::from_utf8_lossy(&output.stdout)
    );
}

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
