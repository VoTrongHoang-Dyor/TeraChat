# **TERACHAT — SOVEREIGN WORK OS**

**Phiên bản:** Kiến trúc Tối ưu hóa Phần cứng Cục bộ & Khả năng Tương thích Doanh nghiệp lớn (Tái cấu trúc: CAS Deduplication, MemFS I/O Isolation, Native Metal AI & Thin UI/Fat Core)  
**Góc nhìn:** Principal Engineer / System Architect  
**Technical SEO Keywords:** Zero-Knowledge Architecture, Content Addressable Storage (CAS), Post-Quantum Cryptography (PQC), OpenMLS RFC 9420, WebAssembly (WASM) Sandbox, CRDT Sync, gRPC over FFI IPC, Secure Enclave TPM 2.0, Trusted Execution Environment (TEE), Confidential Computing, AMD SEV-SNP, Intel TDX, gRPC over FFI, Arena Allocation, CQRS Event Sourcing, Delegated Proposer, Shadow Graph, Null-Origin Iframe, Declarative Proxy, MemFS, RAM-Disk, flutter\_rust\_bridge, Panic Boundary, Sensor-based Duty Cycling, Hardware-Accelerated Crypto.

## **1\. BỨC TRANH TOÀN CẢNH & HIỆU QUẢ KINH TẾ (UNIT ECONOMICS & ROI)**

TeraChat không phải là một ứng dụng chat thông thường, nó được định vị là một **"Sovereign Work OS" (Hệ điều hành Công việc có Chủ quyền)**. Mục tiêu cốt lõi là thay thế hoàn toàn sự phụ thuộc của doanh nghiệp vào các nền tảng SaaS (Slack, Microsoft Teams) thông qua việc trao trả 100% quyền kiểm soát hạ tầng và dữ liệu về tay khách hàng.

* **1.1. Từ bỏ mô hình Chat SaaS truyền thống:** Khách hàng sở hữu hoàn toàn hạ tầng lưu trữ. Server chỉ xử lý ciphertext, bảo vệ tuyệt đối sự riêng tư doanh nghiệp.  
* **1.2. Quyết định giới hạn phạm vi (Scope Bound):** ❌ **Đã loại bỏ hoàn toàn Customer-facing messaging.** Không cố gắng ép đối tác bên ngoài chuyển nền tảng. Focus 100% vào **E2EE Internal Workspace**, triệt tiêu hoàn toàn vector tấn công từ bên ngoài và đơn giản hóa quản lý định danh.  
* **1.3. Mô hình Kinh doanh & Chiến lược MVP:** Thuần túy B2B Enterprise-only, License-Gated. **Lưu ý chiến lược:** Phase 1 (MVP) dồn 100% nguồn lực vào lõi E2EE Chat, Sync CRDT, và tối ưu hóa hạ tầng Edge cục bộ (Mac Mini \+ NAS) để nhanh chóng phát hành sản phẩm sinh doanh thu (Revenue-generating product). Toàn bộ hệ sinh thái mở rộng .tapp (WASM), Virtual File System và các module tích hợp phức tạp được dời sang Phase 2\.  
* **1.4. Bài toán Kinh tế (GTM Killer):** Doanh nghiệp mua On-premise để cắt giảm OpEx. Nếu bắt họ build một cụm Kubernetes có Confidential Computing trên AWS/Azure, chi phí duy trì hạ tầng Cloud tự vận hành sẽ triệt tiêu lợi ích tài chính.  
  * *Hạ tầng du kích (Guerrilla Infrastructure):* TeraChat tập trung cải tiến tối ưu hóa phần mềm để cụm **Mac Mini M-series \+ NAS ($1700 CapEx mua đứt)** có thể gánh tải cho 10,000+ nhân sự với ROI hoàn vốn chỉ sau 10 ngày vận hành đầu tiên (So với hóa đơn $50,000/tháng của Slack/Teams cho 10,000 users).

## **2\. KIẾN TRÚC MẬT MÃ ZERO-KNOWLEDGE & QUẢN TRỊ TOÀN VẸN DỮ LIỆU**

### **2.1. Mô hình Dumb Relay & Sealed Sender**

Nguyên tắc bất di bất dịch: Server chỉ đóng vai trò là "Trạm trung chuyển mù" (Dumb Relay) và lưu trữ Ciphertext. Mọi metadata giao tiếp đều được che giấu bằng kỹ thuật Sealed Sender để chống phân tích lưu lượng (Traffic Analysis Mitigation).

### **2.2. Trust Kernel: Hardware-Backed Security (Secure Enclave & TPM 2.0)**

Hệ thống áp dụng nguyên tắc **Zero-Trust Key Management**. Các khóa bí mật tuyệt đối không lưu trữ trên ổ đĩa thông thường mà được sinh ra và neo cứng trực tiếp vào phần cứng vật lý thông qua Trust Kernel của thiết bị (Apple Secure Enclave via Keychain API, TPM 2.0 trên Windows/Linux, và Android StrongBox/Keystore).

### **2.3. Compliance Escrow Key: Cơ chế Ủy thác Tuân thủ cho Khối Gov/Finance**

Đối với khối Gov/Finance, yêu cầu thanh tra pháp lý (e-Discovery) được giải quyết thông qua cơ chế **Compliance Escrow Key**. Khóa này được phân rã thành đa chữ ký thông qua lược đồ chia sẻ bí mật Shamir. Chỉ khi có sự phê duyệt đồng thuận (Quorum) của Ban Quản trị, khóa mới được phục dựng bên trong môi trường TEE bảo mật để giải mã xuất dữ liệu tuân thủ, triệt tiêu hoàn toàn rủi ro từ Rogue Admin.

### **2.4. Phân mảnh Cấu trúc Dữ liệu: CQRS & Append-Only Event Log**

Để vô hiệu hóa "quả bom" phình to dữ liệu (State Bloat) do lạm dụng CRDT cho mọi tính năng, TeraChat phân mảnh cấu trúc dữ liệu theo nguyên tắc CQRS (Command Query Responsibility Segregation):

