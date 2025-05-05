# <img src="vauxl_icon.png" alt="Vauxl Logo" width="30" height="30" style="vertical-align: middle; margin-right: 10px;"> Vauxl

[![Rust](https://img.shields.io/badge/Rust-1.76%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![CI Status](https://github.com/VauxlNet/vauxl/actions/workflows/client_ci.yaml/badge.svg)](https://github.com/VauxlNet/vauxl/actions/workflows/client_ci.yaml)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)


## Architecture

### Two‑Layer (Monolithic Core + GUI)
- Structure 
  - `core` crate: everything (call logic, codecs, networking, timers, audio I/O)
  - `gui` crate: Dioxus front‑end
- Pros
  - Fast to start—only two crates to manage.
  - Fewer boundaries means less boilerplate “glue.”
- Cons
  - Harder to test just the domain logic in isolation (you’ll need to stub networking and audio inside core).
  - Any change to transport or codecs forces rebuild of the entire core.
  - Swapping out a dependency (say Tokio → async‑std) ripples through domain code.

### Three‑Layer (Core + Bridge/Facade + GUI)
- Structure
  - `core` crate: pure VoIP domain (traits for transport, audio, timers, signaling)
  - `bridge` crate: implements core traits with Tokio, cpal, codec libs, and exposes a simple async façade (e.g. dial(), hangup(), stream of events)
  - `gui` crate: Dioxus UI that consumes only the bridge API
- Pros
  - core is 100% business logic, zero external deps—easy unit tests and CI.
  - Bridge hides all runtime and audio‑framework details from UI.
    - You can build other front‑ends (CLI, WebAssembly) against the same bridge.
- Cons
  - Three crates to maintain and version.
  - Some initial boilerplate writing those façade methods and event streams.

### Four‑Layer (Hexagonal / Clean Architecture)
- Structure
  - `domain` crate: call‐state machine, media‐pipeline traits, error types
  - `transport` crate: network I/O (RTP/SIP over UDP/TCP) & timers
  - `media` crate: audio capture/playback, codec implementations (Opus, G.711, etc.)
  - `bridge` crate: combines domain + transport + media into an ergonomic async API
  - `gui` crate: Dioxus UI only talks to bridge
- Pros
  - Maximum separation of concerns—each piece can be swapped, mocked, or independently versioned.
  - Ideal for very large teams or when you expect many protocol/media implementations/plugins.
  - Smaller compile units can shave CI/build times if you only change one layer.
- Cons
  - Significant upfront complexity and Cargo configuration.
  - More crates = more release artifacts and more “glue” code.
  - Could be overkill unless you need heavy pluggability or have multiple teams.

### Path

- Start two‑layer.
- After a few features, pull out the “core traits + pure logic” into core.
- Create a small bridge crate when you build your first GUI.
Only split transport vs media when you actually need two independent implementations.

## Messaging

#### Guest Identity & E2EE Handshake
- On first run, each client generates a long‑term key pair (e.g. ed25519 for signing/identity + X25519 for key agreement).
- Show the user a short, human‑readable fingerprint (like “AB12‑34CD”) so they can verify they’re talking to the right person.
- When Alice wants to message Bob, she needs Bob’s IP (or DNS) and his public key fingerprint.
- Perform a mutual authenticated Diffie‑Hellman handshake over a UDP/TCP channel (ICE/UDP hole‑punch if behind NAT, using public STUN servers).
- Derive a symmetric session key and then encrypt everything with an AEAD cipher (e.g. ChaCha20‑Poly1305).

#### NAT Traversal & Bootstrapping
- You can’t rely on everyone having a public IP—use ICE (STUN + optional TURN) to negotiate connectivity.
- Bundle a small list of well‑known STUN servers in your client config. If direct P2P fails, fall back to a TURN relay (you can even self‑host a TURN server, but that introduces a semi‑central component).
- Exchange “candidate” addresses over whichever channel you have (e.g. QR code, a trust‑on‑first‑use link you email, or simple “copy–paste” of IP:port + fingerprint).

#### Spam & Abuse Mitigation
- Connection Requests: every new peer connection must be explicitly accepted. Don’t auto‑accept any inbound ICE handshake—even if it solves NAT punching, require a UI dialog.
- Proof‑of‑Work (optional): to deter automated spamming, you can require the initiator to include a small PoW token (e.g. Hashcash) that the receiver verifies before showing the “Accept connection?” prompt.
- Rate‑Limit & Blocklists:
  - Let users block senders by fingerprint/IP.
  - Enforce client‑side rate limits (e.g. max N new connection requests per minute from the same IP range).
- Greylisting: silently drop or defer messages from unverified peers until the user allows them.

#### Offline & Store‑and‑Forward
Pure P2P means if Bob is offline, Alice’s messages cannot be delivered. To support offline delivery you have two broad choices:
- Light Relay Layer
  - Peers can optionally designate one or more “relay” nodes (could be friends’ always‑on clients or a small fleet of volunteer servers you self‑host).
    - When you send a message, you say “deliver to Bob via Relay X.” Relay X holds it encrypted under your session key until Bob comes back online and fetches it.
    - Relay only sees ciphertext, so E2EE is preserved.
- DHT‐Based Store
  - Implement a small distributed hash table (Kademlia) where peers store (keyed by Bob’s public key) messages until he comes online.
  - Very complex to get reliable, highly available, and to prevent spam storage.

If you want to stay truly server‑less, you must accept “no offline messaging” initially and then layer on a lightweight relay network later.

#### Local History & Synchronization
- Each client persists its own chat history encrypted at rest (e.g. using the session key or a higher‑level “history” key derived from your identity key).
- Outgoing messages: keep a local journal until you get an ACK from the peer.
- Incoming messages: store in a ring buffer or database (sled, sqlite) with timestamps and status (read/unread).
- If you later add multiple devices per user, you’d need a sync protocol (e.g. CRDTs or append‑only logs) over the same P2P/E2EE channel or via your relay. But that’s an advanced iteration.


### MVP Roadmap

- Identity & Key Management
  - Generate key pairs, show fingerprint, let user set a display name.
- Direct P2P Messaging (Online‑Only)
  - ICE with STUN only, mutual handshake, derive session key, send/receive text messages.
  - UI: prompt for remote IP + fingerprint, accept/decline incoming.
- Local History
  - Persist both sides of a conversation in an encrypted local store.
- Spam Controls
  - Mandatory “Accept connection” UI, blocklist by fingerprint, simple rate‑limits.
  - (Optional) Relay‑Based Offline Delivery
  - Implement a tiny TURN‑like relay for store‑and‑forward only.
  - Relay stores encrypted blobs until the recipient fetches them.


By starting with online‑only P2P and a hard “accept connection” barrier, you get a working E2EE guest chat with minimal infra. Once that’s stable, you can add optional relays for offline delivery or explore DHTs for fully decentralized store‑and‑forward.

## First Client Implementation Concepts - NeedToCheck

Identity & Key Storage
• On first launch, generate a device identity:
– ed25519 keypair for signing/identity
– X25519 keypair for Diffie‑Hellman key agreement
• Persist the key material in an OS‑protected location (and nowhere else):
– Desktop: use the user’s keyring/keychain (e.g. macOS Keychain, Windows DPAPI, Linux Secret Service) via a Rust crate like secret-service or keyring.
– Mobile: use iOS Keychain / Android Keystore via your Rust bindings or via the GUI toolkit’s plugin.
• If you can’t rely on an OS keystore (e.g. portable binary), store the raw private keys in a file under your app’s data directory (~/.local/share/vauxl or %APPDATA%\vauxl) with filesystem permissions locked to the user (0600).

Runtime Session Keys & E2EE
• When A wants to chat with B:
– A copies B’s “address” (IP:port or hostname) and public ed25519 fingerprint.
– A and B perform a mutual authenticated X25519 handshake over UDP (use ICE with STUN for NAT punching).
– Derive a symmetric AEAD key (ChaCha20‑Poly1305).
– All subsequent messages get encrypted+authenticated under that session key.
• You never store these session keys on disk—keep them in memory only, tied to the open P2P socket.
• When the session closes, zero‑out the session key in RAM.

Local Message Storage (Encrypted at Rest)
• You still need to persist chat history for offline viewing. Use an embedded, file‑based store—no server process:
Option A: SQLite + SQLCipher
– rusqlite with SQLCipher gives you ACID, indexing, and full‑database encryption.
– The database file (e.g. ~/.local/share/vauxl/messages.db) is encrypted at rest with a key you derive on startup.
Option B: Pure‑Rust KV store (sled, RocksDB, or LMDB) + your own envelope encryption
– Use sled or heed and wrap each value with your own AEAD over the session key or a long‑term “history” key.
• History key management:
– Derive a single “history” symmetric key on first run (random 256‑bit), then store that in the same OS keystore as your identity keys.
– On startup, load it and unlock the DB.
• Schema example (if using SQLite):

```SQL
CREATE TABLE conversations (
id           BLOB PRIMARY KEY,   -- SHA256 of peer’s public key
display_name TEXT NOT NULL
);
CREATE TABLE messages (
conversation_id  BLOB REFERENCES conversations(id),
timestamp        INTEGER,        -- UNIX millis
is_outgoing      BOOLEAN,
ciphertext       BLOB,
PRIMARY KEY (conversation_id, timestamp)
);
```
• All ciphertext blobs inside messages are already E2EE; SQLCipher just encrypts the whole file so someone who steals messages.db can’t even see metadata.

“No‑Log” & Minimal Persistence
- Never write plaintext or metadata you don’t need:
- Don’t log raw packet dumps, don’t log session keys.
- Only keep display names, timestamps, and ciphertext.
- If you want ephemeral chats, add a “self‑destruct” flag on conversations that purges history when the session closes.
- Allow users to wipe all local data/redact histories via a menu.

Offline Delivery (Later)
- Phase 1 (LAN‑only, online‑only): messages only flow if both peers are live.
- Phase 2 (optional small relay): spin up lightweight TURN‑style relays that store only encrypted blobs until the peer fetches them. The relay never sees keys or plaintext.

Putting It Together
- Folder layout on disk (example Linux):
  - ~/.local/share/vauxl/ 
  <br>├── keys.bin (encrypted identity & history keys, stored via OS keystore)
  <br>├── messages.db (SQLCipher‑encrypted SQLite file)
  <br>└── config.json (minimal: last‑used peers, STUN/TURN servers)
- At startup:

Unlock OS keystore → load identity & history keys.
Open SQLCipher DB with history key.
Spawn your ICE/STUN handler, ready to accept inbound connections.
- At shutdown:
- Close all P2P sessions, zero‑out session keys in memory, close DB.
With this design you get:
- True device‑only P2P E2EE (no central server)
- Encrypted data‑at‑rest with minimal metadata
- No always‑on SQL server—just an embedded DB
- Full privacy‑by‑design: nothing is persisted unless it’s ciphertext you need to display chat history.