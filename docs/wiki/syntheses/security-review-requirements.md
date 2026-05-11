---
type: synthesis
created: 2026-05-11
tags: [terachat, security, review, cryptography, applied-cryptographer, audit]
sources: [tera-core-spec, tera-gov-spec, tech-debt-registry, improvement-plan-2026-05-11]
status: resolved
resolves: "Điểm yếu #7 — Thiếu Applied Cryptographer"
---

# Security Review Requirements

TeraChat là sản phẩm Zero-Knowledge — một lỗi crypto duy nhất phá vỡ toàn bộ sản phẩm. Security review không phải là "nice to have" — nó là deployment blocker.

## Ai Review?

| Phase | Reviewer | Expertise Required |
|-------|----------|--------------------|
| **Prototype** | Senior Rust Engineer + Security-conscious code review | Memory safety, basic crypto correctness |
| **Phase 1 (MVP)** | Security Architect (internal) | MLS protocol, JWT, OIDC flows |
| **Phase 2A (PQ-KEM)** | **Applied Cryptographer (external)** | Post-quantum cryptography, hybrid KEM |
| **Phase 2B (Mesh)** | Distributed Systems Security Engineer | P2P security, BLE attack surface |
| **Phase 2D (AI)** | ML Security Engineer | Model integrity, PII redaction accuracy |
| **Phase 3A (Gov)** | **Third-party Audit Firm** | ISO 27001, penetration testing |
| **Phase 3B (Mil)** | **Government-cleared Auditor** | Air-gapped security, side-channel |

## Review Gates Per Phase

### Prototype — Internal Review

| Check | Tool/Method | Pass Criteria |
|-------|-------------|---------------|
| No `unsafe` without justification | `cargo geiger` | 0 unsafe blocks (or documented) |
| Dependencies audited | `cargo audit` | 0 critical CVE |
| Secrets in code | `gitleaks detect` | 0 findings |
| Basic crypto: key never logged | Manual review | 0 `println!("{:?}", key)` |

### Phase 1 MVP — Security Architect Review

| Check | Tool/Method | Pass Criteria |
|-------|-------------|---------------|
| `ffi_boundary!` covers ALL `extern "C"` | `grep -r "extern \"C\"" source/` cross-ref với `ffi_boundary!` usage | 100% coverage |
| ZeroizeOnDrop on ALL key types | `cargo miri test --test zeroize_verification` | 100% pass |
| License JWT validation chain | Manual crypto review | Không thể bypass |
| OIDC token validation | Manual review + OWASP checklist | Không có IDP mix-up, không có token replay |
| Relay: verify ciphertext only | Penetration test | Relay không có plaintext tại bất kỳ endpoint nào |
| MLS epoch rotation | Manual review | Old member không decrypt được new messages |

### Phase 2A — Applied Cryptographer (EXTERNAL)

Đây là gate quan trọng nhất. Cần thuê external cryptographer review:

| Check | Method | Pass Criteria |
|-------|--------|---------------|
| ML-KEM-768 implementation | Formal code review | Đúng FIPS 203 draft |
| Hybrid KEM: security proof | Mathematical review | Không weaker hơn mạnh nhất trong hai |
| Key encapsulation flow | Code review | Không leak, không reuse nonce |
| Composite key derivation | HKDF audit | Salt unique, info string correct |
| Side-channel resistance | `cargo miri` + manual `#[constant_time]` audit | Không timing leak |

### Phase 2D — AI Security Review

| Check | Method | Pass Criteria |
|-------|--------|---------------|
| SanitizedPrompt: 0 PII false negative | Test với 1000 mẫu PII thật (SSN, NIF, email, phone) | 100% detection |
| Model integrity: BLAKE3 hash on load | Code review | Không thể load model không signed |
| Egress guard: 0 raw embedding output | Code review + test | Tất cả output qua sanitization |
| ONNX model isolation | Sandbox audit | Model không truy cập filesystem/network |

### Phase 3A — Third-Party Audit

- **ISO 27001 certification audit**
- **Penetration test** (external firm): tấn công relay, client, IPC
- **OWASP MASVS** cho mobile apps
- **Supply chain audit**: `cargo vet`, `cargo deny`, SBOM verification

## Khi Nào Cần Thuê?

```
PROTOTYPE: Không cần — internal review đủ
PHASE 1:   Không cần — Security Architect đủ
PHASE 2A:  BẮT BUỘC — external Applied Cryptographer
PHASE 2D:  NÊN — ML Security Engineer (có thể internal nếu team có)
PHASE 3A:  BẮT BUỘC — third-party audit firm
PHASE 3B:  BẮT BUỘC — government-cleared auditor
```

## Budget Ước Tính

| Review | Estimated Cost |
|--------|---------------|
| Applied Cryptographer (Phase 2A) | $15,000-$30,000 (2-3 tuần) |
| ML Security Engineer (Phase 2D) | $10,000-$20,000 (1-2 tuần) |
| ISO 27001 Audit (Phase 3A) | $30,000-$60,000 (3-6 tháng) |
| Penetration Test (Phase 3A) | $20,000-$40,000 (2-3 tuần) |
| Gov/Military Audit (Phase 3B) | $50,000-$100,000 (tùy scope) |

## 🧠 Design Decision

**Tại sao Applied Cryptographer là external bắt buộc?** → Internal team dù giỏi đến đâu cũng có blind spot — người thiết kế không thể audit chính mình. PQ-KEM hybrid là lĩnh vực hẹp cần chuyên gia đã từng implement hoặc audit ít nhất 1 hệ thống tương tự. Một lỗi trong key encapsulation → toàn bộ Zero-Knowledge bị phá vỡ. Chi phí $15K-$30K rẻ hơn nhiều so với thiệt hại nếu có lỗ hổng bị khai thác.
