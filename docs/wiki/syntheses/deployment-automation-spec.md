---
type: synthesis
created: 2026-05-11
tags: [terachat, deployment, automation, it-admin, pilot, spec]
sources: [phase-framework, narrowed-phase-1-mvp, tera-core-spec, enterprise-license-model]
status: resolved
resolves: "Điểm yếu #9 — Không có deployment automation spec"
---

# Deployment Automation Specification

**Đây là spec quan trọng nhất cho Phase 1.** Nó định nghĩa trải nghiệm của IT admin — người quyết định pilot thành công hay thất bại.

## Goal

> Một IT admin (không có DevOps background) deploy được TeraChat trong **≤ 30 phút** và demo cho Board trong cùng buổi sáng.

## Deployment Flow

```
1. IT Admin mua license trên terachat.io → nhận license key qua email
2. Mở terminal trên Mac mini / Linux server
3. Chạy MỘT lệnh:
   curl -fsSL https://install.terachat.io | sudo bash
4. Script tự động:
   ├─ Detect OS (macOS / Ubuntu / Debian)
   ├─ Download TeraRelay binary (~15MB)
   ├─ Tạo system service (launchd / systemd)
   ├─ Tự sinh TLS cert (Let's Encrypt hoặc self-signed)
   ├─ Khởi tạo SQLite WAL database
   ├─ Tải OPA policy mặc định
   ├─ Hiển thị admin URL + credentials tạm
   └─ In ra: "TeraChat ready at https://your-domain.com"
5. IT Admin mở browser → Admin Console
6. Nhập license key → activate
7. Tạo workspace đầu tiên → mời nhân viên qua email
8. Nhân viên nhận email → tải app → đăng nhập qua Google/Azure
9. XONG. Demo được.
```

## Yêu cầu kỹ thuật

### TeraRelay Single Binary

```yaml
binary: terachat-relay
size: < 20MB (compressed)
dependencies: none (static binary)
platforms: [macOS (x86_64 + arm64), Ubuntu 22.04+, Debian 12+]
startup_time: < 5 giây
memory_idle: < 50MB
memory_100_users: < 256MB
```

### Install Script

```bash
#!/bin/bash
# install.terachat.io
# Yêu cầu: curl, systemd (Linux) hoặc launchd (macOS)

set -e

# 1. Detect platform
OS=$(uname -s)
ARCH=$(uname -m)

# 2. Download binary
echo "Downloading TeraRelay..."
curl -fsSL "https://dl.terachat.io/relay/latest/terachat-relay-${OS}-${ARCH}.tar.gz" | tar xz

# 3. Verify checksum
sha256sum -c terachat-relay.sha256

# 4. Install binary
sudo mv terachat-relay /usr/local/bin/
sudo chmod +x /usr/local/bin/terachat-relay

# 5. Setup service
if [ "$OS" = "Darwin" ]; then
    # macOS launchd
    cat <<EOF | sudo tee /Library/LaunchDaemons/com.terachat.relay.plist
    # ... launchd config ...
EOF
    sudo launchctl load /Library/LaunchDaemons/com.terachat.relay.plist
elif [ "$OS" = "Linux" ]; then
    # Linux systemd
    cat <<EOF | sudo tee /etc/systemd/system/terachat-relay.service
    # ... systemd config ...
EOF
    sudo systemctl daemon-reload
    sudo systemctl enable --now terachat-relay
fi

# 6. Bootstrap
sudo terachat-relay bootstrap --auto-tls --admin-email="${ADMIN_EMAIL}"

# 7. Done
echo ""
echo "TeraChat ready at https://$(hostname)"
echo "Admin Console: https://$(hostname)/admin"
echo "Admin credentials: saved to /etc/terachat/admin_credentials.txt"
```

### Health Check Endpoint

```protobuf
// GET /health
// Response:
{
  "status": "healthy",
  "version": "1.0.0",
  "uptime_seconds": 3600,
  "active_users": 15,
  "db_size_mb": 42,
  "tls_valid_until": "2027-05-11",
  "license": {
    "status": "valid",
    "seats_used": 15,
    "seats_max": 50,
    "valid_until": "2026-12-31"
  }
}
```

## Failure Modes & Recovery

| Scenario | Behavior | IT Admin Action |
|----------|----------|-----------------|
| Port 443 already in use | Detect pre-flight → error message + suggest `--port 8443` | Change port or stop existing service |
| TLS cert generation fails | Fallback to self-signed + warning | Manual cert upload in Admin Console |
| Low disk space (< 500MB) | Warning, continue with reduced features | Free up disk space |
| License invalid | Relay starts but blocks user connections | Enter valid license in Admin Console |
| DB corruption on restart | Auto `PRAGMA integrity_check` + rebuild from WAL | None (automatic) |

## Admin Console (Minimal)

IT Admin có thể làm những việc sau từ browser:

1. **Dashboard:** Active users, messages sent, storage used, license status
2. **Users:** Mời/khóa user, map SAML attributes → roles
3. **Workspaces:** Tạo workspace, set authority scope (branch/department)
4. **License:** Xem trạng thái license, ngày hết hạn
5. **Deploy .tapp:** Tải .tapp file → deploy đến region/department
6. **Audit Export:** Xuất audit log → PDF signed
7. **Support:** Link đến runbook, contact TeraChat support

## Tiêu chí kiểm thử deployment

Kịch bản test bắt buộc:

