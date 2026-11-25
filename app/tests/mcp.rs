//! End-to-end tests for the MCP JSON-RPC 2.0 server over stdio.

use std::io::Write;
use std::process::{Command, Stdio};

/// Feed `lines` to `firecraft mcp --demo` and return the reply lines.
fn session(lines: &[&str]) -> Vec<String> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_firecraft"))
        .args(["mcp", "--demo"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn mcp server");

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        for line in lines {
            writeln!(stdin, "{line}").expect("write request");
        }
    }
    let out = child.wait_with_output().expect("mcp exit");
    assert!(
        out.status.success(),
        "mcp exited with {:?}: {}",
        out.status,
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(str::to_string)
        .collect()
}

#[test]
fn initialize_reports_protocol_and_server_version() {
    let replies = session(&[r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#]);
    assert_eq!(replies.len(), 1, "{replies:?}");
    let reply = &replies[0];
    assert!(reply.contains("\"protocolVersion\""), "{reply}");
    assert!(reply.contains("\"name\":\"firecraft\""), "{reply}");
    assert!(
        reply.contains(env!("CARGO_PKG_VERSION")),
        "serverInfo version should match the crate: {reply}"
    );
    assert!(reply.contains("\"id\":1"), "{reply}");
}

#[test]
fn notifications_get_no_reply() {
    let replies = session(&[
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/cancelled","params":{}}"#,
        r#"{"jsonrpc":"2.0","id":7,"method":"ping"}"#,
    ]);
    assert_eq!(replies.len(), 1, "notifications must not produce replies: {replies:?}");
    assert!(replies[0].contains("\"id\":7"));
}

#[test]
fn tools_list_covers_the_documented_tools() {
    let replies = session(&[r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#]);
    let reply = &replies[0];
    for tool in [
        "firecraft.info",
        "firecraft.apply",
        "firecraft.runScript",
        "firecraft.export",
        "firecraft.undo",
        "firecraft.redo",
        "firecraft.listCommands",
    ] {
        assert!(reply.contains(tool), "missing {tool} in {reply}");
    }
    assert!(reply.contains("\"inputSchema\""), "{reply}");
}

#[test]
fn apply_and_info_tools_work_against_the_showcase() {
    let replies = session(&[
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"firecraft.apply","arguments":{"command":"document.demoShowcase"}}}"#,
        r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"firecraft.info"}}"#,
    ]);
    assert_eq!(replies.len(), 2, "{replies:?}");
    assert!(replies[0].contains("\"isError\":false"), "{}", replies[0]);
    assert!(replies[1].contains("width"), "{}", replies[1]);
    assert!(replies[1].contains("1920"), "{}", replies[1]);
}

#[test]
fn unknown_tool_is_a_jsonrpc_error() {
    let replies = session(&[
        r#"{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"firecraft.nope"}}"#,
    ]);
    assert!(replies[0].contains("-32602"), "{}", replies[0]);
    assert!(replies[0].contains("Unknown tool"), "{}", replies[0]);
}

#[test]
fn unknown_method_is_a_jsonrpc_error() {
    let replies = session(&[r#"{"jsonrpc":"2.0","id":6,"method":"does/not/exist"}"#]);
    assert!(replies[0].contains("-32601"), "{}", replies[0]);
}

#[test]
fn malformed_request_without_method_is_rejected() {
    let replies = session(&[
        r#"{"jsonrpc":"2.0","id":8,"params":{}}"#,
        "not json at all",
        r#"{"jsonrpc":"2.0","id":9,"method":"ping"}"#,
    ]);
    // The server is line-oriented and must stay alive after junk.
    assert!(replies.len() >= 2, "{replies:?}");
    assert!(replies[0].contains("-32600"), "{}", replies[0]);
    assert!(replies.last().unwrap().contains("\"id\":9"), "{replies:?}");
}

#[test]
fn export_tool_writes_a_real_png() {
    let dir = std::env::temp_dir().join(format!("firecraft-mcp-export-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join("out.png");
    let request = format!(
        r#"{{"jsonrpc":"2.0","id":10,"method":"tools/call","params":{{"name":"firecraft.export","arguments":{{"path":"{}"}}}}}}"#,
        path.display()
    );
    let replies = session(&[&request]);
    assert!(replies[0].contains("\"isError\":false"), "{}", replies[0]);
    let bytes = std::fs::read(&path).expect("exported file");
    assert!(bytes.starts_with(b"\x89PNG"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn script_tool_runs_each_line_and_reports_the_count() {
    let replies = session(&[
        r#"{"jsonrpc":"2.0","id":11,"method":"tools/call","params":{"name":"firecraft.runScript","arguments":{"script":"document.demoShowcase;edit.undo"}}}"#,
    ]);
    assert!(replies[0].contains("Executed 2 script command(s)"), "{}", replies[0]);
}

#[test]
fn resources_read_returns_the_document_summary() {
    let replies = session(&[r#"{"jsonrpc":"2.0","id":12,"method":"resources/read","params":{}}"#]);
    assert!(replies[0].contains("document://current"), "{}", replies[0]);
    assert!(replies[0].contains("\\\"width\\\""), "{}", replies[0]);
}
