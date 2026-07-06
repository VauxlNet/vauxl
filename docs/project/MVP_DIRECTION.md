# MVP Direction

Decided 2026-05-21, recorded here 2026-07-06. This is the active product direction. It refines `MATRIX_FIRST.md` with the reasoning behind the MVP scope. For why the stack looks the way it does, see `STACK_POSTMORTEM.md`.

## Positioning

The differentiator is Discord-grade community and voice UX, encrypted, self-hostable, on an open standard. Matrix supplies the standard, the federation, and the audited crypto. Vauxl supplies the polish.

## First users

The maintainers and a small circle of privacy-focused testers. Dogfooding first: friendly, low-stakes, install-tolerant users who give a real feedback loop before any public launch.

## Server stance

- The client must work against a standard, unmodified Matrix homeserver.
- `org.vauxl.*` extensions come later, and only when a concrete feature demands one.
- No custom or forked server is required for the MVP. The server repo is a longer-term track and must not block the client.

## Team and operations scope

Small team. The hosted-platform, auth, TURN, and payments apparatus is deferred entirely. MVP means: a client people can run, plus self-hosting guidance. No large operated deployment.

## Security model

- Rust owns everything security-critical: crypto, key handling, sync, storage, networking, parsing. The core is built on matrix-rust-sdk, which reuses the audited vodozemac crypto, so Vauxl gets Rust and security without writing its own cryptography.
- The web presentation layer is untrusted ("Humble UI"). It never holds key material and talks to the core only through a minimal, typed IPC boundary. See `client/docs/DATA_INTERFACE.md` for the contract and boundary rules.
- Accepted Matrix caveat: even with E2EE content, a homeserver sees metadata (who talks to whom, when, membership). Self-hosting among a trusted circle, which is the MVP plan, mitigates this.

## Shell decision

The desktop shell is Tauri v2. Known tradeoff, accepted deliberately: on Linux Tauri uses the system WebKitGTK webview (a polish and security-patch-cadence risk), while Windows uses WebView2/Chromium. Because the core is decoupled from the shell (see the build workflow below and `PROTOCOL_AGNOSTIC_ARCHITECTURE.md`), the shell choice is reversible and is not allowed to become existential.

## Build workflow

1. Prototype the web UI first to nail the Discord feel, built against an abstract data interface (`ChatBackend`) with a mock backend. The UI survives; it is not throwaway.
2. The UI stays web, permanently. It is never rewritten in Rust. A Rust UI is the documented failure mode for this team.
3. The same interface is then implemented by a real Rust core (matrix-rust-sdk) in the Tauri backend. Swapping mock for real changes one constructor; the UI does not change.

The type contract has one source of truth: TypeScript bindings are generated from the Rust types (tauri-specta). Any domain logic sketched in JS during prototyping is reimplemented in the Rust core, not translated mechanically; promoting prototype code into a security-critical core is not acceptable.

## Status

The contract-first client exists and builds green in the `client` repo: `core/` (contract + mock), `src-tauri/` (shell + generated bindings), `src/` (React UI). Next milestone: implement `ChatBackend` with matrix-rust-sdk behind the same interface.
