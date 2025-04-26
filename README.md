# Pimcore Asset Bridge

**AssetBridge** connects external file systems to Pimcore’s Digital Asset Management (DAM) system in real-time.

It consists of:
- `asset-bridge-daemon`: A lightweight, high-performance Rust daemon for monitoring filesystem changes.
- `AssetBridgeBundle`: A Symfony bundle for Pimcore that processes and synchronizes file events into assets.

---

## Features

- Real-time detection of create, modify, delete, and rename events
- Batched JSON event output with accurate timestamps
- Graceful shutdown handling (Ctrl+C, SIGTERM)
- Full asset synchronization with Pimcore
- Designed for high reliability and minimal resource usage

---

## Installation

### 1. Install via Composer

```bash
composer require cleggypdc/pimcore-asset-bridge
```

This will install the AssetBridge Bundle and its required files, including the daemon binary.

---

### 2. Enable the Bundle

After installation, enable the bundle in your Pimcore project:

```bash
php bin/console pimcore:bundle:enable AssetBridgeBundle
```

---

### 3. (Optional) Verify Daemon Binary

The filesystem monitoring daemon (`asset-bridge-daemon`) is located at:

```plaintext
vendor/cleggypdc/pimcore-asset-bridge/bin/asset-bridge-daemon
```

You can test it manually:

```bash
./vendor/cleggypdc/pimcore-asset-bridge/bin/asset-bridge-daemon /path/to/watch
```

---

### 4. Set Up AssetBridge Integration

Use the provided Symfony command to process filesystem events into Pimcore:

```bash
cat events.json | php bin/console asset-bridge:process-events
```

or specify an input file:

```bash
php bin/console asset-bridge:process-events --input=/path/to/events.json
```

---

## Example Setup Diagram

```plaintext
[ Filesystem Changes ]
        ↓
[ asset-bridge-daemon ]
        ↓
(Generate batched JSON)
        ↓
[ bin/console asset-bridge:process-events ]
        ↓
(Import / Update / Delete Pimcore Assets)
```

---

## Requirements

- PHP 8.0 or higher
- Pimcore 10.5+ or 11.x
- Linux server (daemon binary currently targets Linux x86_64)

---

## TODO

### Core Functionality
- [ ] Implement `handleCreate()` to import new files into Pimcore DAM
- [ ] Implement `handleModify()` to update modified assets
- [ ] Implement `handleDelete()` to remove deleted files from Pimcore DAM
- [ ] Implement `handleRename()` to move/rename assets within Pimcore

### Daemon Improvements
- [ ] Allow daemon to flush batches dynamically based on event volume
- [ ] Add optional configuration file support (e.g., YAML) for daemon
- [ ] Detect remote filesystem mounts and handle accordingly
- [ ] Build systemd service file for daemon to run automatically
- [ ] Improve JSON event output (optional custom timestamp formats)
- [ ] Support error logging and retries for failed asset operations
- [ ] Add support for `.bridgeignore` files to exclude patterns/filetypes
- [ ] Add `max_depth` setting to limit recursive folder watching
- [ ] Support block-level changes (advanced: use Pimcore Versions or Notes?)

### Architecture Improvements
- [ ] Configure the daemon to use Pimcore Datahub API instead of local console
- [ ] (Preferred) Expose a minimal API controller inside AssetBridgeBundle for event ingestion
- [ ] Allow daemon instances to be configured dynamically via Pimcore Settings UI

### Cross-Platform Support
- [ ] (Optional) Add support for Windows/Mac binaries in the future
- [ ] (Optional) Expose a Symfony command to control daemon (start/stop)

### Monitoring & Metrics
- [ ] (Optional) Add basic metrics (number of processed events, error counts)
- [ ] (Optional) Add Prometheus exporter or simple `/metrics` endpoint (future)

### QA / Maintenance
- [ ] Write basic test coverage for daemon and Symfony command
- [ ] (Optional) Publish bundle to Packagist for wider adoption

---


---

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0).  
See the [LICENSE](LICENSE) file for details.

---

Built with ❤️ by [cleggypdc](https://github.com/cleggypdc)