* **Dữ liệu Tuyến tính (Chat Messaging):** Tuyệt đối không dùng CRDT. Tin nhắn bản chất là chuỗi sự kiện tuyến tính, được xử lý bằng **Append-Only Event Log** kết hợp với **Vector Clocks**. Tin nhắn chỉ được ghi chèn vào cuối, không làm xáo trộn đồ thị. Các thao tác Sửa/Xóa (Edit/Delete) chỉ là các sự kiện ghi đè (override events) ở lớp hiển thị, giúp giảm 90% dung lượng lưu trữ cục bộ.  
* **Dữ liệu Trạng thái (Collaborative Notes, Thread Titles):** Giới hạn CRDT khắt khe chỉ ở những vùng dữ liệu có tính cập nhật đồng thời cao này. Quá trình dọn rác (Tombstone Garbage Collection) được thực thi cực kỳ nhẹ nhàng trên các snapshot tĩnh của Event Log.

### **2.5. Thuần hóa AI Agent với Mô hình Shadow Graph (Đồ thị Bóng)**

Tuyệt đối không cấp quyền ghi (Write/Mutate) trực tiếp lên đồ thị dữ liệu gốc (Root DAG) cho bất kỳ mô hình AI nào (dù Confidence Score đạt 99.9%), tránh rủi ro "ảo giác" (hallucination) phá hỏng vĩnh viễn cấu trúc dữ liệu Zero-Knowledge.

* **Cơ chế Shadow Graph:** Khi xảy ra xung đột đồng bộ ngoại tuyến, Rust Core bảo toàn cả hai nhánh dữ liệu gốc (Branch A và B). Local AI Agent chỉ được phép đọc và tạo ra một **Shadow Branch (Nhánh Bóng)** chứa phương án giải quyết xung đột đề xuất.  
* **Con người kiểm duyệt (Human-in-the-loop):** Hệ thống hiển thị đề xuất của AI trên UI. Chỉ khi người dùng bấm "Chấp nhận", Rust Core mới ký số (Cryptographic Signature) để chính thức commit nhánh Shadow đó thành trạng thái chính thức của hệ thống. Nếu AI crash hoặc hallucinate, hệ thống tự động fallback về luồng xử lý xung đột thủ công truyền thống (như Git merge conflict).

### **2.6. Ranh giới Cô lập Tuyệt đối (Decoupling Core & AI)**

* **Cấm AI can thiệp mã nguồn:** Module mật mã tc-crypto được bảo vệ bằng CODEOWNERS lock trong Git. Các công cụ AI không có quyền can thiệp.  
* **Giao tiếp gián tiếp qua Host ABI:** Các tiến trình AI không bao giờ được chạm vào vùng nhớ chứa Key Material. Mọi thao tác đi qua Host ABI (Serialize bằng MessagePack).

## **3\. GIAO THỨC SINH TỒN NGOẠI TUYẾN TERALINK & ĐỒNG BỘ HIỆU NĂNG CAO**

### **3.1. Các Giới Hạn Cứng Trong Mạng T3 BLE Emergency**

"Phần mềm không thể sửa được lỗi của vật lý". Việc ép sóng BLE cõng payload quá lớn hoặc thả nổi số lượng thiết bị phát sóng sẽ tạo ra bão vô tuyến vô hiệu hóa toàn mạng lưới. Hệ thống áp dụng các giới hạn vật lý và logic cực đoan sau:

* **Kiến trúc Truyền miệng Mù có Kiểm soát (Controlled Blind Gossip Mesh):** Để hỗ trợ mạng khẩn cấp quy mô tập đoàn lên tới 10,000 người di động tự do, TeraChat áp dụng giao thức Gossip hiệu chuẩn từ BitChat với 3 lá chắn chống bão mạng cứng:  
  1. **Khử trùng lặp tầng mạng (GCS Deduplication):** Mỗi gói tin mang một ID duy nhất. Khi nhận gói tin, Client phát lại đúng 1 lần, sau đó ghi log ID vào bộ lọc Golomb Coded Set (GCS) Filter cực kỳ tiết kiệm bộ nhớ. Nếu nhận lại gói tin trùng lặp từ hướng khác, hệ thống lập tức loại bỏ (drop) ngay tại driver của tc-mesh, triệt tiêu bão mạng ngay lập tức.  
  2. **Thời gian sống giới hạn (Hop Limit TTL):** Thiết lập giới hạn bước nhảy cứng (mặc định TTL \= 7). Khi TTL \= 0, gói tin tự động bị tiêu hủy, ngăn tin nhắn chạy vòng lặp vô tận trong tòa nhà/nhà máy.  
  3. **Định tuyến tại Nguồn (Source Routing):** Khi Client A đã cache được lộ trình tối ưu đến Client C (A \-\> B \-\> C), lộ trình này được mã hóa cứng vào Header. Client B nhận được sẽ chỉ thụ động chuyển tiếp (Dumb Relay) cho C mà không phát tán (broadcast) vô nghĩa ra xung quanh.  
* **Bảng Điểm Ưu Tiên Mạng Mesh Động (Dynamic Mesh Score \- DMS):** Định kỳ mỗi 60 giây, tầng tc-mesh chạy ngầm luồng đánh giá DMS để tự xếp hạng thiết bị vào đồ thị định tuyến:

| Hạng (Tier) | Loại thiết bị | Điều kiện Môi trường / Phần cứng | Vai trò trong mạng Mesh |
| :---- | :---- | :---- | :---- |
| **Tier 1** | Desktop / Laptop (Windows, macOS, Linux) | Luôn cắm sạc (Nguồn AC điện lưới), RAM dồi dào, CPU đa nhân. | **Super Relay (Lõi):** Cho phép Store-and-Forward full bandwidth, không giới hạn hàng đợi, gánh tải truyền nhận cho toàn bộ khu vực xung quanh. |
| **Tier 2** | Laptop chạy Pin | Pin \> 50%, đang kết nối Wi-Fi LAN hoặc Wi-Fi Direct. | **Active Relay:** Tiếp sóng tin nhắn thông thường và lệnh hệ thống. Tự động hạ cấp xuống Leaf Node khi pin tụt \< 40%. |
| **Tier 3** | Thiết bị Android | Pin \> 50%, RAM trống \> 50% (Tiến trình nền OS không quá tải). | **Edge Relay (Biên):** Hỗ trợ truyền thông Gossip trong phạm vi cục bộ (Local Cluster), chỉ nhận và forward các gói tin có kích thước tối ưu. |
| **Tier 4** | iPhone (iOS) | Khoảng cách gần (RSSI mạnh), Sóng Bluetooth ổn định, App đang mở hoặc vào background \< 3 phút. | **Passive Relay / Leaf Node:** Chỉ tham gia tiếp sóng khi khoảng cách cực gần và không tìm thấy bất kỳ Tier 1, 2, 3 nào xung quanh. |

