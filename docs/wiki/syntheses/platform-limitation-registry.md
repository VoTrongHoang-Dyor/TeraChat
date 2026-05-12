---
type: synthesis
created: 2026-05-12
tags: [terachat, platform, limitation, disclosure, cross-platform, xplat]
sources: [tech-debt-registry, terachat-technical-audit-2026, platform-rollout-phasing]
status: active
resolves: "Điểm yếu #7 — Cross-platform limitations chưa được registry tập trung"
---

# Platform Limitation Registry

**Quyết định:** Mọi giới hạn nền tảng phải được documented, disclosed, và tracked. Không có "surprise limitation" khi khách hàng đã ký contract.

---

## Nguyên tắc Disclosure

1. **Pre-contract disclosure:** Mọi limitation phải có trong `Pricing_Packages.html` trước khi khách hàng ký
2. **Tier impact:** Mỗi limitation được đánh dấu tier bị ảnh hưởng (Standard / Enterprise / Gov)
3. **No workaround = clearly stated:** Không hứa sẽ fix nếu Apple/Google/Huawei không cung cấp API

---

## Limitation Registry

### XPLAT-01 — iOS W^X → wasm3 Performance Penalty

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | iOS (tất cả phiên bản) |
| **Root Cause** | iOS W^X policy — không cho phép JIT compilation trong user-space app |
| **Impact** | `.tapp` chạy trên `wasm3` interpreter, chậm hơn 10-100x so với `wasmtime` JIT trên Desktop |
| **Affected Tier** | Tất cả tier trên iOS |
| **Workaround** | Fuel metering (deterministic limit) thay vì wall-clock timeout. .tapp publishers phải test trên cả hai engine. |
| **Disclosure** | "iOS .tapp performance may differ from Desktop. Compute-heavy .tapps (BankFeeds reconciliation, Finance aggregation) should be tested on iOS before deployment." |
| **Status** | ⚠️ Permanent limitation — Apple policy, cannot fix |

### XPLAT-02 — AWDL + Personal Hotspot Conflict

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | iOS |
| **Root Cause** | AWDL (Apple Wireless Direct Link) bị disable khi iOS làm Personal Hotspot |
| **Impact** | Khi user share internet từ iPhone, EMDP Mesh coordinator role không hoạt động — chỉ BLE (~4.75kbps) available |
| **Affected Tier** | Tất cả tier — mesh degradation |
| **Workaround** | Không có. Document trong EMDP user guide. Gov-tier SLA không cover iOS hotspot scenarios. |
| **Disclosure** | "EMDP Mesh performance is degraded when iOS device is used as Personal Hotspot. For optimal mesh performance, use a dedicated internet connection." |
| **Status** | ⚠️ Permanent limitation — Apple framework behavior |

### XPLAT-03 — Huawei HMS Push Limitation

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | Huawei (HarmonyOS / EMUI) |
| **Root Cause** | HMS Push không hỗ trợ `data-only` message type |
| **Impact** | SCIM < 30s SLA không thể đảm bảo. Polling-based fallback = 4h delay. Gov/Military tier impossible trên Huawei. |
| **Affected Tier** | Huawei devices: Standard tier only (4h SLA). Enterprise và Gov tier không available. |
| **Workaround** | Polling mỗi 4h. Huawei users được thông báo rõ ràng trong app. |
| **Disclosure** | "Due to HMS Push platform limitations, real-time SCIM (<30s) is not available on Huawei devices. Huawei devices operate on Standard tier with 4-hour sync interval. For Enterprise SLA, please use Android (Google Mobile Services) or iOS." |
| **Status** | ⚠️ Permanent limitation — HMS platform constraint |

### XPLAT-04 — Linux mlock() Refusal

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | Linux (air-gapped deployment) |
| **Root Cause** | `mlock()` bị từ chối nếu process không có `CAP_IPC_LOCK` |
| **Impact** | Key material có thể bị swap ra disk — vi phạm Zero-Knowledge promise |
| **Workaround** | Exit code 78 + syslog message. Admin Console alert hướng dẫn set `ulimit -l` hoặc `CAP_IPC_LOCK`. |
| **Disclosure** | "Air-gapped Linux deployment requires CAP_IPC_LOCK capability. Without it, key material may be swapped to disk. Setup guide includes step-by-step capability configuration." |
| **Status** | ⚠️ Partially resolved — documented in TERA-CORE §11.5 |

### XPLAT-05 — Linux Tauri WebView SharedArrayBuffer

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | Linux Desktop (Tauri + WebKitGTK) |
| **Root Cause** | `SharedArrayBuffer` yêu cầu `COOP: same-origin` + `COEP: require-corp` HTTP headers |
| **Impact** | Zero-copy Data Plane không hoạt động trên Linux Desktop. Silent degrade về JSON serialization (chậm hơn 50-100x). |
| **Affected Tier** | Linux Desktop users |
| **Workaround** | Rust Core local HTTP server set COOP+COEP headers. Nếu WebKitGTK không hỗ trợ → fallback JSON. Document performance difference. |
| **Disclosure** | "Linux Desktop uses JSON serialization for IPC instead of SharedArrayBuffer. Performance is sufficient for messaging but large file transfers may be slower than macOS/Windows. Full SAB support requires WebKitGTK 2.40+." |
| **Status** | ⚠️ Resolved for WebKitGTK 2.40+. Older versions: documented degradation. |

