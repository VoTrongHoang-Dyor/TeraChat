Chào bạn. Dưới góc độ của một Senior Software Engineer theo trường phái **First Principles Thinking** (Tư duy từ những nguyên lý cơ bản) và tinh thần **Monozukuri** (Chế tác hoàn hảo), tôi từ chối việc "vá víu" (patching) từng lỗi một. Các bản vá rải rác chính là mầm mống của Nợ kỹ thuật (Tech Debt) thế hệ tiếp theo.

Để giải quyết 11 lỗ hổng (6 CRITICAL, 5 HIGH) mà chúng ta đã chốt lại, đồng thời **bảo toàn tuyệt đối 3 triết lý cốt lõi của TeraChat** (Zero-Knowledge, Offline-first, Hardware-bound keys), chúng ta phải tái cấu trúc hệ thống thông qua **5 Combo Giải pháp Kiến trúc (Architectural Combos)**.

Mỗi Combo dưới đây được thiết kế như một cơ chế **Poka-yoke (Chống sai lỗi)** ở tầng hệ thống, giải quyết gốc rễ vật lý và toán học của vấn đề.

---

### COMBO 1: "Aegis" Boundary & Continuous Masking

**Giải quyết:** TD-006 (FFI Panic Abort) & TD-007 (iOS Memory Compression).
**Nguyên lý cơ bản:** Bộ nhớ RAM không bao giờ an toàn nếu ranh giới giao tiếp (FFI) mỏng manh và hệ điều hành (OS) có quyền can thiệp tầng thấp (Paging/Compression).

* **Poka-yoke FFI (Chống hoảng loạn):** Khai tử việc truyền con trỏ trần (raw pointers) mang dữ liệu nhạy cảm qua C-ABI. Tại *mọi* ranh giới `extern "C"`, bọc toàn bộ bằng `std::panic::catch_unwind`. Thiết lập một `Panic Hook` toàn cục: Nếu Rust Core panic, tiến trình không được phép `abort` ngay. Nó phải gọi hàm `Secure_Arena_Wipe(0x00)` để ghi đè toàn bộ vùng nhớ Key trước khi trả về Exit Code cho Host (Flutter/Tauri).
* **Continuous XOR Masking (Biến hình liên tục):** Để chống lại việc iOS XNU Kernel nén RAM (gây lách luật `ZeroizeOnDrop`), Key Material không bao giờ được lưu dưới dạng Plaintext khi nghỉ (at rest) trong RAM.
  * Khởi tạo một luồng (thread) chạy ngầm đánh nhịp mỗi 50ms. Luồng này liên tục XOR Key Material với một Nonce ngẫu nhiên mới.
  * Nếu OS nén trang nhớ đó lại, nó chỉ nén được một chuỗi nhiễu (Ciphertext). Khi Lõi Rust cần dùng Key, nó XOR ngược lại, nạp vào thanh ghi CPU (Register), dùng xong trong `<1ms` và tiếp tục vòng lặp XOR Masking.

### COMBO 2: "Spectrum" QoS & Quantum Armor

**Giải quyết:** TD-008 (BLE QoS) & TD-009 (Quantum Harvest EMDP).
**Nguyên lý cơ bản:** Vật lý vô tuyến (BLE) quy định băng thông hẹp. Mật mã học quy định thời gian bẻ khóa (Lượng tử). Phải tôn trọng cả hai.

* **Bộ phân kênh vô tuyến (Mesh QoS Multiplexer):** Đặt luật cứng ở lớp Data-link của Mesh:
  * `Priority 0` (Control Plane: Epoch Ratchet, Revoke, KillDirective).
  * `Priority 1` (Chat Event).
  * `Priority 2` (Blob Chunks).
  * Lõi Rust theo dõi RTT (Round Trip Time) của sóng BLE. Nếu RTT > 200ms, tự động **Suspend (Đóng băng)** toàn bộ luồng tải File (Priority 2) ở mức Socket. Control Plane luôn có đường truyền thông thoáng tuyệt đối, chống Split-brain.