* **Giải pháp kỹ thuật sống sót nền cho iPhone (iOS Background Survival):** Cơ chế CoreBluetooth của iOS sẽ kill tiến trình (via Jetsam) nếu ứng dụng chạy ngầm ngốn RAM quá nhanh hoặc giữ Wakelock quá lâu. Khi iPhone chuyển sang Background ở chế độ T3, Rust Core kích hoạt trạng thái **"Micro-Daemon Mode"**: Giao diện Flutter/Tauri bị đóng băng và giải phóng hoàn toàn khỏi RAM (Zero-UI footprint). Chỉ duy trì duy nhất tiến trình headless tc-mesh bằng Rust Core chạy ngầm với dung lượng RAM cố định **dưới 15MB** (an toàn tuyệt đối dưới ngưỡng cảnh báo 50MB của iOS), cho phép thiết bị iOS làm trạm trung chuyển chặng ngắn (Short-hop Relay) cực kỳ dẻo dai.  
* **Priority Packet Queue & Progressive Loading (Hàng đợi Ưu tiên thông minh):** Thay vì cắt bỏ cứng tin nhắn một cách thô bạo gây ức chế trải nghiệm người dùng, TeraChat chuyển sang cơ chế hàng đợi ưu tiên: Gửi đi nguyên vẹn 50 từ đầu tiên dưới dạng **High Priority** (chiếm vừa vặn 1 frame BLE để hiển thị lập tức). Phần văn bản còn lại tự động bị băm thành các chunk **Low Priority (Background Gossip)**. Ứng dụng áp dụng cơ chế *Progressive Loading*, tự động lấp đầy phần văn bản còn lại một cách mượt mà khi thiết bị lướt qua (handshake) với các Node trung gian khác.  
* **Quy tắc election\_weight \= 0 (iOS):** Do iOS áp đặt cơ chế tự đình chỉ cổng BLE GATT khi khóa màn hình, thiết bị iOS không bao giờ được bầu làm Floor Gateway. Trọng số bầu cử của iOS bị hardcode bằng 0\.  
* **Chặn Tuyệt đối Truyền File (Aggressive Throttling):** Vô hiệu hóa hoàn toàn API gửi File/Media. Mọi luồng dữ liệu ngoại trừ Plaintext (tin nhắn khẩn, tọa độ GPS, form đã nén và Lệnh thu hồi khóa OpenMLS) đều bị MeshMultiplexer chặn (drop) ngay lập tức.

### **3.2. Tiết kiệm Pin & Chống Quá tải Thiết bị Di động (Mobile Battery Overload)**

Điện thoại thông minh của doanh nghiệp không phải là server cắm điện. Để tránh biến thiết bị của nhân viên thành "lò sưởi vật lý" dẫn đến việc gỡ app hàng loạt, TeraChat áp dụng các cơ chế Heuristics và điều khiển phần cứng thông minh:

* **Mạng lưới thích ứng theo cảm biến (Sensor-based Duty Cycling):** Giữ chip BLE quét liên tục 100% thời gian là tự sát về pin. TeraChat kết hợp trực tiếp dữ liệu từ gia tốc kế (Accelerometer) vào logic mạng Mesh. Nếu thiết bị nằm yên trên bàn 5 phút, tần số quét BLE Mesh sẽ tự động hạ từ 1 giây/lần (Active mode) xuống 10 giây/lần (Sleep mode). Ngay khi phát hiện rung động vật lý (người dùng nhấc máy lên), hệ thống ngay lập tức đánh thức (Wake up) vòng lặp Mesh trở lại thời gian thực.  
* **Cơ chế trì hoãn tính toán thông minh (Smart Deferral Heuristics):** Ngừng băm mã hóa (BLAKE3) hoặc nén (Zstd) các file dung lượng lớn ngay lập tức nếu điều kiện hệ thống không thuận lợi. Hệ thống liên tục giám sát trạng thái pin và nguồn sạc: Nếu pin điện thoại \< 30% và không cắm sạc, các tác vụ tính toán nặng nề sẽ bị đẩy vào hàng đợi (Queue), hiển thị trạng thái "Đang chờ nguồn điện thích hợp để mã hóa..." trên UI.  
* **MIME-Type Exception Rule (Chỉ nén văn bản/JSON):** Thuật toán nén Zstandard cực kỳ hiệu quả cho dữ liệu dạng chuỗi văn bản hoặc JSON, nhưng nén các file Media (MP4, JPEG, PDF) là một sự lãng phí tài nguyên CPU vô nghĩa vì bản thân chúng đã được nén trước đó ở tầng ứng dụng. TeraChat hardcode quy tắc bỏ qua Zstd hoàn toàn đối với các định dạng Media đã được nén sẵn để bảo toàn pin.  
* **Đồng bộ hóa theo Điều kiện (Condition-based Sync):** Chấp nhận hy sinh thời gian thực khi pin yếu, ưu tiên tuyệt đối sự sống còn của thiết bị và trải nghiệm người dùng trong môi trường doanh nghiệp khắt khe.

### **3.3. Thiết Kế Bộ Nhớ Đệm Thích Ứng Cho Kỹ Thuật Store-and-Forward (Adaptive Ring Buffer)**

BitChat sử dụng một bộ nhớ đệm cố định là 500 gói tin cho mọi thiết bị. Thiết kế này gây nghẽn cổ chai trên máy tính (vốn thừa tài nguyên) nhưng lại dễ làm tràn RAM điện thoại. TeraChat triển khai kiến trúc **Adaptive Ring Buffer (Bộ nhớ đệm thích ứng theo tài nguyên)**:  
\[Mạng Mesh BLE/LAN\]  
        │  
        ▼  
┌─────────────────────────────────────────────────────────────┐  
│                  MeshMultiplexer (tc-mesh)                  │  
└─────────────────────────────────────────────────────────────┘  
        │  
        ├─► \[Tier 1: Desktop\] ──► Ring Buffer: 10,000 Msgs (RAM) ──► Full FTS5 Index  
        ├─► \[Tier 3: Android\] ──► Ring Buffer: 2,000 Msgs (RAM)  ──► Clean after TTL  
        └─► \[Tier 4: iPhone\]  ──► Ring Buffer: 300 Msgs (RAM)    ──► Evict Aggressively