| # | Test | Target |
|---|------|--------|
| 1 | Fresh Ubuntu 22.04 → chạy install script → TeraChat hoạt động | < 5 phút |
| 2 | Fresh macOS 14 → chạy install script → TeraChat hoạt động | < 5 phút |
| 3 | IT Admin không biết DevOps → deploy thành công chỉ với README | < 30 phút |
| 4 | Kill relay process → auto restart → không mất data | < 10 giây |
| 5 | Hết hạn license → admin console cảnh báo → user vẫn chat được (grace period) | Grace 30 ngày |
| 6 | 50 users online → relay memory < 256MB, CPU < 20% | Load test |

## 🧠 Design Decision

**Tại sao install script qua curl pipe bash?** → Đây là pattern quen thuộc với IT admin (Docker, Homebrew, Oh My Zsh đều dùng). Một binary + một lệnh = zero friction. Trade-off: cần ký script với GPG để chống tampering. Alternative: Homebrew (macOS) + apt repo (Linux) sẽ thêm sau Phase 1.

---

## Backup & Recovery

### Automated Backup

```bash
# Chạy mỗi 6h qua cron/systemd timer
terachat-relay backup --output /mnt/nas/terachat-backup/
# Tạo: hot_dag.db.backup + cold_state.db.backup + config.toml.backup
# Dùng: sqlite3 .backup (online, không lock)
# Verify: BLAKE3 checksum trên mỗi backup file
```

### Recovery Procedure

```bash
terachat-relay restore --from /mnt/nas/terachat-backup/2026-05-12T06:00:00Z/
# 1. Verify backup integrity (BLAKE3 checksum)
# 2. Stop relay gracefully
# 3. Restore databases từ backup
# 4. Replay WAL từ backup time → current
# 5. Start relay
# 6. Health check → verify active
```

### Backup Retention Policy

| Tier | Retention | Frequency |
|------|-----------|-----------|
| Standard | 7 ngày | Mỗi 6h |
| Enterprise | 30 ngày | Mỗi 1h |
| Gov/Military | 90 ngày + offline copy | Mỗi 15 phút |

---

## Monitoring Stack

| Component | Tool | Phase |
|-----------|------|-------|
| Metrics | Prometheus (scrape `/metrics` mỗi 15s) | Phase 1 |
| Logs | Loki + Promtail (JSON format) | Phase 1 |
| Dashboards | Grafana (import từ `grafana/dashboards/`) | Phase 1 |
| Alerts | Grafana AlertManager | Phase 1 |

### Critical Metrics

```
terachat_messages_delivered_total
terachat_messages_latency_seconds{quantile="0.5,0.95,0.99"}
terachat_active_connections
terachat_db_size_bytes
terachat_wal_checkpoint_seconds
terachat_mls_epoch_rotation_seconds
terachat_license_days_remaining
terachat_tls_cert_days_remaining
terachat_backup_last_success_timestamp
```

### Critical Alerts

| Alert | Condition | Severity | Channel |
|-------|-----------|----------|---------|
| Relay down | `up == 0` > 60s | P0 | PagerDuty |
| TLS expiry < 7 days | `tls_expiry_days < 7` | P0 | PagerDuty |
| Disk > 80% | `disk_used_pct > 80` | P1 | Slack |
| License expiry < 30 days | `license_days < 30` | P1 | Slack |
| Message latency > 5s (p95) | `p95 > 5` sustained 5m | P1 | Slack |
| DB size > 80% max | `db_size / db_max > 0.8` | P2 | Email |
| Backup failure > 12h | `backup_success == 0` > 12h | P1 | Slack |

---

## Staging Environment

```
staging.terachat.internal:
  - Bản sao của production config (không phải production data)
  - Deploy tự động từ main branch khi CI pass
  - Smoke test sau deploy:
    1. GET /health → 200
    2. Gửi 10 messages giữa 2 test users → all delivered < 2s
    3. Create channel + add member + remove member
    4. Kill relay → auto restart < 10s → 0 data loss
    5. Backup + restore cycle → no corruption
  - Dữ liệu test được reset mỗi tuần
  - Không chạy trên cùng machine với production
```

---

## Configuration File Reference

```toml
# /etc/terachat/config.toml
[server]
listen = "0.0.0.0:443"
health_port = 8443

[tls]
provider = "acme"            # acme | self-signed | manual
domain = "chat.mycompany.com"
acme_email = "it@mycompany.com"

[storage]
data_dir = "/var/lib/terachat"
max_db_size_gb = 100
backup_dir = "/mnt/nas/terachat-backup"

[license]
key = "TERA-XXXX-XXXX-XXXX"
enterprise_tier = "standard"

[auth]
oidc_provider = "google"     # google | azure | keycloak
oidc_client_id = "..."

[mesh]
enabled = false              # Phase 2B
mesh_port = 0

[monitoring]
prometheus_port = 9090
log_level = "info"           # debug | info | warn | error
log_format = "json"          # json | text (json for Loki)
```

---

## Infrastructure Cost Model

| Component | Phase | Provider | Cost/month |
|-----------|-------|----------|------------|
| TeraRelay VPS | Prototype | Hetzner CX22 (2 vCPU, 4GB) | ~$5-8 |
| Blob Storage | Prototype | Cloudflare R2 (S3, no egress) | ~$0 (10GB free) |
| CI/CD | Phase 0 | GitHub Actions | ~$0-8 |
| Monitoring | Phase 1 | Grafana Cloud free tier | ~$0 |
| Secrets | Phase 1 | Doppler free tier | ~$0 |
| Domain + DNS | Prototype | Cloudflare | ~$10/year |
| EV Code Signing | Phase 2 | DigiCert KeyLocker | ~$500/year |
| Apple Developer | Prototype | Apple Developer Program | ~$99/year |
| **Total (Prototype-Phase 1)** | | | **~$30-50/month** |
