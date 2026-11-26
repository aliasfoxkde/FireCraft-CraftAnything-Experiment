# Atheon pattern scan triage

**Date:** 2026-11-26  
**Scope:** `app/src`  
**Input:** `/tmp/firecraft-atheon-scan.json` (Atheon-Enhanced pattern scan)  
**Disposition:** Initial manual triage of every distinct critical/high rule, using the scan's reported matches and representative source context. This is not a clean bill of health or a substitute for a dedicated security review.

## Summary

The scan reported 3,747 matches in `app/src`: 83 critical, 75 high, 1,383 medium, and 2,206 low (risk score is scanner-derived). All critical/high results fall into seven rule patterns. Most are clear heuristic mismatches: the scanner masks snippets so aggressively that ordinary Rust tokens such as `match`, `Rect`, `self`, and UI strings appear as SQL or package names. The unsafe-code rule correctly identifies unsafe Rust and therefore remains a review signal even though the project intentionally confines Windows COM interop to one module. The contact-data rules point to an intentional support email and a Windows COM GUID, respectively.

The findings are triaged below, not silently suppressed. No source or scanner rules were changed as part of this triage. Medium/low findings were not individually reviewed.

## Critical and high findings

| Severity | Rule / category | Count | Triage | Evidence and next action |
|---|---|---:|---|---|
| Critical | `sql-concatenation` | 83 | **Likely false positive (high confidence)** | Representative masked matches include `self...`, `Rect...`, `egui...`, and UI event checks across `app.rs`, `icons.rs`, `canvas.rs`, `menus.rs`, and other modules. `app/src` contains find/replace query strings, but these are in-memory graphics object text searches; no SQL engine or SQL client dependency is declared in `app/Cargo.toml`. No SQL statement construction was found in the inspected source. **Action:** improve scanner language/context filtering and rerun; separately investigate any future match that includes an actual SQL API or statement. |
| High | `typosquat-common-package-names` | 44 | **Likely false positive (high confidence)** | Matches are ordinary Rust identifiers, keywords, comments, and UI labels (examples include `Open`, `Close`, `Drag`, and `Create`). The rule is being applied to source tokens rather than dependency coordinates. Cargo dependencies are declared in manifests, and no suspicious package declaration is evidenced by these source matches. **Action:** scan Cargo manifests/lockfile with ecosystem-aware package-name validation; do not treat ordinary source words as package installs. |
| High | `rust-unsafe-block` | 19 | **Real code requiring focused review; intentional boundary** | All results are in `app/src/shell_windows.rs`, which explicitly documents the crate's `unsafe_code` denial and this module as the reviewed COM exception under ADR-0002. The module uses Windows COM shell APIs for AppUserModelID and Jump Lists; errors are propagated/logged. The unsafe syntax is real, not a scanner false positive. This scan does not prove pointer/COM lifecycle safety. **Action:** review COM initialization apartment/thread lifetime, HRESULT handling (including `CoInitializeEx`), interface ownership, and each API's safety contract; retain the narrow module boundary and add targeted Windows validation/tests if gaps are found. |
| High | `leaderboard-sql-injection` | 7 | **Likely false positive (high confidence)** | Representative matches are ordinary Rust `match` expressions and format strings in `app.rs`, `colors.rs`, `demo.rs`, and `harness.rs`; the code is not a leaderboard or a database query path. No SQL dependencies or leaderboard implementation were found in the relevant source. **Action:** disable/reconfigure this rule for Rust source or require a database/query API context before escalating. |
| High | `mcp-exfil-pattern` | 3 | **Likely false positive in sampled matches; verify on rule-aware rerun** | Matches are ordinary condition checks in `canvas.rs`, `layout.rs`, and `menus.rs`; the scanner's snippets do not show credential access, network transmission, or an exfiltration sink. `app/src/mcp.rs` implements an MCP stdio server and explicit document tools, including export to a caller-provided path, so MCP-related code deserves a separate threat-model review; the mere presence of MCP is not evidence of exfiltration. **Action:** inspect full-context matches after scanner output can retain context; review MCP tool authorization/path behavior and ensure no unrequested network or secret access. |
| High | `email-address` | 1 | **Expected/intentional** | `app/src/panels/mod.rs:951` contains the support link `mailto:hello@firecraft.app`. **Action:** confirm this is the intended public contact address and that it is safe to publish; then classify it as an approved public contact datum in the scanner workflow if that workflow supports contextual dispositions. |
| High | `phone-number` | 1 | **False positive (high confidence)** | `app/src/shell_windows.rs:26` is the tail of `CLSID_SHELL_LINK`'s GUID literal `0x00021401_0000_0000_c000_000000000046`, not a phone number. **Action:** use GUID/context-aware detection or exclude this literal shape from phone-number alerts. |

### Reading the result counts

The counts above are scanner matches, not counts of confirmed vulnerabilities. In particular, the 83 critical SQL-labelled matches should not be interpreted as 83 SQL injection defects. Automated masking obscures the evidence, and pattern names alone are insufficient for a vulnerability claim. Only the unsafe blocks are confirmed instances of the construct named by the rule; that is an audit signal, not proof of a defect. The email is a visible public contact address. The phone rule is demonstrably triggered by a GUID suffix.

## Aegis status

The required Aegis profile could not be run in this environment. The job-manager admission hook required the managed profile named `aegis-scan`, but the configured job manager did not expose that profile. The available `aegis-workspace-tests` profile is scoped to the Aegis workspace and runs its Cargo test workflow; it is not a FireCraft pattern-scanning profile, so substituting it would not validate this repository. Consequently, **Aegis coverage is unavailable for this audit**. No Aegis-clean claim is made. The Atheon scan did run and produced the findings summarized above.

## Follow-up

1. Run Atheon again after configuring Rust-aware rules or scoped exclusions, preserving raw full-context evidence for any security-sensitive match.
2. Have a reviewer inspect the Windows COM safety boundary and MCP tool behavior as focused manual reviews; this triage is not a sign-off for either.
3. Provide a correctly scoped Aegis job-manager profile and rerun it against this repository. Record profile identity, source revision, command/job ID, and raw result before updating status.
4. Reconcile all categories, including medium/low, in a separate pass if a complete scanner disposition is required. This document covers only critical/high rules.