### XPLAT-06 — Windows EV Code Signing

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | Windows |
| **Root Cause** | EV Code Signing cần hardware token (USB HSM). Cloud CI runners không hold được EV token. |
| **Impact** | Windows signed binary cần dedicated self-hosted runner với physical YubiKey. CI/CD pipeline cho Windows phức tạp hơn các nền tảng khác. |
| **Affected Tier** | Windows distribution |
| **Workaround** | Dedicated Mac mini self-hosted runner + YubiKey FIPS USB. Manual PIN entry hoặc TPM 2.0 Auto-unseal cho CI. |
| **Disclosure** | "Windows builds require hardware code signing. Release cadence for Windows may be 24-48h behind macOS/iOS due to signing process." |
| **Status** | ⚠️ Permanent limitation — Microsoft EV requirement |

### XPLAT-07 — HarmonyOS .waot AOT Portability

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | HarmonyOS |
| **Root Cause** | `.waot` AOT compilation tạo device-specific native code. Huawei không đảm bảo cross-device AOT portability. |
| **Impact** | WasmParity CI cần validate cả JIT và AOT paths. Không thể đảm bảo `.tapp` từ một HarmonyOS device chạy giống hệt trên device khác. |
| **Affected Tier** | HarmonyOS devices |
| **Workaround** | AOT validation trong CI cho 3 HarmonyOS reference devices. Limit .tapp distribution trên HarmonyOS cho đến khi Huawei document AOT portability. |
| **Status** | ⚠️ Flagged — pending Huawei documentation |

### XPLAT-08 — Android OEM Background Kill

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | Android (MIUI, ColorOS, OriginOS, Funtouch OS) |
| **Root Cause** | OEM battery management kill Background Task không callback. Phụ thuộc vào user bật "Autostart" + "Battery Optimization Bypass". |
| **Impact** | Mesh BLE + Background Sync tê liệt trên ~40% Android Asia market nếu user không cấu hình đúng. |
| **Affected Tier** | Tất cả tier trên Android OEM bị ảnh hưởng |
| **Workaround** | Android ForegroundService với persistent notification. Onboarding flow guide user bật "Autostart" + disable battery optimization. Detection: nếu service bị kill 3 lần trong 1h → notification "TeraChat needs background permission." |
| **Disclosure** | "Android devices from Xiaomi, Oppo, Vivo require additional setup for background sync and mesh networking. Setup wizard will guide you through the process (~2 minutes). Without setup, offline mesh and background sync will not function." |
| **Status** | ⚠️ Partially resolved — ForegroundService approach documented |

### XPLAT-09 — Non-Deterministic Builds

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | Tất cả nền tảng |
| **Root Cause** | Rust + C/Assembly (SQLCipher, `ring` crate) cross-compiled qua môi trường khác nhau tạo ra non-deterministic binaries |
| **Impact** | Không thể verify `.ipa` binary giống hệt source code. Vi phạm ISO 27001 A.8.4. Gov/Military audit không thể pass. |
| **Affected Tier** | Gov/Military (không ảnh hưởng Standard/Enterprise) |
| **Workaround** | Nix Flakes + YubiKey HSM on-premise CI. Build trong network-isolated Nix container. SBOM + cosign attestation cho mọi artifact. |
| **Disclosure** | "Hermetic (deterministic) builds are available for Gov/Military tier customers with on-premise CI. Standard and Enterprise tiers use cloud CI with signed SBOM attestation." |
| **Status** | ⚠️ Resolved for Gov tier via Hermetic Build Forge (Tech_Debt §5.6). Standard tier: documented trade-off. |

### XPLAT-10 — iOS AWDL + Personal Hotspot (Same as XPLAT-02)

| Thuộc tính | Giá trị |
|------------|--------|
| **Platform** | iOS |
| **Root Cause** | Trùng với XPLAT-02 |
| **Status** | ⚠️ Merged into XPLAT-02 |

---

## Platform SLA Matrix

| Platform | E2EE Messaging | Mesh | .tapp Performance | SCIM Real-time | Tier Available |
|----------|---------------|------|-------------------|----------------|----------------|
| **macOS** | ✅ Full | ✅ Full (Phase 2B) | ✅ wasmtime JIT | ✅ <30s | Standard, Enterprise, Gov |
| **iOS** | ✅ Full | ⚠️ Degraded w/ Hotspot | ⚠️ wasm3 interpreter | ✅ <30s | Standard, Enterprise |
| **Android (GMS)** | ✅ Full | ⚠️ OEM setup required | ✅ wasmtime JIT (some) | ✅ <30s | Standard, Enterprise |
| **Android (MIUI/ColorOS)** | ✅ Full | ⚠️ Requires ForegroundService | ✅ wasmtime JIT (some) | ⚠️ Delayed without setup | Standard |
| **Windows** | ✅ Full | ✅ Full (Phase 2B) | ✅ wasmtime JIT | ✅ <30s | Standard, Enterprise |
| **Linux** | ✅ Full | ✅ Full (Phase 2B) | ✅ wasmtime JIT | ✅ <30s | Standard, Enterprise, Gov |
| **Huawei** | ✅ Full | ❌ Not supported | ⚠️ wasm3 interpreter | ❌ 4h polling | Standard only |

---

## Testing Requirements Per Platform

| Platform | Test Device Required | Phase |
|----------|---------------------|-------|
| macOS | Mac mini M2 Pro (development machine) | Prototype |
| iOS | iPhone 12+ (BLE 5.0), iPhone 14+ (prod sim) | Prototype |
| Android GMS | Google Pixel (reference) | Phase 2A |
| Android OEM | Samsung Galaxy, Xiaomi, Oppo/Vivo | Phase 2A |
| Windows | Windows 11 machine (EV signing) | Phase 2D |
| Linux | Ubuntu 22.04+ VM | Phase 3A |
| Huawei | Huawei device with HMS | Phase 3A |

---

*XPLAT-REGISTRY v1.0.0 · 2026-05-12 · Created from Technical Audit recommendations*