* **Áo giáp Lượng tử (Quantum Escrow):** Loại bỏ ECIES (Curve25519) thuần túy trong `EmdpKeyEscrow`. Nâng cấp lên **Hybrid ECIES + ML-KEM-768**. Vì ML-KEM có payload lớn (~1.18KB) vượt quá MTU của BLE, sử dụng thuật toán mã hóa xóa (Erasure Coding) **RaptorQ (RFC 6330)** để chia nhỏ gói Escrow thành 5 mảnh nhỏ phát qua BLE Beacon. Đảm bảo chống thu thập lượng tử (Store-Now-Decrypt-Later) mà không làm rớt gói tin.

### COMBO 3: "Chronos" Trust & "Tarpit" Protocol

**Giải quyết:** TD-010 (OS Time Spoofing) & TD-012 (Weaponized Wipe DoS).
**Nguyên lý cơ bản:** Hệ thống Zero-Trust không được phép tin tưởng thời gian do User/OS cung cấp, và Tính Sẵn sàng (Availability) không được phép bị phá hủy bởi các cuộc tấn công Brute-force rẻ tiền.

* **Đồng hồ tuyệt đối (Hardware Monotonic Tick):** Khai tử `SystemTime::now()` trong các logic kiểm tra hết hạn (TTL, Revoke) khi Offline.
  * Khi có mạng: Lấy "Anchor Time" từ máy chủ.
  * Khi mất mạng: Rust Core gắn bộ đếm thời gian vào **Monotonic Counter** của TPM 2.0 hoặc đếm chu kỳ nhịp CPU (Tick-Clock). Dù User lùi giờ HĐH về năm 2000, Rust Core vẫn biết chính xác đã trôi qua bao nhiêu giây thực tế.
* **Tarpit (Đầm lầy) & Duress PIN (Mã kề cổ):** Xóa bỏ luật "Sai 5 lần -> Xóa DB".
  * Thay bằng **Exponential Backoff Tarpit**: Sai lần 5 -> Tăng cost của thuật toán Argon2id lên mức cao nhất, khóa thiết bị 5 phút. Sai lần 10 -> Khóa 1 tiếng. Kẻ địch không thể Brute-force, nhưng dữ liệu vẫn an toàn (Đảm bảo Availability).
  * Thêm tính năng **Duress PIN**: Người dùng cài một mã PIN thứ hai (VD: PIN thật là 1234, Duress PIN là 9999). Nếu bị chĩa súng bắt mở máy, nhập 9999. Máy vẫn mở vào một giao diện "Trống rỗng", nhưng ngầm kích hoạt lệnh `Zeroize` xóa vĩnh viễn Master Key ở nền.

### COMBO 4: "Fortress" Data Plane

**Giải quyết:** TD-011 (Localhost Proxy), TD-013 (CAS Side-Channel), TD-015 (Hash Collision).
**Nguyên lý cơ bản:** Đường truyền dữ liệu cục bộ phải được xác thực danh tính tiến trình (Process Identity), và hàm băm (Hash) không được phép rò rỉ ngữ cảnh hoặc đụng độ.

* **Poka-yoke Local Proxy:** Đóng cửa hoàn toàn `127.0.0.1`.
  * Desktop/Laptop: Dùng **Unix Domain Sockets (UDS)** trên macOS/Linux và **Named Pipes** trên Windows. Rust Core xác thực chính xác PID (Process ID) của Tauri UI trước khi truyền byte. Malware port-scan sẽ bị mù hoàn toàn.
  * Mobile: Dùng **One-Time Streaming Token (OTST)** dài 256-bit sinh ra ngẫu nhiên cho mỗi lần chạm xem file. URL sẽ có dạng nội bộ với Token. Sai Token -> Chặn IP nội bộ 5 phút.
* **Băm kép & Rắc muối (Dual-Salted Hash):** Tái cấu trúc hàm tính định danh file trên NAS.
  * Chống đụng độ (TD-015): `cas_hash = BLAKE3(chunk) || SHA-512(chunk)[0:32]`.
  * Chống suy luận CAS Deduplication (TD-013): Thêm Salt: `BLAKE3(Workspace_ID || Tapp_Context || Ciphertext)`. Việc Deduplication giờ đây chỉ diễn ra trong phạm vi một phòng ban (Workspace), kẻ tấn công từ phòng ban khác không thể dò tìm file mật.