* **1\. Cấu hình Kích thước Đệm động (Buffer Capacity):**  
  * **Tier 1 (Desktop):** Dung lượng đệm đạt **10,000 gói tin**. Desktop lưu trữ lượng lớn lịch sử chưa đồng bộ của các thiết bị xung quanh (gửi hộ/nhận hộ) để chờ xả tải về Server khi kết nối LAN được thiết lập lại.  
  * **Tier 3 (Android):** Dung lượng đệm đạt **2,000 gói tin**. Tận dụng năng lực quản lý tiến trình nền mở của Android để duy trì mạch Gossip lâu hơn trong phòng ban.  
  * **Tier 4 (iPhone):** Hạ dung lượng đệm xuống còn **300 gói tin**. Đệm xoay vòng và giải phóng cực nhanh (Aggressive Eviction) để bảo vệ RAM cho iOS dưới 15MB.  
* **2\. Cơ chế Khử trùng (Deduplication):** Khi nhận một gói tin, tc-mesh kiểm tra packet\_id bằng GCS Filter trong RAM. Nếu phát hiện đã tồn tại, gói tin bị hủy (drop) ngay lập tức để chặn bão mạng.  
* **3\. Cơ chế Xả đệm Thông minh (Smart Flushing):** Mỗi gói tin trong bộ nhớ đệm được cấu hình một thời gian sống nghiêm ngặt (Strict TTL) để dọn dẹp RAM liên tục, ngăn chặn tình trạng tràn bộ đệm (Buffer Overflow).

### **3.4. Kiến trúc Đồng bộ Hai Lớp & Delegated Proposer cho TreeKEM**

Đồng bộ được phân tách thành Control Plane (MLS) và Data Plane (CRDT/Event Log). Với các nhóm chat 5,000+ users, việc xoay khóa MLS TreeKEM liên tục sẽ vắt kiệt pin điện thoại di động. Việc dồn toàn bộ tính toán TreeKEM nặng nề vào TEE ở server cũng gây quá tải hàng đợi tính toán của Enclave.

* **Giải pháp Delegated Proposer (Người đề xuất ủy quyền):** Khi một thiết bị di động cần cập nhật khóa (Leaf Update), thay vì đẩy thẳng lên Server TEE để tính toán, thiết bị sẽ ủy quyền tác vụ này cho một **Fat Client** (như ứng dụng Desktop/Laptop của đồng nghiệp đang online trong cùng group). Fat Client sẽ chạy thuật toán tính toán cây khóa TreeKEM mới và gửi bản đề xuất (Proposal) lên Server.  
* **TEE đóng vai trò Sequencer & Verifier:** Máy chủ TEE lúc này được giải phóng tài nguyên tính toán đắt đỏ, nó chỉ mất vài mili-giây để xác minh (Verify) chữ ký mật mã của bản đề xuất và sắp xếp thứ tự (Sequence) để broadcast xuống các thiết bị khác.  
* **Bảo mật TEE (Trusted Execution Environment):** Quá trình xác minh và ký duyệt này bắt buộc phải diễn ra bên trong môi trường cách ly phần cứng TEE của máy chủ (Intel TDX, AMD SEV-SNP hoặc Apple Silicon Secure Enclave trên Mac Mini) để bảo vệ Session Keys khỏi bị rò rỉ kể cả khi SysAdmin có quyền root.  
* **Mesh-Freeze (Đóng băng xoay khóa ngoại tuyến):** Ở Tầng T3 BLE Emergency, OpenMLS bị đóng băng tạm thời, chỉ dùng **Static Session Keys** để bảo toàn 100% băng thông vô tuyến cho nhắn tin khẩn cấp.

### **3.5. GATT Streaming Reassembly & Tiết kiệm Băng thông**

Băng thông BLE hiệu dụng chỉ đạt \~0.5 Mbps. Hệ thống áp dụng các quyết định kiến trúc sinh tử để chống Broadcast Storm và tấn công tràn bộ nhớ (OOM):

* **Loại bỏ Mật mã Lượng tử khi Offline:** Mã hóa lượng tử Kyber768 chiếm \~1100 bytes ciphertext, gây nghẽn mạng nghiêm trọng nếu đẩy qua BLE. Khi rớt mạng, hệ thống **tự động ngắt PQ-KEM**, chỉ duy trì mã hóa E2EE tiêu chuẩn Curve25519/X25519 để bảo toàn băng thông.  
* **MTU Negotiation Động & Backpressure Queue:** Bỏ thiết kế hardcode (như fix cứng 3 frames) vốn rất dễ gây lỗi tràn bộ đệm (nhất là trên Android với chipset yếu). Tầng tc-mesh triển khai MTU Negotiation tự động điều chỉnh chunk size theo MTU thực tế của thiết bị đích. Đồng thời, áp dụng một hàng đợi Backpressure (pendingPeripheralWrites) để điều tiết tốc độ nhả gói tin ra CoreBluetooth.  
* **Chống OOM Attack bằng Strict TTL Reassembly:** Đối với các gói tin phân mảnh bắt buộc, nút nhận sẽ tạm lưu trên RAM mask. Tuy nhiên, để chống lại kiểu tấn công OOM (bắn spam Fragment 0 liên tục làm tràn RAM), TeraChat áp dụng cơ chế Strict TTL cực ngắn (ví dụ: 250ms). Nếu không nhận đủ các mảnh còn lại trong thời gian này, vùng RAM đó sẽ lập tức bị tiêu hủy (zeroize), không nương tay với dữ liệu rác.

### **3.6. Offload sang Crypto Hardware (Tận dụng phần cứng thiết bị di động)**

Dù lõi giao thức mật mã cấp cao sử dụng Post-Quantum Cryptography và MLS, đối với các tác vụ mật mã đối xứng cơ bản (như mã hóa/giải mã các File Chunk hoặc Blobtype bằng AES-GCM-256), hệ thống bắt buộc phải bỏ qua việc tự chạy thuật toán phần mềm bằng CPU điện thoại:

* **Hardware Acceleration:** Rust Core sử dụng các bindings đặc quyền gọi trực tiếp vào API phần cứng gốc của hệ điều hành (**iOS CryptoKit** thông qua Swift FFI và **Android Keystore/NSS** thông qua JNI).  
* **Kết quả:** Các chip ARM hiện đại (A-series của Apple hoặc Snapdragon của Qualcomm) có tích hợp sẵn các tập lệnh tối ưu cho mật mã học ở cấp độ phần cứng. Việc gọi trực tiếp các khối xử lý này giúp giảm tới 85% tải năng lượng lên CPU, giữ cho thiết bị di động hoàn toàn mát mẻ ngay cả khi truyền tải dữ liệu dung lượng lớn cục bộ.

