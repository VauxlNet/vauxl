# <img src="vauxl_icon.png" alt="Vauxl Logo" width="30" height="30" style="vertical-align: middle; margin-right: 10px;"> Vauxl

[![Rust](https://img.shields.io/badge/Rust-1.76%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![CI Status](https://github.com/VauxlNet/vauxl/actions/workflows/ci.yaml/badge.svg)](https://github.com/VauxlNet/vauxl/actions/workflows/ci.yaml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
<!-- TODO: Add links to community chat (Discord/Matrix?) -->
<!-- [![Discord](https://img.shields.io/discord/YOUR_DISCORD_ID?label=Discord)](YOUR_DISCORD_INVITE_LINK) -->

Vauxl is an ambitious project to build a privacy-focused, open-source, and highly customizable communication platform for VoIP and text chat. Built primarily in Rust, it aims to provide a familiar user experience (inspired by Discord) while giving users greater control over their data and environment through self-hosting options and extensive client customization.

## Key Features (Planned)

*   **Core Communication:** Voice & Text channels, Direct Messages (with planned E2EE).
*   **User Features:** Rich User Profiles, Friend System with Presence Status.
*   **Server Management:** Roles & Permissions system.
*   **Customization:**
    *   **Theming:** Built-in themes (e.g., Dark, Light Professional) and support for user-created custom CSS themes.
    *   **Plugin System:** Planned support for extending client functionality via secure WebAssembly (Wasm) plugins.
*   **Hosting Flexibility:**
    *   **Self-Hosted Instance:** Deploy your own multi-tenant Vauxl server.
    *   **Embedded LAN Server:** Run a lightweight server directly from the client for local network use (e.g., LAN parties).
*   **Privacy-First:** Designed to minimize data collection, with E2EE as a core goal for private communications.
*   **Cross-Platform Client:** Desktop client built with Tauri for Windows, macOS, and Linux.
*   **Modern Tech Stack:** Leveraging Rust for performance and safety.

## Vision & Goals

In a world of increasingly centralized communication platforms, Vauxl aims to provide a robust, user-controlled alternative. We believe users should have choices about where their data resides and how their communication tools look and behave. Our goals are to deliver:

1.  **Privacy & Security:** A platform where users can communicate securely, with features like E2EE.
2.  **User Control:** Flexible hosting and deep client customization.
3.  **Familiarity & Functionality:** A feature set and UI/UX that feels comfortable for users migrating from other platforms, while offering unique advantages.
4.  **Openness:** Fully open-source development under the Apache 2.0 license, encouraging community involvement.

## ⚠️ Current Status: Pre-Alpha / Early Development ⚠️

**Vauxl is currently in the very early stages of development (Phase 0/1). It is NOT yet ready for general use.**

Core functionalities are actively being built. Expect bugs, missing features, and potentially breaking changes. Contributions are welcome (see below), but please be aware of the project's nascent stage.

## Technology Stack

| Category                 | Chosen Technology                                    |
| :----------------------- | :--------------------------------------------------- |
| **Primary Language**     | Rust (Latest Stable)                                 |
| **Client UI Framework**  | Tauri (Latest Stable)                                |
| **Client Frontend**      | HTML5, CSS3, TypeScript + React/Vue/Svelte (TBD)     |
| **Async Runtime**        | Tokio                                                |
| **Network (TCP/TLS)**    | `tokio-rustls` using `rustls`                        |
| **Network (UDP/DTLS)**   | `webrtc-rs/dtls` (or QUIC exploration later)         |
| **Network (Media/SRTP)** | `webrtc-rs/srtp`                                     |
| **Audio Codec**          | `opus` crate                                         |
| **Audio I/O**            | `cpal` crate                                         |
| **Crypto (Hashing)**     | `argon2` crate                                       |
| **Crypto (E2EE)**        | `libsignal-protocol-rs` (Research) or primitives     |
| **Database (Hosting)**   | PostgreSQL                                           |
| **Database Driver**      | `sqlx` crate                                         |
| **Serialization**        | `serde` with `serde_json`, `rmp-serde` (MessagePack) |
| **Plugin System**        | WebAssembly (Wasm) + `wasmtime`                      |
| **Build & CI/CD**        | Cargo, GitHub Actions                                |

## Project Structure (Monorepo with Cargo Workspace)

This repository uses a monorepo structure managed by Cargo Workspaces. This keeps closely related components (client, server core, hosting instance) together for easier atomic changes and dependency management.

*   `vauxl/`: Root workspace definition (`Cargo.toml`)
    *   `vauxl-client/`: The Tauri desktop application (Rust backend + Web frontend).
    *   `vauxl-server-core/`: Shared Rust library containing core server logic (state, networking, crypto).
    *   `vauxl-hosting-instance/`: The standalone, multi-tenant Rust server application.
    *   `vauxl-protocol/`: (Optional) Shared crate for communication protocol definitions.
    *   `docs/`: Project documentation.

## Getting Started (Development)

**Prerequisites:**

*   Rust Toolchain (latest stable): [https://rustup.rs/](https://rustup.rs/)
*   Node.js and npm/yarn: [https://nodejs.org/](https://nodejs.org/)
*   Tauri Prerequisites: Follow the guide at [https://tauri.app/v1/guides/getting-started/prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites) for your specific OS.

**Steps:**

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/Vauxl/vauxl.git
    cd vauxl
    ```
2.  **Install frontend dependencies:**
    ```bash
    cd vauxl-client
    npm install # or yarn install
    ```
3.  **Build & Run the client in development mode:**
    ```bash
    # Still inside the vauxl-client directory
    cargo tauri dev
    ```
    *(Note: This currently only starts the client shell. Server functionality needs to be built and integrated according to the development plan.)*

## Contributing

We welcome contributions! As the project is in its early stages, contributions to core features, bug fixing, documentation, and testing are highly valuable.

Please read our **[CONTRIBUTING.md](CONTRIBUTING.md)** guide for details on the development process, code style, and how to submit pull requests.

We adhere to a **[CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)**. Please ensure you follow it in all interactions.

## License

Vauxl is licensed under the **[Apache License Version 2.0](LICENSE)**. See the LICENSE file for details.

## Community & Contact

*   **Issues:** Report bugs or suggest features via [GitHub Issues](https://github.com/Vauxl/vauxl/issues).
*   **Discussions:** For questions, ideas, and general discussion, use [GitHub Discussions](https://github.com/Vauxl/vauxl/discussions).
*   **Chat:** <!-- TODO: Add link to Matrix/Discord server when created -->