### COMBO 5: "Semantic Shield" & Hermetic Forge

**Giải quyết:** TD-014 (AI Prompt Injection) & XPLAT-09 (Build Non-determinism).
**Nguyên lý cơ bản:** Logic tĩnh (Regex) không thể chặn được ngôn ngữ tự nhiên. Mã nhị phân phải được chứng minh nguồn gốc bằng toán học, không phụ thuộc vào hệ điều hành biên dịch.

* **Tường lửa ngữ nghĩa (Semantic DLP Firewall):** OPA Regex là không đủ với WASM có quyền truy cập LLM. Triển khai một LLM Quantized cỡ nhỏ (chạy cục bộ trên TeraRelay Node) làm "Người kiểm duyệt". Mọi dữ liệu JSON sinh ra từ WASM `Egress_Outbox` muốn ra internet phải qua LLM này chấm điểm (LLM-as-a-Judge) để phát hiện xem có chứa thông tin nhạy cảm bị ẩn giấu dưới dạng phép ẩn dụ (Prompt Injection Exfiltration) hay không.
* **Lò rèn Tất định (Nix-based Hermetic Build):** Xóa bỏ các CI/CD runner dùng OS mặc định.
  * Sử dụng **Nix Flakes** để đóng băng môi trường (Bit-for-bit reproducible builds). Glibc, MSVC, Clang, Rustc được khóa cứng mã Hash. Bất kỳ kỹ sư nào tải mã nguồn về compile cũng ra được file `.apk` hay `.exe` có mã Hash giống hệt CI.
  * Triển khai **Hardware-in-the-loop Signing**: CI Runner nội bộ cắm YubiKey HSM. Mã nhị phân build xong phải được ký (EV Code Signing) tự động bởi Hardware Key, loại bỏ hoàn toàn Human-error và Supply Chain Attacks (Chuẩn ISO 27001 A.8.4).

---

### KẾT LUẬN (VERDICT)

Việc áp dụng 5 Combo Kiến trúc này không phải là "thêm tính năng", mà là **củng cố vật lý và toán học của hệ thống**.

Với các combo này, TeraChat sẽ không bị bẻ cong bởi sự thất thường của Hệ điều hành (OOM Kill, Paging, Clock Drift), sự giới hạn của phần cứng (BLE Bandwidth), hay sự tiến hóa của vũ khí tấn công (Quantum, AI).

Hệ thống đã đạt trạng thái **"Secure by First Principles"** (Bảo mật từ nguyên lý gốc), sẵn sàng cho quy trình Pen-test cấp độ quân sự và kiểm toán FIPS 140-3.
Chào bạn, với tư cách là Tech Lead / Security Architect của dự án, tôi đánh giá cao việc chuyển dịch từ hạ tầng Cloud truyền thống sang mô hình **"On-Premises Enterprise Appliance"** (Mac Studio + NAS + RTX Node). Hướng đi này giải quyết triệt để bài toán *Data Sovereignty* (chủ quyền dữ liệu) - một yêu cầu tối thượng của khách hàng khối chính phủ và quốc phòng, đồng thời tạo môi trường R&D khép kín rất tốt.

Trả lời trực tiếp vào câu hỏi của bạn về việc chọn NAS: **Tôi gạch bỏ Synology và đề xuất bắt buộc phải sử dụng TrueNAS (phiên bản Scale dựa trên Linux/ZFS) cho kiến trúc của TeraChat.**

Dưới đây là góc nhìn kiến trúc, các trade-off và cách chúng ta sẽ map hạ tầng này vào mã nguồn hiện tại.

### 1. Tại sao là TrueNAS thay vì Synology? (Góc độ Kiến trúc & Bảo mật)

Synology mang lại sự ổn định "cắm là chạy", nhưng nó là một hộp đen (black-box) với hệ điều hành đóng và cơ chế ảo hóa bị giới hạn. Đối với một hệ thống Enterprise-grade như TeraChat, NAS không chỉ là nơi "chứa file", nó là một **Persistent State Node** trong mạng lưới phân tán.