## **4\. THIẾT KẾ HẠ TẦNG TỐI ƯU CỰC HẠN (SME APPLIANCE VS CLOUD-NATIVE)**

Để giải quyết triệt để bài toán kinh tế và khả năng tiếp cận của 90% doanh nghiệp, TeraChat định vị hạ tầng của mình dựa trên triết lý **"Software-Driven Hardware Efficiency" (Tối ưu hóa phần cứng bằng phần mềm cấp thấp)**. Cụm Mac Mini \+ NAS là kiến trúc deploy cốt lõi được tối ưu hóa liên tục, trong khi Kubernetes/Confidential Computing là tùy chọn bổ sung cho phân khúc siêu lớn.

### **4.1. SME & Enterprise Appliance (Mac Mini M-series \+ NAS \- Kiến trúc Trung tâm)**

Đây là tầng hạ tầng chủ lực của TeraChat. Bằng cách can thiệp sâu vào luồng xử lý và I/O của hệ điều hành, hệ thống cho phép một cụm phần cứng giá rẻ $1700 gánh tải vượt trội hơn các server SaaS truyền thống:

* **Mã hóa Trùng lặp từ Nguồn (CAS \- Content Addressable Storage):** Giải quyết triệt để nguy cơ sập ổ đĩa 2TB của NAS khi phục vụ hàng ngàn người dùng liên tục chia sẻ tài liệu chung:  
  1. *BLAKE3 Fingerprint:* Trước khi mã hóa E2EE, Client tự động băm (hash) file gốc bằng thuật toán siêu tốc BLAKE3 để sinh mã định danh duy nhất (Content ID).  
  2. *Deduplication Query:* Client gửi mã hash này lên NAS hỏi trạng thái tồn tại.  
  3. *Zero-Upload Rekeying:* Nếu file đã tồn tại, Client hủy lệnh upload. Nó chỉ lấy Key mã hóa của file gốc trên NAS, bọc lại bằng Key của người nhận mới (giao thức Rekeying) và ghi một tham chiếu metadata (vài KB). Tiết kiệm đến **70% dung lượng thực tế trên NAS**.  
* **Dumb Speedway Router (Trạm trung chuyển phi trạng thái):** Mac Mini **tuyệt đối không giải mã** dữ liệu. Toàn bộ việc tính toán mật mã (MLS), index văn bản (FTS5), nén dữ liệu (Zstd) được đẩy sạch về thiết bị người dùng (Client-Side Compute). Sử dụng cơ chế bất đồng bộ không chặn (Non-blocking I/O) qua kqueue (macOS) của Rust tokio, một con Mac Mini M-series đơn lẻ có thể duy trì **50,000 kết nối đồng thời** với CPU tiêu thụ \< 15%.  
* **Phân tầng Lưu trữ & Cách ly I/O (MemFS I/O Isolation):** HDD của NAS có tốc độ ghi ngẫu nhiên (Random Write) cực kỳ tệ. Khi hàng ngàn người chat cùng lúc, SQLite WAL ghi liên tục sẽ làm nghẽn đầu đọc (Disk I/O Churn).  
  1. *RAM-Disk / MemFS Buffer:* Trên Mac Mini, mount một phân vùng bộ nhớ tạm RAM-Disk (MemFS) 2GB \- 4GB. Mọi giao dịch ghi nóng (active routing, presence, SQLite WAL) được thực thi trực tiếp trên RAM-Disk này với độ trễ microsecond.  
  2. *Sequential Sync:* Định kỳ mỗi 5 giây, tiến trình Rust Core gom toàn bộ giao dịch trên RAM-Disk thành các khối tuần tự lớn (Sequential Write Blocks) và ghi xuống NAS trong một luồng I/O duy nhất. Giải phóng 90% áp lực Disk I/O của NAS.  
* **Native Metal AI Engine (No-Docker Bypass):** Chạy Local AI (Qwen2.5) trên Docker/K8s Linux ảo hóa trên Mac Mini sẽ ngốn thêm 30% tài nguyên hao phí. TeraChat chạy trực tiếp dưới dạng daemon native của macOS (launchd plist), gọi Local AI thông qua bindings trực tiếp với **Metal API** (sử dụng llama.cpp hoặc candle viết bằng Rust), tận dụng băng thông bộ nhớ Unified Memory siêu rộng của chip Apple M-series và chạy trực tiếp trên Apple Neural Engine mà không qua ảo hóa, kiểm soát tuyệt đối nhiệt độ và xung nhịp phần cứng.  
* **ZK Memory Agent Auto-Triage bên trong TEE (Enclave vật lý):** Để tránh việc biến Mac Mini thành *Single Point of Compromise*, tiến trình dọn dẹp xung đột dữ liệu và tính toán MLS TreeKEM bắt buộc phải thực thi bên trong **Apple Secure Enclave Processor (SEP)**. Bất kỳ hành vi can thiệp vật lý hoặc RAM dumping nào từ SysAdmin biến chất cũng không thể trích xuất được Session Keys của hệ thống.

### **4.2. Option B: Enterprise Cloud-Native Scaling (Tùy chọn bổ sung \- Optional)**

Dành riêng cho các siêu tập đoàn lớn (\>100,000 users) đã có sẵn hạ tầng cluster K8s và chấp nhận chi trả OpEx lớn:

* **Confidential K8s Cluster:** Các Container stateless được đóng gói bằng Helm và Terraform, chạy trên các Node hỗ trợ Trusted Execution Environment (TEE) cấp độ phần cứng máy chủ (**Intel TDX**, **AMD SEV-SNP** hoặc **AWS Nitro Enclaves**).  
* **Distributed State (FoundationDB/CockroachDB):** Toàn bộ trạng thái (metadata, escrow) đẩy xuống cơ sở dữ liệu phân tán chuẩn HA để triệt tiêu độ trễ Leader Election của mạng Mesh-consensus cũ khi có node sập. Failover tính bằng mili-giây.

## **5\. KHUNG THỰC THI CLIENT CAO CẤP (THIN UI, FAT CORE)**

Để loại bỏ triệt để rủi ro rò rỉ ranh giới sở hữu bộ nhớ (memory ownership leakage), triệt tiêu chi phí serialization khổng lồ, và chống lại lỗi crash "câm" (Silent Crashes / Segfaults) do con trỏ lơ lửng khi chia sẻ bộ nhớ qua C-ABI thô:

