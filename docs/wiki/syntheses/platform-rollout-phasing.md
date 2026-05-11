---
type: synthesis
created: 2026-05-11
tags: [terachat, platform, rollout, phasing, ios, macos, android, windows, linux]
sources: [terachat-architecture-overview, phase-framework, tech-debt-registry]
status: resolved
resolves: "Điểm yếu #3 — 9 nền tảng từ day 1"
---

# Platform Rollout Phasing

**Quyết định:** macOS + iPhone trước (cùng hệ sinh thái Apple). Mỗi nền tảng mới chỉ được thêm sau khi nền tảng trước đó đã có pilot ổn định.

## Nguyên tắc

1. **Một cặp nền tảng mỗi phase** — không thêm ồ ạt
2. **Cùng hệ sinh thái trước** — Apple (macOS+iPhone), sau đó Google (Android+Oppo), sau đó Microsoft (Windows), sau đó Linux, cuối cùng Huawei
3. **Phải có khách hàng yêu cầu** — không thêm nền tảng chỉ vì "có thể"
4. **Mỗi nền tảng mới = 1 tháng thêm vào timeline**

## Lộ trình

```
PHASE 1 MVP (Tháng 1-4)
├─ macOS (Desktop) ← PRIMARY — IT admin deploy ở đây
└─ iPhone (Mobile) ← PRIMARY — người dùng chính

  ↓ Sau khi ký pilot đầu tiên ↓

PHASE 2 MỞ RỘNG (Tháng 5-12)
├─ Android (Mobile) ← Google Mobile Services
├─ Oppo (Mobile) ← ColorOS — thị trường Việt Nam/Á
└─ Windows (Desktop) ← Doanh nghiệp Windows-heavy

  ↓ Sau khi có 10+ khách hàng ↓

PHASE 3 TOÀN CẦU (Tháng 13-24)
├─ Linux (Desktop) ← Gov/Defense/Education
├─ Huawei (Mobile) ← HMS — thị trường Trung Quốc
├─ Mac Server ← On-premise deployment
└─ Physical Server ← Air-gapped deployment
```

## Tại sao macOS + iPhone trước?

| Lý do | Chi tiết |
|-------|----------|
| **Cùng hệ sinh thái** | Cùng Secure Enclave API, cùng build chain (Xcode), cùng ngôn ngữ Swift cho bindings |
| **Cùng developer** | 1 team có thể làm cả desktop + mobile (SwiftUI Catalyst tái dùng code) |
| **Thị trường enterprise** | macOS phổ biến ở startup/tech, iPhone thống trị mobile enterprise |
| **Bảo mật nhất quán** | Secure Enclave behavior giống nhau trên cả hai — không cần code riêng |
| **Test đơn giản hơn** | 2 nền tảng, 1 hệ điều hành, 1 bộ test |

## Chi phí mỗi nền tảng thêm vào

| Nền tảng | Effort thêm (tháng) | Lý do |
|----------|---------------------|-------|
| Android | 1.5 | JNI/StrongBox API khác, OEM fragmentation (Samsung ≠ Xiaomi ≠ Pixel) |
| Oppo | 0.5 | Giống Android + ColorOS background kill mitigation |
| Windows | 1.5 | TPM 2.0 API khác, EV signing, MSI installer, SCM service |
| Linux | 1.0 | systemd, AppArmor/SELinux, .deb/.rpm packaging |
| Huawei | 2.0 | HMS ecosystem hoàn toàn khác, ArkUI, napi-harmony bridge |
| Mac Server | 0.5 | Giống macOS nhưng headless, launchd, HA config |
| Physical Server | 1.0 | Air-gapped, Shamir ceremony, HSM integration |

## Rủi ro nếu làm 9 nền tảng từ đầu

1. **Phân tán nguồn lực** — team 5 người không thể test 9 nền tảng
2. **Bug platform-specific** — mỗi OEM Android có behavior riêng (XPLAT-08)
3. **Không đủ thiết bị test** — cần mua 20+ devices để test thật
4. **CI/CD bùng nổ** — 9 nền tảng × 4 target (debug/release x arm/x86) = 36 build jobs

## Khi nào thêm nền tảng mới?

Điều kiện để mở khóa nền tảng tiếp theo:

- [ ] Nền tảng hiện tại có 3+ khách hàng trả tiền, chạy ổn định 30+ ngày
- [ ] Có ít nhất 1 khách hàng YÊU CẦU nền tảng mới (không phải "nice to have")
- [ ] CI pipeline cho nền tảng mới pass 100% unit tests
- [ ] Có thiết bị thật để test (không chỉ emulator)

## 🧠 Design Decision

**Tại sao Android không phải là ưu tiên số 2 dù thị phần lớn hơn iPhone?** → Ở phân khúc enterprise (khách hàng trả tiền), iPhone có thị phần cao hơn Android. OEM fragmentation (Oppo kill background task — XPLAT-08) khiến Android tốn nhiều effort test hơn. Bắt đầu với iOS nơi behavior nhất quán, sau đó expand sang Android khi đã có revenue.
