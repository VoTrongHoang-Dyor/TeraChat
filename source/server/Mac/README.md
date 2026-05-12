# TeraChat Mac Server — launchd Configuration
#
# Runs Rust Core daemon as a macOS LaunchAgent per ADR-001.
# This configuration is for the Solo/SME deployment tier (Mac mini).

## Prerequisites
# - macOS 13+ (Apple Silicon)
# - Rust 1.75.0 (installed via rustup)
# - License JWT from TeraChat
# - TLS certificates generated

## Service Configuration

### LaunchAgent plist

The daemon runs as a user-level LaunchAgent. For Enterprise tier,
use LaunchDaemon instead (requires root).

Location: `~/Library/LaunchAgents/com.terachat.core.plist`

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.terachat.core</string>
    
    <key>ProgramArguments</key>
    <array>
        <string>/usr/local/bin/terachat-core</string>
        <string>--config</string>
        <string>/etc/terachat/config.toml</string>
    </array>
    
    <key>RunAtLoad</key>
    <true/>
    
    <key>KeepAlive</key>
    <dict>
        <key>SuccessfulExit</key>
        <false/>
    </dict>
    
    <key>StandardOutPath</key>
    <string>/var/log/terachat/core.log</string>
    
    <key>StandardErrorPath</key>
    <string>/var/log/terachat/core.error.log</string>
    
    <key>EnvironmentVariables</key>
    <dict>
        <key>RUST_LOG</key>
        <string>info</string>
        <key>TERA_DB_PATH</key>
        <string>/var/lib/terachat/data</string>
    </dict>
    
    <key>SoftResourceLimits</key>
    <dict>
        <key>NumberOfFiles</key>
        <integer>4096</integer>
    </dict>
</dict>
</plist>
```

### Core Configuration (config.toml)

Location: `/etc/terachat/config.toml`

```toml
[server]
# IPC socket path — UDS for desktop clients
ipc_socket = "/var/run/terachat/core.sock"
# gRPC listen address for local clients
grpc_listen = "127.0.0.1:50051"

[database]
hot_dag_path = "/var/lib/terachat/data/hot_dag.db"
cold_state_path = "/var/lib/terachat/data/cold_state.db"
# SQLCipher key is derived from Secure Enclave — never stored here
wal_mode = true

[relay]
# TeraRelay blind routing
listen = "0.0.0.0:443"
tls_cert = "/etc/terachat/certs/relay.crt"
tls_key = "/etc/terachat/certs/relay.key"

[mesh]
ble_enabled = true
wifi_direct_enabled = true
# EMDP emergency protocol TTL (seconds)
emdp_ttl = 3600

[ai]
# BYOM endpoint — configurable per tenant
endpoint = "http://localhost:11434/v1/chat/completions"
# PII Redaction is MANDATORY — cannot be disabled
pii_redaction_enabled = true

[telemetry]
# OpenTelemetry — optional
otel_endpoint = ""
otel_sample_rate = 0.1
# Prometheus metrics (local only)
metrics_listen = "127.0.0.1:9100"

[security]
# Secure Enclave key tag
enclave_key_tag = "com.terachat.device-identity"
# mTLS for federation
mtls_enabled = false
```

## Deployment Steps

```bash
# 1. Build from source
cd source/core
cargo build --release --locked

# 2. Install binary
sudo cp target/release/terachat-core /usr/local/bin/

# 3. Create directories
sudo mkdir -p /etc/terachat/certs
sudo mkdir -p /var/lib/terachat/data
sudo mkdir -p /var/log/terachat
sudo mkdir -p /var/run/terachat

# 4. Copy configuration
sudo cp config/mac-server/config.toml /etc/terachat/

# 5. Generate TLS certificates (self-signed for dev)
openssl req -x509 -newkey ed25519 -keyout /etc/terachat/certs/relay.key \
  -out /etc/terachat/certs/relay.crt -days 365 -nodes \
  -subj "/CN=terachat-relay"

# 6. Install LaunchAgent
cp com.terachat.core.plist ~/Library/LaunchAgents/
launchctl load ~/Library/LaunchAgents/com.terachat.core.plist

# 7. Verify
launchctl list | grep terachat
curl -s http://127.0.0.1:9100/metrics | head -5
```

## Health Checks

```bash
# Service status
launchctl list com.terachat.core

# Logs
tail -f /var/log/terachat/core.log

# Metrics
curl http://127.0.0.1:9100/metrics

# IPC socket
ls -la /var/run/terachat/core.sock
```