### **5.1. Thiết Kế Thin UI, Fat Core & Giao tiếp An toàn (Safe IPC Bridges)**

Rust Core đóng vai trò là một headless daemon duy nhất, nắm giữ toàn quyền quản lý bộ nhớ và trạng thái. Phần UI (Flutter trên Mobile hoặc Tauri trên Desktop) bị hạ cấp thành một "Dumb Renderer" (trình hiển thị ngu ngốc) chỉ phản ứng với các state snapshot được đóng gói sẵn.

* **Cấm tự viết tay cầu nối FFI thô (No Hand-rolled C-ABI):** Tuyệt đối không tự viết tay FlatBuffers truyền nhận thô qua C-ABI vì lỗi lệch offset hoặc dọn dẹp bộ nhớ không đồng bộ sẽ gây lỗi Segment Fault hủy diệt toàn bộ ứng dụng di động ngay lập tức.  
  * **Giải pháp:** Sử dụng công cụ tự động hóa cầu nối an toàn **flutter\_rust\_bridge (FRB V2)** cho Mobile và **Tauri IPC Bridge** được tối ưu hóa cho Desktop. FRB V2 tự động map các mảng byte thô từ bộ nhớ Rust trực tiếp sang Uint8List của Dart với cơ chế quản lý vòng đời tự động (**auto-drop**), bảo vệ an toàn tuyệt đối ranh giới phân bổ bộ nhớ.  
* **Panic Boundary Scaffolding (Bắt lỗi ranh giới hoảng loạn):** Rust Core tuyệt đối không được phép gặp lỗi "panic" xuyên qua ranh giới FFI (gây sập ứng dụng di động ngay lập tức mà không ghi lại log).  
  * Toàn bộ các hàm Export ra ngoài FFI bắt buộc được bao bọc bằng macro **std::panic::catch\_unwind**. Khi Rust xảy ra lỗi crash cục bộ, macro này sẽ bắt lại Stack Trace, tự động serialize thành cấu trúc JSON và an toàn chuyển ngược về UI dưới dạng Result::Err(CorePanic { stack\_trace, context }). UI sẽ bắt lỗi này để hiển thị màn hình báo lỗi "Gracefully" hoặc tự động tái khởi động tiến trình ngầm thay vì biến mất đột ngột trên máy của nhân viên.  
* **Mô hình "UI chỉ đọc" (Immutable Snapshot Handle Pattern):** Thay vì truyền các cấu trúc dữ liệu đồ thị phức tạp (như cả một cây CRDT hay mảng tin nhắn lớn) qua cầu nối FFI gây phình to payload, Rust Core chỉ trả về một ID tham chiếu (**Handle ID**). UI sử dụng Handle ID này để gửi yêu cầu truy vấn hẹp các kiểu dữ liệu nguyên thủy (primitive types) tối giản cần hiển thị trên màn hình hiện tại, triệt tiêu tối đa rủi ro corrupt vùng nhớ dùng chung.

### **5.2. Quản lý Bộ nhớ Lõi: Arena Allocation (bumpalo)**

Các tác vụ tính toán mật mã nhóm (MLS) và trộn văn bản (CRDT/Event Log) liên tục sinh ra hàng triệu đối tượng nhỏ có thời gian sống ngắn hạn trên Heap, dễ gây phân mảnh bộ nhớ (memory fragmentation) làm sập app sau vài ngày treo máy.

* **Cơ chế:** Áp dụng mô hình **Arena Allocator** (sử dụng crate bumpalo của Rust) cho từng phiên làm việc (Session) hoặc từng chu kỳ đồng bộ.  
* **Thực thi:** Khi một thao tác MLS tính toán xong cây khóa hoặc CRDT merge xong một batch tin nhắn, thay vì phải chạy giải phóng thủ công hàng ngàn con trỏ nhỏ lẻ, Rust Core thực hiện drop toàn bộ Arena đó trong một thao tác ![][image1]. Khái niệm Memory Leak bị triệt tiêu hoàn toàn ở cấp độ kiến trúc.

### **5.3. Watchdog Process Isolation & OOM Self-Healing**

Dù Rust Core được tối ưu cực hạn, rủi ro OOM (Out of Memory) trên các thiết bị di động cấu hình yếu vẫn có thể xảy ra.

* **Cô lập Tiến trình:** Rust Core / WASM Runtime được chạy trên một process/thread hoàn toàn độc lập với luồng UI (Web Worker trên nền Web, XPC Service trên macOS, hoặc Background Service trên Android).  
* **Cơ chế Watchdog tự phục hồi:** Thiết lập một tiến trình giám sát (Watchdog) ở tầng UI. Nếu phát hiện Core Process tiêu thụ vượt ngưỡng an toàn (ví dụ: 200MB RAM trên thiết bị di động), Watchdog sẽ chủ động ra lệnh "bắn hạ" (kill) và khởi động lại Core. Nhờ cơ chế lưu trữ SQLite cục bộ kết hợp với kiến trúc Offline-first, quá trình tái kết nối này diễn ra hoàn toàn trong suốt với người dùng (seamless) dưới dạng biểu tượng "Đang kết nối lại..." trong vài mili-giây, tuyệt đối không làm crash UI ứng dụng chat của người dùng.

## **6\. HỆ SĨNH THÁI .TAPP & ĐƯỜNG ỐNG ĐA TẦNG TÍN NHIỆM (PHASE 2\)**

Việc áp dụng một chính sách bảo mật cào bằng (one-size-fits-all) cho cả cộng đồng mã nguồn mở và kỹ sư nội bộ sẽ tự bóp nghẹt hệ sinh thái của chính mình. Sự phân tách ranh giới giữa Public Marketplace (terachat.io) và Enterprise Internal Workspace đòi hỏi một kiến trúc **Dual-Trust Pipeline (Đường ống Đa tầng tín nhiệm)**, được dời sang Phase 2 để tối ưu hóa tài nguyên R\&D cho Phase 1:

### **6.1. Khung Phân phối và Quản trị Đa tầng Tín nhiệm**

