# Spec-Enterprise-Secure-Enclave.md — TeraChat Enterprise Platform

```yaml
# DOCUMENT IDENTITY
id: "TERA-ENCLAVE"
title: "TeraChat — Enterprise Secure Enclave & AI Security Specification"
version: "1.0.0"
status: "ACTIVE"
audience: "Security Engineer, AI Architect, Enterprise Compliance Officer"
purpose: "Đặc tả kỹ thuật các cơ chế bảo mật cấp độ Enclave, bảo vệ dữ liệu PII và giới hạn quyền hạn của AI khi hoạt động nội bộ hệ thống."
depends_on: ["TERA-CORE", "TERA-GOV"]
```

## §1 — ARCHITECTURAL INVARIANTS & AUDIT RESOLUTIONS (SECURE ENCLAVE)

### 1.1 AI PII Redaction vs. BankFeeds / CRM Data
**Constraint:** AI Micro-NER models often fail to identify regional-specific Private Identifiable Information (PII) like localized banking formats, posing an injection vulnerability against the underlying AI analysis sessions.
**Resolution:** The `SanitizedPrompt` newtype strictly guarantees non-reversible PII redaction prior to AI payload submission. The embedded NER model’s entity dictionary is configured exclusively via tenant-specific `DomainPiiPolicy` schemas (e.g., specialized regex sets for Regional Account Syntax). Consequently, Zero-Knowledge compliance is structurally enforced across all LLM inference operations.
