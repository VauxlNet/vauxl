# Client Stack Post-Mortem

Written 2026-07-06 from the 2026-05-21 retrospective. This document is a guardrail: no further client stack change may be proposed without updating it and naming what changed.

## History

Roughly four client attempts preceded the current Tauri v2 client, each abandoned early (a handful of source files) and archived in the `*-legacy-v1` GitHub repos and local archive bundles:

1. Flutter (the original PRD stack: Rust core, Flutter UI)
2. Slint
3. An earlier Tauri prototype
4. Electrobun (TS/React, web frontend)

## What actually killed them

Not raw native performance. UI and UX polish. With Flutter, Slint, and the early Tauri attempt, the UI never felt clean, intuitive, or modern; it consistently came out dated and unpolished in this team's hands. Electrobun got closest to the intended feel precisely because the frontend was web.

The performance complaint against Electrobun deserves a footnote: that configuration ran in system-webview mode (bundleCEF disabled), so the sluggishness observed was most likely the WebKitGTK-on-Linux system webview, not the shell architecture itself.

## Conclusions

- Demonstrated strength: web UI. Demonstrated weakness: native-widget toolkits.
- Performance is a measurable, deferrable knob. The project has lost four times on polish and shipping, and zero times on RAM usage. With zero users, performance cannot be the binding constraint.
- The performance lever, when it is ever needed, is pushing sync, crypto, and search into the Rust core behind the decoupled interface, not changing the UI toolkit.

## Standing rules

1. The frontend is web. Rewriting the UI in a native toolkit is the known failure mode.
2. Domain and Matrix logic stay decoupled from the shell (see `PROTOCOL_AGNOSTIC_ARCHITECTURE.md` and `client/docs/DATA_INTERFACE.md`), so the shell choice stays reversible and cheap to revisit.
3. Any restart or stack-change proposal must update this document first: state what failed, with evidence, and why the replacement does not repeat the pattern above.