* **Luồng 1: Cryptographic Manifest cho Public Marketplace (terachat.io):** Đối với các ứng dụng đại trà, TeraChat CA sẽ cấp một **Cryptographic Manifest** (Bản kê khai ký số). Manifest băm (hash) mã nguồn WASM và gắn chặt nó với một danh sách *Allowlist* các miền mạng (network domains) được phép. Nếu ứng dụng gọi một domain không khai báo (vd: fetch('https://evil-hacker.com')), WASM Sandbox lập tức kích hoạt bẫy thực thi (Trap) và tiêu diệt tiến trình.  
* **Luồng 2: Enterprise Side-loading & Private CA (Dành cho Dev Nội bộ):** Doanh nghiệp tự import Enterprise Root Certificate của chính mình. Bất kỳ .tapp nào ký bởi khóa nội bộ này nghiễm nhiên đạt cờ *Enterprise Trust*.  
  * **Chế độ TAPP\_DEV\_MODE:** Kỹ sư nội bộ bật Feature Flag này để side-load trực tiếp file .wasm cục bộ từ máy tính vào TeraChat Client để test trực tiếp mà không cần duyệt hay ký số.  
  * **Network Mocks & Intercepts:** Cho phép .tapp nội bộ gọi các virtual route như fetch('tapp-net://hr-service/api'). Rust Core tự động intercept và map alias hr-service thành IP mạng LAN/VPN thực tế của doanh nghiệp.  
  * **Visual Flagging (Cảnh báo Thị giác):** Bất kỳ .tapp side-load hoặc Internal CA chưa qua Audit kỹ sẽ bị đóng dải màu vàng cảnh báo trực quan trên UI: *"Unverified Enterprise Tapp \- Logging Enabled"*, đảm bảo minh bạch tuyệt đối và chống lập trình viên nội bộ lạm dụng lén lút.

### **6.2. Giải cứu UI: Mô hình Null-Origin Iframe & IPC Bridge**

Thay vì bắt lập trình viên dùng JSON Schema khô khan và vẽ UI bằng JSON tĩnh (gây ức chế và bóp chết DX), TeraChat bóc tách .tapp thành hai luồng độc lập, cho phép họ viết UI bằng bất kỳ framework nào họ muốn (React, Vue, Canvas, WebGL):

* **UI Thread (Frontend):** Chạy trong một \<iframe sandbox="allow-scripts"\> với thuộc tính **Null-Origin** (không có allow-same-origin). Giao diện này hoàn toàn bị cô lập khỏi mạng lưới bên ngoài (chặn bằng Content Security Policy \- CSP) và không thể đọc DOM của TeraChat. Nó được phép dùng WebGL, Canvas để render thoải mái.  
* **Logic Thread (WASM Core):** Chạy nền (background), chứa trạng thái, logic kinh doanh và quyền truy cập dữ liệu mật mã thông qua Host ABI.  
* **Giao tiếp qua IPC Bridge:** Hai luồng giao tiếp với nhau duy nhất qua cơ chế **Message Passing (IPC)** (ví dụ: postMessage). UI trở thành một bức tranh vô tri — chỉ render những gì WASM bảo render và gửi ngược event khi user click. Lỗ hổng XSS trong Iframe hoàn toàn vô hại vì nó bị tước bỏ mọi quyền truy cập Network và LocalStorage.

### **6.3. Phân tầng Runtime: Tapp Trust Tiers & OOM Isolation**

Không phải mini-app nào cũng thao tác với Key mật mã cấp Quốc phòng. Ép giới hạn khắt khe lên mọi loại ứng dụng là biểu hiện của Over-engineering. TeraChat phân tầng tin cậy:

* **Tier 0 (Hệ thống/Crypto):** Giới hạn khắt khe (50MB RAM, cấm float, no-network, Rust tĩnh). Chỉ dành cho Core Modules hệ thống.  
* **Tier 1 (Enterprise Standard):** Mở rộng RAM lên 256MB/512MB. Cho phép sử dụng JavaScript/TypeScript được biên dịch sang WASM (thông qua Javy hoặc Extism) để lập trình viên tự do phát triển. Cho phép float math.  
* **OOM Isolation:** Chấp nhận rủi ro app Tier 1 có thể bị rò rỉ bộ nhớ (memory leak) do code JS/TS chưa tối ưu. Tuy nhiên, nếu một .tapp Tier 1 vượt quá 256MB RAM, TeraChat Client sẽ tự động kill tiến trình của .tapp đó và hiển thị nút "Reload Plugin" trên UI, tuyệt đối không làm ảnh hưởng đến ứng dụng chat chính.

### **6.4. Tái định nghĩa Network Egress: Declarative Proxy & DLP Integration**

Hạn mức Outbox 2MB cứng nhắc bị loại bỏ để hỗ trợ các luồng truyền dữ liệu liên tục hoặc fetch file lớn.

* **Cơ chế:** Áp dụng mô hình **Capability-based Network** (Quyền mạng khai báo giống Deno/Android). Lập trình viên khai báo manifest.json chứa danh sách domain được phép gọi (ví dụ: api.github.com).  
* **Kiểm soát:** .tapp không gọi mạng trực tiếp, mà phải gọi qua một hàm API của TeraChat Core: host.fetch(url). Core sẽ đối chiếu với manifest, và ở cấp độ Enterprise, request này tiếp tục được đẩy qua hệ thống DLP (Data Loss Prevention) proxy của công ty trước khi ra ngoài Internet.

### **6.5. Zero-Copy IPC & Virtual File System (VFS)**

Để giải quyết triệt để nút thắt cổ chai I/O khi truyền các tệp tin lớn (tránh việc serialize thành base64/bytes thô làm đơ giao diện), TeraChat thiết lập:

* **WASI Virtual File System (VFS):** Gắn (mount) một phân vùng Virtual Memory File System trực tiếp vào môi trường WASI của sandbox.  
* **Zero-Copy Memory Sharing:** Sử dụng cấu trúc SharedArrayBuffer (trên Desktop Tauri) và Dart FFI TypedData (trên Mobile Flutter) để chia sẻ trực tiếp con trỏ vùng nhớ (Memory Pointers) qua Host ABI, loại bỏ hoàn toàn chi phí sao chép vùng nhớ.

## **7\. RÃNH HÀO THƯƠNG MẠI & BẢO CHỨNG KỸ THUẬT (MARKETABLE MOATS)**

Các giới hạn khắc nghiệt của Zero-Knowledge đã được đội ngũ hệ thống đóng gói thành 4 Trụ cột Thương mại mạnh mẽ dành cho Sales/Marketing:

### **7.1. Trụ cột 1: Bảo mật Hậu Nhân sự Tuyệt đối (Absolute Post-Employment Security)**

