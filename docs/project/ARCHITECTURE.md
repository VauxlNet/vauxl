# Architecture

> Update (2026-03-02): Vauxl has pivoted to a Matrix-first approach. See `MATRIX_FIRST.md` for the active direction and MVP scope. This document remains as historical architecture context.

This document describes the technical architecture of Vauxl, a secure multiplatform communication platform. It covers the system design, component responsibilities, technology decisions, and implementation patterns.

## Table of Contents

1. [Architectural Overview](#architectural-overview)
2. [Design Principles](#design-principles)
3. [System Layers](#system-layers)
4. [Client Architecture](#client-architecture)
5. [Server Architecture](#server-architecture)
6. [Cryptographic Architecture](#cryptographic-architecture)
7. [Network Architecture](#network-architecture)
8. [Data Architecture](#data-architecture)
9. [Technology Decisions](#technology-decisions)

## Architectural Overview

Vauxl implements a layered architecture with strict separation between the presentation layer (Flutter/Dart) and the application core (Rust). This separation, known as the "Humble UI" pattern, ensures that security-critical operations remain isolated in a memory-safe environment.

```
┌────────────────────────────────────────────────────────────────┐
│                        Client Device                           │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                    Flutter UI (Dart)                     │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐   │  │
│  │  │   Widgets   │  │    BLoC     │  │  LiveKit SDK    │   │  │
│  │  └──────┬──────┘  └──────┬──────┘  └────────┬────────┘   │  │
│  └─────────┼────────────────┼──────────────────┼────────────┘  │
│            │                │                  │               │
│  ┌─────────┼────────────────┼──────────────────┼────────────┐  │
│  │         └────────────────┴──────────────────┘            │  │
│  │              flutter_rust_bridge (FFI)                   │  │
│  └──────────────────────────┬───────────────────────────────┘  │
│                             │                                  │
│  ┌──────────────────────────┴───────────────────────────────┐  │
│  │                     Rust Core                            │  │
│  │  ┌───────────┐  ┌───────────┐  ┌───────────┐  ┌───────┐  │  │
│  │  │   State   │  │  Crypto   │  │  Network  │  │  DB   │  │  │
│  │  │  Manager  │  │ (OpenMLS) │  │   (WSS)   │  │(SQLite│  │  │
│  │  └───────────┘  └───────────┘  └───────────┘  └───────┘  │  │
│  └──────────────────────────────────────────────────────────┘  │
└────────────────────────────────────────────────────────────────┘
                              │
                              │ WSS / UDP
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Server Infrastructure                      │
│                                                                 │
│  ┌─────────────────────┐          ┌─────────────────────────┐   │
│  │   Auth & Platform   │          │   Media Server          │   │
│  │   (Rust + Axum)     │◀──────▶│   (LiveKit SFU)         │   │
│  │                     │   gRPC   │                         │   │
│  │  - PASETO v4 Auth   │          │  - Voice routing        │   │
│  │  - Bloom Filters    │          │  - Video routing        │   │
│  │  - NATS Mesh        │          │  - Simulcast            │   │
│  └──────────┬──────────┘          └─────────────────────────┘   │
│             │                                  ▲                │
│             ▼                                  │                │
│  ┌─────────────────────┐          ┌─────────────────────────┐   │
│  │     ScyllaDB        │          │    Object Storage       │   │
│  │  (Shard-per-Core)   │          │   (S3 / MinIO / FS)     │   │
│  └─────────────────────┘          └─────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
                     ┌──────────────────┐
                     │   Push Gateway   │
                     │ (push.vauxl.net) │
                     └──────────────────┘
```

### Missing Components Addressed

1.  **Object Storage (S3/MinIO)**: Required for file attachments/media. ScyllaDB is not suitable for 100MB+ blobs.
    *   *Small*: Local Filesystem.
    *   *Large*: MinIO/S3.
2.  **Push Gateway**: Self-hosted servers cannot talk to APNS/FCM directly (requires App Signing Key). They must route via a managed privacy-preserving gateway.


## Design Principles

### Memory Safety Without Garbage Collection

The Rust core provides memory safety guarantees through its ownership model rather than garbage collection. This eliminates:

- Buffer overflows
- Use-after-free vulnerabilities
- Data races in concurrent code
- Garbage collection pauses during real-time operations

### Humble UI

The user interface is deliberately restricted to presentation concerns:

- Receives immutable state snapshots from the core
- Dispatches user actions to the core
- Never holds cryptographic material
- Cannot bypass the core for network or database access

This isolation means a vulnerability in the UI layer cannot directly compromise cryptographic keys or bypass encryption.

### Type Safety Across Boundaries

Strong typing is enforced from database to user interface:

- Database queries are compile-time verified (sqlx)
- Rust structs define the source of truth for data shapes
- flutter_rust_bridge generates type-safe Dart bindings
- No `dynamic` types or runtime type coercion

### Zero-Knowledge Server

The server infrastructure cannot decrypt user messages:

- End-to-end encryption via MLS
- Server acts as a "Delivery Service" forwarding opaque ciphertext
- Server stores encrypted blobs, not plaintext
- Metadata minimization where possible

## System Layers

### Layer 1: User Interface

**Technology:** Flutter (Dart)

**Responsibilities:**
- Rendering widgets and handling user gestures
- Visual animations and transitions
- Displaying state received from the core
- Managing LiveKit media streams (audio/video rendering)

**Boundaries:**
- Receives state via flutter_rust_bridge streams
- Sends actions via flutter_rust_bridge function calls
- Accesses media devices via LiveKit SDK only

### Layer 2: Bridge

**Technology:** flutter_rust_bridge v2

**Responsibilities:**
- Automated FFI code generation
- Type marshalling between Dart and Rust
- Async operation support
- Stream bridging for reactive state updates

**Implementation Details:**
- Rust functions are annotated for export
- Codegen produces Dart wrapper classes
- Complex types (structs, enums, options) are mapped automatically
- Errors are converted to Dart exceptions

### Layer 3: Application Core

**Technology:** Rust

**Responsibilities:**
- Application state management
- Business logic validation
- Cryptographic operations
- Database access
- Network communication (excluding media streams)

**Submodules:**

| Module | Responsibility |
|--------|----------------|
| `state` | Centralized application state, event dispatching |
| `crypto` | OpenMLS wrapper, key management |
| `database` | SQLite operations, schema management |
| `network` | WebSocket client, signaling |
| `models` | Shared data structures |
| `bridge` | FFI export definitions |

### Layer 4: Server

**Technology:** Rust (API), Go (Media)

**Components:**

| Component | Technology | Purpose |
|-----------|------------|---------|
| Platform & Auth | Axum (Rust) | Identity, Signaling, Message Relay, NATS Sync |
| Media Server | LiveKit (Go) | Voice/video SFU |
| Database | ScyllaDB | High-throughput persistence (User/Msg) |
| TURN Service | turn-rs (Fork) | High-performance NAT traversal |

## Client Architecture

### State Management

The Rust core maintains a centralized state store. State transitions follow a unidirectional data flow:

1. UI dispatches an action (e.g., `SendMessage`)
2. Core processes the action, updating internal state
3. Core emits a state update via the bridge stream
4. UI receives the new state and recomposes

```
┌──────────────────────────────────────────────────────────────┐
│                         Rust Core                            │
│                                                              │
│  Action ──▶ ┌──────────┐ ──▶ Side Effects ──▶ ┌─────────┐ │
│              │ Reducer  │                        │  State  │ │
│              └──────────┘ ◀── State Update ◀── └────┬────┘ │
│                                                       │      │
└───────────────────────────────────────────────────────│──────┘
                                                        │
                         Stream                         ▼
                                                  ┌──────────┐
                                                  │  UI      │
                                                  │  Rebuild │
                                                  └──────────┘
```

### Database Layer

Local storage uses rusqlite with sqlcipher for encryption at rest.

**Key Decisions:**
- Bundled SQLite (consistent version across platforms)
- sqlcipher for transparent encryption
- Schema migrations managed in Rust
- All queries through Rust core (UI cannot access database)

**Data Categories:**

| Category | Encryption | Retention |
|----------|------------|-----------|
| Cryptographic keys | sqlcipher + MLS key protection | Permanent until revoked |
| Message content | MLS application messages, then sqlcipher | User configurable |
| Message metadata | sqlcipher | User configurable |
| User settings | sqlcipher | Permanent |
| Cache data | sqlcipher | Automatic expiration |

### Network Layer

The Rust core manages a persistent WebSocket connection to the API server.

**Responsibilities:**
- Connection lifecycle (connect, reconnect, backoff)
- Message queue for offline resilience
- Signaling for WebRTC session establishment
- MLS message delivery

**Message Types:**

| Type | Direction | Purpose |
|------|-----------|---------|
| Auth | Outbound | Session establishment |
| MLS Commit/Proposal | Both | Group key management |
| MLS Application | Both | Encrypted user messages |
| Signaling | Both | WebRTC session negotiation |
| Presence | Both | Online status updates |

## Server Architecture

### Auth & Platform Service ("Insane Scale")

A custom Rust service designed for privacy and 1M+ concurrent users.

**Authentication & Security:**
- **Token Strategy**: **PASETO v4.public** (Asymmetric). Allows downstream services (TURN) to verify tokens offline.
- **Revocation**: **Distributed Bloom Filters** synced via **NATS**.
  - Checks are in-memory (nanosecond latency).
  - No database hit for validity checks.
- **Failover**: Stateless nodes; clients automatically failover to secondary regions (Multi-DC).

**Platform Responsibilities:**
- **User Management**: Registration, Identity Keys.
- **Message Relay**: Storing/Forwarding encrypted MLS blobs.
- **Signaling**: WebSocket handling for WebRTC setup.

### Media Server

LiveKit provides the Selective Forwarding Unit (SFU) for voice and video:
- **Simulcast**: Automatic quality adaptation.
- **Codecs**: Opus (Audio), VP8/VP9/H.264/AV1 (Video).
- **Integration**: Validates auth tokens via Public Key (no RPC needed per packet).

### File Storage Strategy

Handling "Military Grade" security for attachments (100MB+):

| Tier | Technology | Description |
|------|------------|-------------|
| **Self-Hosted** | Local FS + Nginx | Files stored in `data/uploads`. Served via authenticated URL. |
| **Enterprise** | MinIO / S3 | Object storage with pre-signed URLs. |
| **Encryption** | Client-Side | Files are encrypted **before** upload (AES-GCM). Server sees opaque blobs. |

### Message Bus (NATS)

Used for "Typing Indicators", "Presence", and "Signaling".

- **Monolith Mode**: Uses internal `tokio::sync::broadcast` (No external dependency).
- **Cluster Mode**: Connects to external `nats-server` for multi-node sync.

### Push Notification Architecture

To allow self-hosted servers to wake up iOS/Android devices without exposing user data:

1.  **Event**: User receives message on Self-Hosted Server A.
2.  **Anonymization**: Server A truncates content, keeping only `message_id` and `sender_id`.
3.  **Relay**: Server A sends encrypted signal to `push.vauxl.net` (Vauxl Managed).
4.  **Delivery**: `push.vauxl.net` contacts Apple/Google.
5.  **Wakeup**: App wakes up, connects to Server A, and downloads the actual message.

**Privacy**: `push.vauxl.net` never sees who is messaging whom or what they said. Only "Device X has new data".


### TURN Service

A privacy-hardened fork of **turn-rs**:
- **Performance**: Pure Rust, `io_uring`/`epoll` driven, >40M pkts/sec.
- **Architecture**: Shared-nothing clusters isolated by region.
- **Routing**: **GeoDNS** determines user connection point (No Anycast).
- **Auth**: gRPC to Platform Service for `ALLOCATE`; local **Moka** cache (60s) for performance.

### Data Persistence Layer

We utilize the **Repository Pattern** to abstract data storage, allowing the platform to scale from a Raspberry Pi to a hyperscale cluster without code changes.

**Storage Backends:**

| Tier | Backend | Recommended Use | Pros | Cons |
|------|---------|-----------------|------|------|
| **Dev** | `MemStore` | Unit Testing, Rapid Prototyping | Zero setup, instate wipe | Data lost on restart |
| **Edge** | `SQLite` | Self-hosting (Teamspeak-like), Small Clusters | runs on 512MB RAM, single file | Vertical scaling limit |
| **Core** | `ScyllaDB` | "Insane Scale" (1M+ Users) | High throughput, shard-per-core | High resource floor, complex ops |

**Schema Design (Scylla/Cassandra variant):**

| Table | Partition Key | Clustering Key | TTL | Purpose |
|-------|---------------|----------------|-----|---------|
| `user_credentials` | `user_id` | - | None | Permanent Identity & Auth Secrets |
| `revoked_tokens` | `jti` | `expires_at` | Auto | Source of Truth for Bloom Filters |
| `offline_messages` | `recipient_id` | `timestamp` | **120d** | Encrypted blobs for device sync |
| `mls_keys` | `user_id` | `key_id` | None | Public key packages for MLS |

**Privacy Policy:**
- **Identity**: Stored permanently until account deletion.
- **Messages**: Strict TTL (Default 120 Days, configurable 0s-1y).
- **Compaction**: `TimeWindowCompactionStrategy` (TWCS) ensures expired data is physically deleted efficiently.

## Cryptographic Architecture

### Messaging Layer Security (MLS)

Vauxl implements IETF RFC 9420 for group messaging encryption.

**Why MLS over Signal Protocol:**

| Aspect | Signal Protocol | MLS |
|--------|-----------------|-----|
| Add member | O(N) operations | O(log N) operations |
| Remove member | O(N) operations | O(log N) operations |
| Group size | Practical limit ~1000 | Scales to 10,000+ |
| Standard | Proprietary | IETF RFC 9420 |

**Key Concepts:**

- **TreeKEM:** Tree-based key encapsulation for group secrets
- **Key Package:** Public key bundle for adding users to groups
- **Commit:** Atomic group state change
- **Proposal:** Suggested group state change
- **Application Message:** Encrypted user content

### Implementation: OpenMLS (Work In Progress)

OpenMLS is the Rust implementation of RFC 9420.
**Status**: Core dependency integration pending.

**Integration Points:**
- Key storage: sqlcipher-encrypted SQLite
- Identity: Ed25519 keypairs via ed25519-dalek
- Transport: Serialized MLS messages over WebSocket

### Key Management

| Key Type | Generation | Storage | Lifetime |
|----------|------------|---------|----------|
| Identity Key | Device first run | Local encrypted DB | Device lifetime |
| Signature Key | Per-user | Local encrypted DB | User account lifetime |
| MLS Epoch Key | Per group commit | Memory + encrypted DB | Until next epoch |
| Message Key | Per message | Derived, not stored | Single use |

## Network Architecture

### Text/Data Transport

WebSocket Secure (WSS) carries all non-media traffic:

- Persistent connection with automatic reconnection
- Message framing with type discrimination
- Compression for large payloads
- Connection multiplexing (single socket per session)

### Media Transport

LiveKit handles media transport:

- UDP preferred for low latency
- DTLS for encryption
- ICE for NAT traversal
- TURN fallback for restrictive networks

### Audio Processing Pipeline

```
Microphone ──▶ Capture ──▶ AEC ──▶ Noise Suppression ──▶ Encode ──▶ Network
                           │               │
                           │               └── dtln-rs (DNN-based)
                           └── fdaf-aec (Rust)

Network ──▶ Decode ──▶ Jitter Buffer ──▶ Mix ──▶ Playback
```

## Data Architecture

### Message Model

```
Message
├── id: UUID
├── group_id: UUID
├── sender_id: UUID
├── timestamp: DateTime
├── content: EncryptedBlob  // MLS application message
├── content_type: Enum      // Text, Image, File, etc.
└── local_state: Enum       // Sending, Sent, Delivered, Read

// After decryption in Rust core:
DecryptedMessage
├── id: UUID
├── sender: UserInfo
├── timestamp: DateTime
├── content: Content        // Typed content union
│   ├── Text { markdown: String }
│   ├── Image { ... }
│   ├── File { ... }
│   └── ...
└── reactions: Vec<Reaction>
```

### User Model

```
User
├── id: UUID
├── username: String
├── display_name: String
├── avatar_url: Option<String>
├── status: PresenceStatus
└── public_keys: KeyPackageBundle

// Local extension (not synced to server)
LocalUserData
├── user_id: UUID
├── notes: String
├── nickname: Option<String>
└── notification_settings: Settings
```

### Group Model

```
Group
├── id: UUID
├── name: String
├── icon_url: Option<String>
├── members: Vec<MemberInfo>
├── channels: Vec<Channel>
├── roles: Vec<Role>
└── mls_state: MLSGroupState  // Opaque to UI

Channel
├── id: UUID
├── name: String
├── channel_type: Enum  // Text, Voice, Announcement
└── permissions: PermissionSet
```

## Technology Decisions

### Why Flutter over Kotlin Multiplatform

| Factor | Flutter | Kotlin Multiplatform |
|--------|---------|----------------------|
| Desktop stability | Production ready | Experimental (Compose Desktop) |
| Rendering | Custom (Skia/Impeller), identical pixels | Platform widgets, slight differences |
| Plugin ecosystem | Extensive | Growing |
| Developer tooling | Mature (DevTools, hot reload) | Good but less polished |
| Integration with Rust | flutter_rust_bridge (mature) | UniFFI (requires more setup) |

### Why LiveKit over Custom WebRTC

Building a production WebRTC stack requires:
- Echo cancellation
- Noise suppression
- Bandwidth estimation
- Simulcast management
- SRTP encryption
- STUN/TURN integration

LiveKit provides all of this as a tested, open-source package. Building from scratch with str0m would take significant additional development time.

### Why Abstract Storage (ScyllaDB / SQLite)

To support both "Insane Scale" (1M+ Users) and efficient Self-Hosting:

- **The Problem with Scylla-Only**: Scylla requires significant resources (RAM/CPU pinning) that makes it cost-prohibitive for small, self-hosted instances.
- **The Solution**: We define a strict `UserStore` / `MessageStore` trait.
  - **SQLite**: Default for self-hosting. Single file, works everywhere, sufficient for <10k users.
  - **ScyllaDB**: Plug-in replacement for enterprise deployments requiring linear horizontal scaling.


### Why Axum over Other Rust Web Frameworks

| Factor | Axum | Actix-web | Warp |
|--------|------|-----------|------|
| Maintainer | Tokio team | Community | Sean McArthur |
| Type safety | Excellent | Good | Excellent |
| Ecosystem | tokio native | Actor model | tokio compatible |
| Learning curve | Moderate | Moderate | Steeper |
| Active development | Very active | Active | Slower |

## Performance Considerations

### Virtual List Rendering

Chat history uses virtualized lists (Flutter `ListView.builder`):
- Only visible items are rendered
- Efficient scroll position management
- Target: 60fps with 100,000+ messages

### Rust Core Responsiveness

Long-running operations must not block the UI:
- Database operations are async
- Cryptographic operations use thread pools
- State updates are batched to reduce bridge crossings

### Memory Management

- Rust core: Ownership model prevents leaks
- Dart UI: Minimize widget rebuilds
- Image caching: Bounded LRU caches
- Message history: Pagination, not full load

## Appendix: Crate Dependencies

### Core Rust Crates

| Crate | Purpose |
|-------|---------|
| tokio | Async runtime |
| serde | Serialization |
| rusqlite | SQLite bindings |
| sqlcipher | Database encryption |
| openmls | MLS implementation |
| ed25519-dalek | Signature keys |
| tracing | Structured logging |
| thiserror | Error types |
| anyhow | Error handling |
| flutter_rust_bridge | FFI generation |

### Server Rust Crates

| Crate | Purpose |
|-------|---------|
| axum | Web framework |
| sqlx | Database access |
| tower | Middleware |
| jsonwebtoken | JWT handling |
| argon2 | Password hashing |
| uuid | Identifier generation |
