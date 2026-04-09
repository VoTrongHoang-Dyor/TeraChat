# TeraChat Project Structure

TeraChat-Project/
├── .agents/                                 # Không gian cấu hình cho AI Agents.
│   ├── .claude/                             # Metadata và cấu hình kỹ năng cho AI Agent.
│   │   ├── marketplace.json                 # Định nghĩa các plugin/tools từ marketplace.
│   │   ├── plugin.json                      # Cấu hình plugin nội bộ.
│   │   ├── settings.local.json              # Thiết lập môi trường local cho Agent.
│   │   └── skills-lock.json                 # Quản lý phiên bản các kỹ năng (skills) của AI.
│   ├── commands/                            # Các kịch bản lệnh tự động hóa quy trình.
│   │   ├── deloy.md                         # Quy trình và lệnh triển khai (Deployment pipeline).
│   │   ├── fix-issue.md                     # Prompt/Quy trình chuẩn để debug và vá lỗi.
│   │   └── review.md                        # Tiêu chuẩn Code Review định hướng bảo mật.
│   └── rules/                               # Bộ quy tắc ràng buộc (constraints) cho dự án.
│       ├── main.md                          # Nguyên tắc thiết kế cốt lõi (Core principles).
│       └── rust-build-resolver.md           # Quy tắc xử lý xung đột dependency khi build Rust Core.
├── assets/                                  # Tài nguyên tĩnh (UI assets, icons, sơ đồ kiến trúc).
├── config/                                  # Cấu hình hệ thống và môi trường chung.
│   └── Agent.md                             # Định nghĩa Persona và Scope cho System Agent.
├── docs/                                    # Trung tâm tài liệu thiết kế và đặc tả kỹ thuật (Specs).
│   ├── HTML/                                # Phiên bản web (compiled) của tài liệu đặc tả kĩ thuật và business.
│   │   ├── Design.html                      #
│   │   ├── Executive_Summary.html           # Là bản tóm tắt các điểm cốt lõi của một tài liệu kinh doanh dài.
│   │   ├── Function.html                    # Là tài liệu chứa toàn bộ chức năng của Sovereign Work OS (TeraChat).
│   │   ├── Introduction.html                # Là tài liệu giới thiệu tổng quan về sản phẩm.
│   │   ├── Pitch_Deck.html                  # Là tài liệu giúp giới thiệu tổng quan về ý tưởng, mô hình kinh doanh của sản phẩm
│   │   ├── Pricing_Packages.html            # Là tài liệu kinh doanh chiến lược nhằm cấu trúc các mức giá khác nhau của sản phẩm
│   │   ├── Spec-Client-IPC-And-UI-Bridge.html # UI/Native Domain: Đặc tả giao tiếp IPC và cầu nối FFI giữa UI và Core Rust.
│   │   ├── Spec-Core-Cryptography-And-Mesh.html # Crypto/Core Domain: Đặc tả E2EE, Zero-knowledge proof và mạng P2P Mesh.
│   │   ├── Spec-Dual-Sync-And-Local-Storage.html # Sync/Data Domain: Đặc tả cơ chế Offline-first, CRDT và mã hóa SQLite local.
│   │   ├── Spec-Ecosystem-And-Trust-Chain.html # Ecosystem Domain: Đặc tả vòng đời .tapp WASM Sandbox, Structured DataGrant và xác thực chữ ký Ed25519 từ Marketplace.
│   │   ├── Spec-Enterprise-Secure-Enclave.html # Infrastructure Domain: Cơ sở hạ tầng phần cứng, tích hợp HSM và RBAC doanh nghiệp.
│   │   ├── Spec-Identity-And-Governance.html # Identity Domain: Quản trị định danh DID, SCIM 2.0 sync và phân quyền tuyệt đối qua OPA Policy Engine.
│   │   ├── Spec-Wasm-Tapp-Runtime.html      # WASM Runtime Domain: Môi trường Sandbox thực thi Mini-App (Tapp) an toàn.
│   │   ├── TeraChat.html                    # Là bản tóm tắt về sản phẩm TeraChat
│   │   └── Web_Marketplace.html             # Là trang web chứa thông tin về TeraChat và là cửa hàng ứng dụng tiện ích .tapp
│   ├── MCP/                                 # Là file chứa các MCP giúp AI có thể tiếp cận với các kĩ thuật và công nghệ mới
│   ├── MD/                                  # Là file chứa các Markdown của các kiến trúc cốt lõi (Core Domains).
│   │   ├── Arrange.md                       # Là tài liệu chứa nội dung cập nhập và sửa đổi của các file MD khác
│   │   ├── Design.md                        # Là tài liệu chứa thiết kế tổng quan (UX/UI & System Architecture).
│   │   ├── Function.md                      # Là tài liệu chứa toàn bộ chức năng của Sovereign Work OS (TeraChat).
│   │   ├── Introduction.md                  # Là tài liệu chứa giới thiệu tổng quan về sản phẩm.
│   │   ├── Note.md                          # Là tài liệu chứa các ghi chú kỹ thuật, trade-off và nợ kỹ thuật (Tech Debt),...của team kĩ thuật.
│   │   ├── Spec-Client-IPC-And-UI-Bridge.md # UI/Native Domain: Đặc tả giao tiếp IPC và cầu nối FFI giữa UI và Core Rust.
│   │   ├── Spec-Core-Cryptography-And-Mesh.md # Crypto/Core Domain: Đặc tả E2EE, Zero-knowledge proof và mạng P2P Mesh.
│   │   ├── Spec-Dual-Sync-And-Local-Storage.md # Sync/Data Domain: Đặc tả cơ chế Offline-first, CRDT và mã hóa SQLite local.
│   │   ├── Spec-Ecosystem-And-Trust-Chain.md # Ecosystem Domain: Đặc tả vòng đời .tapp WASM Sandbox, Structured DataGrant và xác thực chữ ký Ed25519 từ Marketplace.
│   │   ├── Spec-Enterprise-Secure-Enclave.md # Infrastructure Domain: Cơ sở hạ tầng phần cứng, tích hợp HSM và RBAC doanh nghiệp.
│   │   ├── Spec-Identity-And-Governance.md  # Identity Domain: Quản trị định danh DID, SCIM 2.0 sync và phân quyền tuyệt đối qua OPA Policy Engine.
│   │   ├── Spec-Wasm-Tapp-Runtime.md        # WASM Runtime Domain: Môi trường Sandbox thực thi Mini-App (Tapp) an toàn.
│   │   ├── Tech_Debt.md                     # Là tài liệu chứa các ghi chú kỹ thuật, trade-off và nợ kỹ thuật (Tech Debt),...của team kĩ thuật.
│   │   └── TestMatrix.md                    # Ma trận kiểm thử bao phủ các ca kiểm thử (Edge cases & Failure states).
│   └── Directory-tree.md                    # Cấu trúc cây thư mục (file hiện tại).
├── lessons/                                 # Tài liệu đào tạo AI (AI tự ghi ,điều chỉnh và phát triển qua từng lịch sử tương tác).
├── phase/                                   # Tài liệu lộ trình dự án (Roadmap).
│   └── phase_1.md                           # Mục tiêu, phạm vi và thiết kế kỹ thuật cho Giai đoạn 1.
├── source/                                  # Source code hệ thống (The Single Source of Truth).
│   ├── bindings/                            # Lớp trung gian (Foreign Function Interface) giao tiếp đa ngôn ngữ.
│   │   ├── napi-harmony/                    # Cầu nối Rust - Node.js/C++ cho HarmonyOS.
│   │   ├── uniffi-android/                  # Cầu nối Rust - Kotlin/JNI cho Android.
│   │   ├── uniffi-apple/                    # Cầu nối Rust - Swift cho iOS/macOS.
│   │   └── wasm-bridge/                     # Cầu nối Rust - WebAssembly cho Web Client.
│   ├── clients/                             # Tầng trình diễn UI (Presentation Layer).
│   │   ├── android/                         # Native Android App (Kotlin/Jetpack Compose).
│   │   ├── apple/                           # Native Apple App (SwiftUI).
│   │   ├── desktop/                         # Desktop App (Tauri hoặc Electron).
│   │   ├── harmonyos/                       # Native HarmonyOS App (ArkUI).
│   │   └── web/                             # Web App Client (React/Vue + WebAssembly).
│   ├── core/                                # Trái tim của hệ thống (Rust Workspace) - Secure by Design.
│   │   ├── tc-crdt-sync/                    # Logic phân giải xung đột dữ liệu phi tập trung (CRDT).
│   │   ├── tc-crypto/                       # Lõi Hybrid PQ-KEM (Kyber768), quản lý HKMS và ép buộc ràng buộc ZeroizeOnDrop trên toàn RAM.
│   │   ├── tc-mesh/                         # Survival Mesh Networking: Tự tổ chức P2P qua BLE/Wi-Fi Direct và giao thức khẩn cấp EMDP.
│   │   ├── tc-store/                        # TeraVault VFS, quản lý hot_dag.db cho CRDT và SQLite FTS5 Zero-Knowledge search.
│   │   └── tc-tapp/                         # WASM Engine: Thực thi và cô lập các ứng dụng vi mô (Trusted Apps).
│   ├── infra/                               # Hạ tầng dưới dạng Code (IaC) & Triển khai.
│   │   ├── bare-metal/                      # Cấu hình máy chủ vật lý cho môi trường cực kỳ bảo mật (On-premise).
│   │   ├── edge-routing/                    # Cấu hình mạng phân phối biên (Edge Nodes).
│   │   ├── k8s-clusters/                    # Manifests triển khai trên Kubernetes cho Cloud/Enterprise.
│   │   ├── mac-mini/                        # Cấu hình máy chủ trên đơn Mac mini.
│   │   └── mac-mini-clusters/               # Cấu hình máy chủ trên cụm Mac mini.
│   └── integration_tests/                   # Các kịch bản test tích hợp kết nối giữa Core và Bindings.
├── tests/                                   # Hệ thống kiểm thử chất lượng và khả năng chịu lỗi.
│   ├── chaos-mesh/                          # Bơm lỗi giả lập (mất mạng đột ngột, node chết) để test tính bền bỉ.
│   ├── cross-platform-e2e/                  # Kiểm thử hành vi người dùng cuối (End-to-End) trên đa nền tảng.
│   └── ffi-stress/                          # Kiểm thử tải (Load Testing) và dò rỉ bộ nhớ (Memory Leak) qua cầu nối Rust FFI.
└── Tools/                                   # Các công cụ tiện ích (Utility scripts) dùng trong nội bộ.
    ├── content-synchronization.py           # Script đồng bộ hóa tài liệu giữa các môi trường.
    ├── directory-tree-sync.py               #
    ├── open-business.py                     # Tool mở tài liệu business.
    └── open-document.py                     # Tool mở tài liệu kĩ thuật.