* **Lợi thế của ZFS (TrueNAS):** ZFS hỗ trợ tính năng *Immutable Snapshots* (Bản ghi trạng thái không thể thay đổi) ở cấp độ block. Khi kết hợp với kiến trúc dữ liệu CRDT của TeraChat (tại `Spec_Dual_Sync_And_Local_Storage.md`), nếu một node bị lỗi logic mạng hoặc dính ransomware, chúng ta có thể rollback trạng thái của toàn bộ Vector DB và Mesh state về vài giây trước đó mà không làm hỏng tính toàn vẹn của CRDT.
* **OS-level Control & Zero-Trust:** TrueNAS Scale cho phép chúng ta chạy trực tiếp các container (Docker/Kubernetes) bằng đặc quyền root có kiểm soát. Chúng ta có thể nhúng thẳng các module Rust (như relay node của libp2p) và Vector DB (Chroma/Milvus) lên NAS, đồng thời cấu hình WireGuard/Tailscale ở tầng kernel để đưa NAS vào mạng Mesh ẩn (Darknet) của TeraChat. Synology sẽ tạo ra bottleneck lớn về overhead nếu xử lý các tác vụ này.

### 2. Tích hợp Máy chủ AI (RTX Node) và Rủi ro Thất bại (Failure Cases)

Đưa thêm RTX 4090 vào Giai đoạn 3 là một bước đi tất yếu để chạy các tác vụ RAG quy mô lớn. Lúc này, Mac Studio M2 Max sẽ chuyển dịch vai trò từ "cỗ máy chạy mọi thứ" sang **API Gateway & Load Balancer**, viết bằng Rust.

Tuy nhiên, giao tiếp giữa Mac Studio và RTX Node qua mạng LAN nội bộ sẽ sinh ra độ trễ (latency) và các rủi ro sập nguồn/OOM (Out of Memory) trên GPU. Nguyên tắc thiết kế của chúng ta là không bao giờ để toàn bộ dịch vụ AI chết chỉ vì node RTX gặp sự cố.

**Giải pháp: Graceful Degradation (Suy thoái có kiểm soát)**
Chúng ta phải thiết kế một Fallback Routing trong mã nguồn Rust. Nếu truy vấn gRPC/mTLS từ Mac sang RTX bị timeout hoặc trả về lỗi cạn kiệt tài nguyên, Mac Studio sẽ tự động "hứng" lại request đó và chạy bằng sức mạnh MLX nội bộ (Unified Memory 64GB) ở tốc độ chậm hơn, nhưng đảm bảo tính liên tục của dịch vụ.

*Pseudo-code minh họa luồng xử lý trên Mac Studio:*

```rust
async fn route_llm_inference(req: LlmRequest) -> Result<Response, TeraError> {
    // Thử gọi sang RTX Node (High Performance)
    match rtx_node_client.infer(req.clone()).await {
        Ok(resp) => Ok(resp),
        Err(e) if e.is_network_timeout() || e.is_gpu_oom() => {
            log::warn!("RTX Node failed ({}). Failing over to Mac Studio MLX fallback...", e);
            
            // Chuyển hướng tác vụ về Unified Memory của Mac (chậm hơn nhưng an toàn)
            local_mlx_engine::infer_with_quantized_model(req).await
        }
        Err(fatal_err) => Err(fatal_err),
    }
}
```

### 3. Cập nhật Tài liệu Kỹ thuật (Domain Mapping)

Việc áp dụng hạ tầng này đòi hỏi chúng ta phải điều chỉnh lại các bản đặc tả hiện tại:

* **`Spec_Enterprise_Secure_Enclave.md`**: Cần bổ sung ngay topology "Local Appliance Model". Xác định rõ Mac Studio là *Control Plane* và NAS + RTX là *Data/Compute Plane*.
* **`Spec_Core_Cryptography_And_Mesh.md`**: Phải định nghĩa cơ chế xác thực nội bộ. Dù 3 thiết bị này nằm chung một phòng, chúng ta vẫn phải mã hóa kết nối giữa chúng theo chuẩn Zero-Trust.
* **`Spec_Wasm_Tapp_Runtime.md`**: Cấu hình để WASM Runtime trên máy khách biết cách đẩy các logic nặng về Mac/RTX thay vì cố gắng xử lý cục bộ gây nóng thiết bị.

---