* **Giá trị:** "Chỉ 1 click trên IdP, dữ liệu công ty trên máy nhân viên cũ bốc hơi thành rác, kể cả khi máy đang tắt mạng."  
* **Bảo chứng Kỹ thuật:** Webhook từ IdP (Google/Azure AD) kích hoạt xoay khóa OpenMLS (Cryptographic Eviction). Lõi hot\_dag.db lập tức trở thành khối ciphertext vô nghĩa, thay thế hoàn toàn cho lệnh Remote Wipe vật lý dễ bị bypass.

### **7.2. Trụ cột 2: Trải nghiệm Di động Zero-Bloat (Mô hình Quản trị Dữ liệu Thông minh)**

*Giải quyết bệnh phình dung lượng app làm cạn kiệt ổ cứng di động (Local Bloat) của các ứng dụng chat truyền thống.*

* **Bảo chứng Kỹ thuật:**  
  * **7-Day Sliding Window & Snapshot-Delta Sync:** Mobile giới hạn vòng đời dữ liệu cục bộ ở mức 7 ngày. Lịch sử cũ được kéo từ server dưới dạng **Delta Sync** chèn thẳng vào RAM để xem, xem xong drop ngay, không ghi xuống ổ đĩa.  
  * **Hard Pruning (Chống phình WAL trên iOS):** Nhận diện rủi ro SQLite WAL phình to gây Jetsam kill, hệ thống ép chạy PRAGMA wal\_checkpoint(TRUNCATE) khi App vào Background.  
  * **Aggressive Media Eviction:** Chỉ lưu bản Thumbnail nén siêu nhỏ trên máy. File gốc tự xóa khỏi RAM điện thoại sau 72h, nhưng vẫn tồn tại an toàn trong cụm MinIO Object Storage của doanh nghiệp. Tải lại theo nhu cầu (Fetch-on-demand) kết hợp P2P CDN nội bộ.

### **7.3. Trụ cột 3: Vận hành Không Ma Sát & Khả năng Sinh tồn (Autonomous Resilience)**

* **Giá trị:** "Cắm điện là chạy. Đứt mạng internet, hệ thống tự động fallback và duy trì liên lạc nội bộ ổn định."  
* **Bảo chứng Kỹ thuật:**  
  * Mạng nhận thức phần cứng (DMS Topology) tự động đẩy thiết bị yếu xuống Leaf Node; Tự động fallback linh hoạt giữa LAN ↔ mDNS ↔ BLE.  
  * **Progressive Loading:** Kết hợp Priority Packet Queue ở mạng T3, tin nhắn dài tự động hiển thị 50 từ đầu ngay lập tức (High Priority), các chunk sau được load ngầm khi lướt qua node khác thay vì cắt cụt cứng gây ức chế.  
  * **Ứng phó thông minh theo điều kiện môi trường:** Tự động điều chỉnh chu kỳ quét của chip thu phát sóng dựa trên cảm biến di chuyển (**Sensor-based Duty Cycling**), và trì hoãn các tác vụ nén/hash tài nguyên lớn khi pin yếu để bảo vệ tối đa trải nghiệm thực tế của người dùng.

### **7.4. Trụ cột 4: Tuân thủ Pháp lý Tối cao & Chống Độc tài (Compliance Security)**

* **Giá trị:** "Khả năng xuất dữ liệu thanh tra pháp lý mà không tạo Backdoor. Miễn nhiễm với các đợt hack tài khoản Admin."  
* **Bảo chứng Kỹ thuật:** Khóa Compliance Escrow phân rã đa chữ ký, cần sự phê duyệt Quorum. Việc ủy thác tính toán MLS và ZK Agent được đưa vào **TEE (Trusted Execution Environment)** của cụm máy chủ hoặc Apple Silicon Secure Enclave trên Mac Mini, triệt tiêu hoàn toàn rủi ro bị can thiệp vật lý hay root dump.

## **8\. CHỨNG CHỈ AN NINH & ĐIỀU KIỆN TUÂN THỦ CHÍNH PHỦ**

* **FIPS 140-3 Validation:** Lõi tc-crypto hướng tới chuẩn Level 2/3, chạy chế độ Hybrid-mode PQC (ML-KEM-768 \+ X25519). Khởi tạo khóa yêu cầu HW TRNG.  
* **Common Criteria EAL4+:** Audit tính toàn vẹn của Sandbox WASM và IPC FFI Memory Safety.  
* **Ban Cơ yếu Chính phủ (VN):** Kiểm toán mã nguồn không Backdoor, đáp ứng pháp lý bằng Compliance Escrow.  
* **DISA STIGs:** Script tự động Hardening OS macOS/Linux của cụm Cloud-Native / Node Edge đạt chuẩn quốc phòng.

[image1]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACsAAAAZCAYAAACo79dmAAAB80lEQVR4Xu2WPUtcQRSGj4pioyD4hYXaxL+QUlSwsFKwCCii2GhCYgT/gNhaWFhYK2phLaKV6WwEsVEsBD9ANEFMERJiPjwv567Mvjuzc9VVEPaBl9155tzZmXvv3rkiRV4PbSwidLN4LC2aBc28ppr6fPxnkYIKzXeWD2FO7IeHknaz5krz674il0tNHUuHKc04y4Q+zS7LGKVik/zCHQl/NP9Yii0Ki2FWNb/FxkTeZ3dngf4GlvnAAccsHTrFarrIwzWSY2KTHdX8ZBniXOL3XObMrzmuKXExYpMFacaRdrHCbfJMjVjdjeMWxS51jLST7WHJ3IoVVnIHMShWt+e4v5plpx0izWSx6E2WDAZKcwmOxOrGHIf2jNMOgboPLIlDzQlLl3pJP1lfHdoj5Hyg7iNLYkNyx8+iTKwg9k/sF6vjxxrcMDkfqPvEkliXyGSB74wxoRq4aZYeUPeZJXGguWbJYLvzTSTDqVh/OXeI+SWWHlA3yZLADrnF0gcG22epfBV7WoRYkfzbMKgVG3+WOwjU9LIM8U3sgB2xexjf32ZV5NIq4auCzQOLxYZzlnziHSL0XA6NU1DwI9gwnsI7zQXL52BC7Mw9BSw49n5RMPBGVsUyJR0S3+oLzmPuObwc/WD5EpRo3rCMMMCiSJE83AG6Vn40dnExgwAAAABJRU5ErkJggg==>