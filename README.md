# Pimcore Asset Bridge

**Pimcore Asset Bridge** connects external file systems with Pimcore’s Digital Asset Management (DAM) system in real-time.

It consists of:
- `asset-bridge-daemon`: A lightweight, high-performance Rust daemon for monitoring filesystem changes.
- `AssetBridgeBundle`: A Symfony bundle for Pimcore that processes and synchronizes file events into assets.

---

## Components

- **daemon/**: Rust-based filesystem watcher.
- **pimcore-bundle/**: Pimcore Symfony bundle extension.

## Key Features

- Real-time create, modify, delete, rename detection
- Clean batching and debouncing
- Full asset synchronization with Pimcore
- Remote filesystem detection
- Graceful shutdown and error resilience
