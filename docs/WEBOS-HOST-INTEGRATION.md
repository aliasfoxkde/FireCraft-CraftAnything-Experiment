# FireCraft ↔ ArtCraft WebOS Host Integration Contract

**Status:** Proposed protocol contract; implementation not yet present in FireCraft main
**Prepared:** 2026-11-27
**FireCraft repository:** aliasfoxkde/FireCraft-CraftAnything-Experiment
**Host repository:** aliasfoxkde/ArtCraft-WebOS
**Related plans:** [Quality & WebOS Roadmap](QUALITY-AND-WEBOS-ROADMAP-2026-11-27.md) · [WebOS Package Runtime Handoff](https://github.com/aliasfoxkde/ArtCraft-WebOS/blob/docs/firecraft-wasm-package-runtime-roadmap-2026-11-27/docs/planning/FIRECRAFT-WASM-PACKAGE-AND-RUNTIME-HANDOFF-2026-11-27.md)

This contract describes a proposed boundary between FireCraft's Rust/WASM editor and a WebOS host. It is not an implementation claim. The host remains a separate project; this file defines what a future FireCraft adapter could expect from that host and what it must never assume. No matching host protocol implementation exists in the current FireCraft source. Treat this as a draft: coordinate and version it with the host project before implementation or advertising support.

---

## 1. Contract principles

1. **Standalone first:** FireCraft must work in a normal browser without WebOS. Host integration is optional.
2. **Host owns privileges:** WebOS controls storage, user file pickers, clipboard access, notifications, app lifecycle and app permissions.
3. **Typed requests only:** FireCraft never sends arbitrary shell commands, filesystem paths outside its app root, HTML to execute, or generic “run this code” requests.
4. **Capability-scoped:** every privileged operation is associated with a capability negotiated at launch.
5. **MessagePort-based session:** the long-lived channel is a per-launch MessagePort transferred by the host, not a global event listener trusting a source string.
6. **Versioned and bounded:** every message has a schema version, request ID where appropriate, size limits, timeout/cancellation and explicit errors.
7. **No wildcard data transfer:** document bytes, file contents, user preferences and private metadata are sent only through the negotiated channel, never to a wildcard postMessage target.
8. **No fabricated durability:** FireCraft reports “saved” only after the host confirms durable completion.
9. **Reversible:** host capability denial, disconnect, timeout or app shutdown must not corrupt the open document.
10. **Auditable:** messages and state transitions can be traced to app version, package digest, host API version and session ID without logging document contents.

## 2. Runtime detection and handshake

### 2.1 Standalone browser

When FireCraft is not embedded in a trusted host, it uses its existing standalone browser behavior:
- browser file picker/input for open/import;
- download flow for save/export;
- browser clipboard APIs subject to browser permissions and user gestures;
- local browser storage only for settings and explicit browser-mode persistence;
- no assumption that WebOS storage or host APIs exist.

A missing host or rejected handshake is not a startup failure. The editor continues in standalone mode and reports that host integration is unavailable.

### 2.2 WebOS package mode

The WebOS package runtime must verify the package manifest and establish a sandboxed frame before FireCraft is initialized. Recommended direction:
- sandbox scripts but do not grant same-origin access to the WebOS shell;
- create a fresh MessageChannel per app launch;
- transfer one port to the FireCraft frame during a one-use bootstrap;
- use a random, cryptographically generated session nonce tied to that package launch;
- bind the port to the iframe/window lifecycle and revoke it on navigation, reload, unmount, uninstall or permission revocation.

For a sandbox with an opaque origin, event.origin may be "null"; do not try to treat that string as an identity. The initial port transfer is a short-lived bootstrap capability. It must carry no document data. All subsequent privileged operations use the private port and the negotiated session.

The host must not use source-marker strings as authentication. The iframe's WindowProxy identity helps bind a frame, but origin validation or a MessagePort capability is still required.

### 2.3 Handshake sequence

1. WebOS verifies package source, release tag, source SHA, asset digest, file inventory, package schema and host API compatibility.
2. WebOS creates a sandboxed iframe and a new MessageChannel.
3. WebOS transfers one port plus a one-time nonce and a minimal host.init envelope.
4. FireCraft verifies the expected app ID, supported protocol major, session nonce, schema and capability list.
5. FireCraft sends app.hello with its app version, build/source identity and supported protocol range.
6. WebOS returns host.ready with negotiated protocol version, granted capabilities, max payload size and app-scoped storage identity.
7. FireCraft initializes the host client and emits app.ready.
8. On any mismatch, timeout or duplicate handshake, the host closes the port and FireCraft falls back to standalone behavior or shows a clear host compatibility error.

Never transfer the user's saved document as part of handshake. Restore is an explicit typed operation after host.ready.

---

## 3. Envelope and correlation

All port messages use a JSON envelope with transferable binary payloads where necessary. Exact JSON Schema files should be checked into both repositories and tested against the same fixtures.

Required envelope fields:
- protocol: major/minor protocol identifier;
- type: allowlisted message name;
- sessionId: random per-launch identity;
- requestId: random per-request identity for request/response operations;
- payload: operation-specific schema;
- transfer: optional ArrayBuffer/Blob objects transferred through MessagePort, not embedded base64 strings.

The host validates every message before dispatch:
- protocol major is supported;
- session ID matches the active app instance;
- request ID is unique and not already completed/cancelled;
- type is allowlisted;
- capability was granted for the current app/session;
- payload and transfer sizes are within bounds;
- paths, MIME types, filenames and document schema are validated;
- no unsupported extra privilege is inferred from a message field.

FireCraft validates all host responses with the same rigor. A malformed response, wrong session or unknown request ID is discarded and reported as a protocol failure.

Use a short documented initial maximum payload (64 MiB to align with the existing WebOS file bridge) and explicit size errors. Add chunked/streaming transfer only after memory and cancellation behavior are tested. Never silently truncate a document or image.

## 4. Initial message types

### Lifecycle and negotiation

| Type | Direction | Purpose |
|---|---|---|
| host.init | host → app | One-time session bootstrap; transfers the MessagePort |
| app.hello | app → host | App ID/version/build and supported protocol range |
| host.ready | host → app | Negotiated version, capability grants and limits |
| app.ready | app → host | Editor initialized and ready for host operations |
| host.suspend / host.resume | host → app | App lifecycle visibility/background changes |
| app.shutdown | app → host | Graceful shutdown request/acknowledgement |
| host.permission.changed | host → app | Capability granted/revoked during session |

### Storage and document operations

| Type | Direction | Capability | Purpose |
|---|---|---|---|
| host.storage.list | app → host | app.storage.read | List entries in the FireCraft app-private storage root |
| host.storage.read | app → host | app.storage.read | Read an app-private file |
| host.storage.write | app → host | app.storage.write | Write an app-private file; response confirms durability |
| host.storage.delete | app → host | app.storage.write | Delete an app-private file |
| host.file.open | app → host | files.open | Request user-mediated file selection/import |
| host.file.save | app → host | files.save | Request user-mediated save/export dialog |
| host.document.recovery.write | app → host | app.storage.write | Write a versioned recovery snapshot |
| host.document.recovery.read | app → host | app.storage.read | Read a recovery snapshot after startup/consent |

Paths for storage.list/read/write/delete are always relative to the app's private root. A request cannot supply an arbitrary absolute system path. User file picker results are one-time transferred data or an opaque host-issued handle, not permission to enumerate the user's disk.

### Optional host APIs

| Type | Direction | Capability | Notes |
|---|---|---|---|
| host.clipboard.read | app → host | clipboard.read | Requires user gesture and current permission |
| host.clipboard.write | app → host | clipboard.write | Requires user gesture and explicit browser/host permission |
| host.notification.create | app → host | notifications.create | User-visible completion/error notifications |
| host.window.request | app → host | window.lifecycle | Only allowlisted window actions; no arbitrary shell command |

Do not implement a broad generic command-run, filesystem, process, network-proxy or user-account API.

## 5. Error and cancellation contract

Every request returns one of:
- ok with a schema-validated result;
- denied with capability and user-action explanation;
- cancelled;
- timeout;
- unavailable;
- invalid-request;
- too-large;
- storage-error;
- incompatible-version;
- internal-error with a redacted diagnostic ID.

The app must preserve document state on denied, cancelled, timeout and unavailable errors. The host must not return partial data as a successful file/document write.

Requests are cancellable. On app shutdown or frame navigation, the host cancels pending operations and closes the port. FireCraft stops outstanding operations and does not send new requests through a stale session.

Retry policy:
- read operations may be retried after a new handshake if the user action remains valid;
- writes need idempotency keys or a queryable completion record to prevent duplicate writes;
- user file picker/save requests must not reopen repeatedly after a timeout without an explicit user action.

## 6. Initial capability profile

The first FireCraft integration should request the smallest useful profile:

| Capability | Initial policy | Details |
|---|---|---|
| app.storage.read | Grant within app root | Read app settings and recovery data |
| app.storage.write | Grant within app root | Save recovery/project files to app root |
| files.open | User-mediated | Host picker for opening/importing files |
| files.save | User-mediated | Host save dialog for export/save |
| clipboard.read | User gesture / permission | Clipboard data only; no background polling |
| clipboard.write | User gesture / permission | User-initiated copy/export |
| notifications.create | Optional / ask | Visible operation status only |
| window.lifecycle | Minimal grant | Ready, suspend, resume, close |
| network.fetch | Deny by default | No general web proxy/network privilege |

A future capability must have a documented user scenario, privacy/security rationale, permission prompt, tests and revocation path. Experimental features do not implicitly receive new capabilities.

## 7. Compatibility and document safety

- Protocol uses major/minor compatibility. Unknown major versions fail closed; unknown optional minor fields may be ignored only where the schema explicitly permits them.
- Each package advertises minimum and maximum supported host API.
- Host API migration is tested against the previous supported FireCraft release and current candidate.
- Document schema read/write compatibility is independent of host API version.
- Rollback of the app binary must not assume the document was rolled back.
- Recovery snapshots are versioned, checksummed and written atomically by the host.
- If a new app writes a document schema that an older stable package cannot read, that new feature must remain Experimental and must either use a backward-compatible representation or require an explicit backup/export before enabling it.
- A failed host write must leave the current document dirty and offer retry/save-as; it must not clear the dirty indicator.

## 8. Required conformance tests

Both repositories should consume shared JSON fixtures or a small protocol test package. FireCraft tests the client; WebOS tests the host. Required cases:
1. successful handshake and protocol negotiation;
2. standalone launch with no host;
3. unsupported major version and too-old host;
4. wrong app ID, stale nonce/session ID and replayed request ID;
5. capability grant, denial and revocation;
6. storage path traversal, absolute path, wrong app root and oversized transfer;
7. file picker cancel, denied permission and successful open/save;
8. autosave durability acknowledgement and recovery;
9. malformed payload, duplicate/unknown response, timeout and cancellation;
10. frame reload/navigation, stale MessagePort and shutdown;
11. hostile frame attempting to spoof ready/export or obtain saved document bytes;
12. package digest/provenance mismatch and package ABI mismatch;
13. WebGL2 startup failure with visible recovery/standalone error;
14. offline standalone use and WebOS cached package launch.

**Contract exit:** both repositories pass the same compatibility fixtures; no test relies on a public hosted FireCraft URL for required local host integration.

## 9. Implementation ownership and sequence

- FireCraft implements the Rust/WASM host client, standalone fallback, schemas/fixtures, package manifest and release asset.
- WebOS implements the trusted bootstrap, MessageChannel lifecycle, capability broker, app-private storage, user file dialogs, sandbox, package catalog/install/cache, channel pointers and rollback.
- The FireCraft PR must not include WebOS source or update WebOS registry versions.
- The WebOS PR must not copy FireCraft Rust or editor modules.
- Package adoption happens only after a build from a published FireCraft source SHA passes the WebOS local-package fixture and compatibility matrix.

The detailed joint implementation sequence is in the [FireCraft Quality & WebOS Roadmap](QUALITY-AND-WEBOS-ROADMAP-2026-11-27.md) and [WebOS Runtime Handoff](https://github.com/aliasfoxkde/ArtCraft-WebOS/blob/docs/firecraft-wasm-package-runtime-roadmap-2026-11-27/docs/planning/FIRECRAFT-WASM-PACKAGE-AND-RUNTIME-HANDOFF-2026-11-27.md).
