# 🛠 Pimcore Asset Bridge

🔋 Connect your file systems to Pimcore’s Digital Asset Management (DAM) in real-time using a lightweight daemon written in rust!

**AssetBridge** monitors filesystem changes, batches them, and updates Pimcore assets — all with high performance and minimal configuration.

## ✨ Features
 - 📁 Real-time file create, modify, delete, and rename detection
 - ⚡ Batched JSON event output with precise timestamps
 - 🧹 Graceful shutdown handling (Ctrl+C, SIGTERM)
 - 💇️ Seamless integration with Pimcore DAM
 - 🛡️ Built for reliability and low resource consumption


## 🛣️ Current Status
**Early Development** — Core functionality is coming together, but many features are still planned. See the TODO section for roadmap details.


## 📦 Installation

### 1. Install via Composer

```bash
composer require cleggypdc/pimcore-asset-bridge
```

### 2. Enable the Bundle

```bash
php bin/console pimcore:bundle:enable AssetBridgeBundle
```

### 3. (Optional) Verify Daemon Binary

The filesystem monitoring daemon (`asset-bridge-daemon`) is located at:

```plaintext
vendor/cleggypdc/pimcore-asset-bridge/bin/asset-bridge-daemon
```

You can test it manually:

```bash
./vendor/cleggypdc/pimcore-asset-bridge/bin/asset-bridge-daemon /path/to/watch
```

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

## Requirements

- PHP 8.0 or higher
- Pimcore 10.5+ or 11.x
- Linux server (daemon binary currently targets Linux x86_64)

## TODO

### Core Functionality
- [ ] Implement `handleCreate()` to import new files into Pimcore DAM
- [ ] Implement `handleModify()` to update modified assets
- [ ] Implement `handleDelete()` to remove deleted files from Pimcore DAM
- [ ] Implement `handleRename()` to move/rename assets within Pimcore

### Daemon Improvements
- [ ] Allow daemon to flush batches dynamically based on event volume
- [ ] Configure poll/update time of daemon for different usecases
- [ ] Add optional configuration file support (e.g., YAML) for daemon
- [ ] Detect remote filesystem mounts (Flysystem) and handle them accordingly
- [ ] Build systemd service file for daemon to run automatically
- [ ] Improve JSON event output (optional custom timestamp formats)
- [ ] Support error logging and retries for failed asset operations
- [ ] Add support for `.bridgeignore` files to exclude patterns/filetypes
- [ ] Add `max_depth` setting to limit recursive folder watching
- [ ] Support block-level changes (advanced: use Pimcore Versions or Notes?)

### Architecture Improvements
- [ ] Allow configuration of daemon to use Pimcore Datahub API instead of local console
- [ ] Expose a minimal API controller inside AssetBridgeBundle for event ingestion
- [ ] Allow daemon instances to be configured dynamically via Pimcore Settings UI

### Cross-Platform Support
- [ ] Expose a Symfony command to control daemon (start/stop)
- [ ] Add support for Windows/Mac binaries in the future

### Monitoring & Metrics
- [ ] Add basic metrics (number of processed events, error counts)

### QA / Maintenance
- [ ] Write basic test coverage for daemon and Symfony command
- [ ] Publish bundle to Packagist for wider adoption

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0).  
See the [LICENSE](LICENSE) file for details.

---

Built with ❤️ by [cleggypdc](https://github.com/cleggypdc)

