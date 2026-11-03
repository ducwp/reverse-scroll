# ⇅ ReverseScroll — Đảo chiều cuộn riêng cho Chuột và Trackpad trên Mac

![Version](https://img.shields.io/badge/version-v0.4.0-blue.svg)
![Platform](https://img.shields.io/badge/platform-macOS%2011%2B-black.svg)
![Architecture](https://img.shields.io/badge/architecture-Universal%20(Apple%20Silicon%20%26%20Intel)-brightgreen.svg)

> **ReverseScroll** là một ứng dụng nhỏ gọn nằm trên thanh Menu Bar của macOS, giúp bạn **đảo chiều cuộn của chuột và của trackpad một cách độc lập**.
>
> Vấn đề quen thuộc: macOS chỉ có **một** công tắc "Cuộn tự nhiên" dùng chung cho cả chuột và trackpad. Bạn muốn trackpad cuộn như điện thoại, nhưng con lăn chuột lại cuộn kiểu truyền thống? ReverseScroll giải quyết đúng chuyện đó, chỉ với một cú click trên menu.

---

## 📥 TẢI VỀ & CÀI ĐẶT NHANH (Khoảng 2 phút)

### ⬇️ Bước 1: Vào trang Releases để tải bộ cài

1. Mở trang **[Releases của ReverseScroll](https://github.com/<USER>/<REPO>/releases/latest)**.
   *(Hoặc vào trang chính của dự án, nhìn cột bên phải, bấm vào mục **Releases** → chọn bản mới nhất ở trên cùng.)*
2. Kéo xuống phần **Assets** (nằm cuối mô tả của bản phát hành).
3. Bấm vào file có đuôi **`.dmg`**, ví dụ **`ReverseScroll-0.4.0.dmg`** để tải về.

> ⚠️ Trong Assets còn có hai mục **"Source code (zip)"** và **"Source code (tar.gz)"**. Đó chỉ là mã nguồn của dự án, **không phải** ứng dụng. Bạn chỉ cần tải file **`.dmg`**.

> 💡 Nếu chưa thấy file `.dmg` trong Assets, có thể bản phát hành vẫn đang được đóng gói. Hãy đợi vài phút rồi tải lại trang.

---

### 📦 Bước 2: Cài đặt vào máy

1. Nhấp đúp vào file **`.dmg`** vừa tải để mở.
2. **Kéo biểu tượng `ReverseScroll` vào thư mục `Applications` (Ứng dụng)** trong cửa sổ hiện ra.
3. Đóng cửa sổ, rồi có thể xoá file `.dmg` đi.

---

### 🛡️ Bước 3: Mở app & xử lý cảnh báo bảo mật của macOS (làm 1 lần duy nhất)

ReverseScroll là ứng dụng miễn phí, chưa được Apple xác minh (chưa mua chứng chỉ Developer trả phí). Vì vậy khi mở lần đầu, macOS Gatekeeper sẽ hiện cảnh báo kiểu:

> *"Không thể xác minh ReverseScroll không chứa phần mềm độc hại…"*

Đây là bình thường với các app tải từ internet mà chưa được Apple xác minh. Bạn chọn **1 trong 2 cách** dưới đây:

#### 👉 Cách A: Dùng chuột (không cần gõ lệnh)

1. Mở `ReverseScroll` từ thư mục **Applications**. Khi hiện cảnh báo, bấm **Xong** (đừng chọn "Chuyển vào thùng rác").
2. Mở **Cài đặt hệ thống** ➔ **Quyền riêng tư & Bảo mật**.
3. Cuộn xuống mục **Bảo mật**, bạn sẽ thấy dòng thông báo `ReverseScroll` đã bị chặn.
4. Bấm nút **Vẫn mở** (Open Anyway) ➔ quét Touch ID hoặc nhập mật khẩu máy để xác nhận ➔ bấm **Mở**.

> Dòng thông báo ở bước 3 chỉ hiện trong khoảng một giờ sau lần mở bị chặn. Nếu không thấy, hãy mở lại app một lần nữa.

#### 👉 Cách B: Dùng Terminal (nhanh nhất)

Mở ứng dụng **Terminal**, dán dòng lệnh sau rồi nhấn **Enter**:

```bash
xattr -cr /Applications/ReverseScroll.app
```

Sau đó mở `ReverseScroll` bình thường: nhấn `Cmd + Space`, gõ `ReverseScroll` rồi nhấn Enter, hoặc mở từ Launchpad.

> ℹ️ Cách "chuột phải vào app rồi chọn Mở" chỉ còn hiệu quả trên macOS cũ. Từ macOS Sequoia (15) trở đi, hãy dùng Cách A hoặc Cách B.

---

### 🔐 Bước 4: Cấp quyền hoạt động

Để đọc và đảo chiều thao tác cuộn, app cần quyền **Trợ năng (Accessibility)**. Khi bạn mở app lần đầu, trang cài đặt tương ứng sẽ tự bật lên giúp bạn.

1. Vào **Cài đặt hệ thống ➔ Quyền riêng tư & Bảo mật ➔ Trợ năng**.
2. Gạt bật công tắc **ReverseScroll**. (Nếu chưa thấy trong danh sách, bấm nút **+** rồi chọn `ReverseScroll` trong thư mục Applications.)
3. Nếu macOS hỏi thêm quyền **Giám sát đầu vào (Input Monitoring)**, vào **Quyền riêng tư & Bảo mật ➔ Giám sát đầu vào** và bật tương tự.

Khi trên menu của app hiện dòng **✅ Đang hoạt động**, mọi thứ đã sẵn sàng. App sẽ tự kích hoạt ngay khi bạn cấp quyền xong, không cần mở lại.

> 🔒 **An toàn:** App chỉ xử lý sự kiện cuộn chuột để đảo chiều. App không kết nối mạng và không thu thập hay gửi đi bất kỳ dữ liệu nào của bạn. Mã nguồn công khai nên bạn có thể tự kiểm tra.

---

## 🎮 HƯỚNG DẪN SỬ DỤNG

ReverseScroll nằm gọn trên thanh Menu Bar góc trên bên phải, với biểu tượng hai mũi tên **⇅**. Bấm vào để mở menu:

| Mục trong menu | Ý nghĩa |
|----------------|---------|
| **✅ Đang hoạt động** | Dòng trạng thái. Nếu hiện ⚠️ nghĩa là chưa có quyền Trợ năng. |
| **Đảo chiều chuột** | Đảo chiều cuộn của con lăn chuột ngoài. *(mặc định: bật)* |
| **Đảo chiều trackpad** | Đảo chiều cuộn của trackpad và Magic Mouse. *(mặc định: tắt)* |
| **Đảo cả cuộn ngang** | Đảo luôn chiều cuộn ngang cho thiết bị đang được bật ở trên. *(mặc định: bật)* |
| **Mở khi đăng nhập** | Tự khởi động ReverseScroll mỗi khi bật máy. |
| **Thoát** | Tắt ứng dụng. |

Hai tuỳ chọn **chuột** và **trackpad** hoàn toàn **độc lập**, bạn bật hoặc tắt từng cái tuỳ ý:

| Chuột | Trackpad | Kết quả |
|:-----:|:--------:|---------|
| ☑ | ☐ | Chỉ đảo chiều chuột *(mặc định)* |
| ☐ | ☑ | Chỉ đảo chiều trackpad |
| ☑ | ☑ | Đảo chiều cả hai |
| ☐ | ☐ | Không đảo gì, giữ nguyên theo hệ thống |

> 💡 **"Đảo" nghĩa là ngược với công tắc "Cuộn tự nhiên" của macOS.**
> Cách dùng phổ biến nhất: để macOS **bật** "Cuộn tự nhiên" (Cài đặt hệ thống ➔ Bàn di chuột / Chuột), rồi tích **Đảo chiều chuột** trong ReverseScroll. Kết quả: trackpad cuộn tự nhiên, còn con lăn chuột cuộn kiểu truyền thống.
> Nếu bạn tắt "Cuộn tự nhiên" ở macOS thì kết quả sẽ ngược lại.

Mọi lựa chọn đều được **tự động ghi nhớ** cho những lần mở sau.

---

## ❓ CÂU HỎI THƯỜNG GẶP (FAQ)

**1. Máy Mac chip Intel có dùng được không?**

> **Có.** ReverseScroll là bản Universal, chạy native trên cả chip Apple Silicon (M1, M2, M3, M4…) lẫn chip Intel. Yêu cầu macOS 11 trở lên.

**2. Menu báo "⚠️ Cần quyền Trợ năng" dù tôi đã bật công tắc?**

> Mỗi bản cài mới có thể bị macOS coi là một app khác, nên quyền cũ không còn tác dụng dù công tắc vẫn hiện bật. Mở **Terminal** và chạy lần lượt:
>
> ```bash
> killall ReverseScroll
> tccutil reset Accessibility com.example.reversescroll
> tccutil reset ListenEvent com.example.reversescroll
> open /Applications/ReverseScroll.app
> ```
>
> Sau đó bật lại quyền **Trợ năng** (và **Giám sát đầu vào** nếu được hỏi) cho ReverseScroll.

**3. Menu báo "✅ Đang hoạt động" nhưng chiều cuộn không thay đổi?**

> Thoát các ứng dụng khác cũng can thiệp vào cuộn chuột, như **Scroll Reverser**, **Mac Mouse Fix**, **Logi Options+**… Chúng có thể đảo thêm một lần nữa hoặc tự tạo sự kiện cuộn riêng, làm hai bên triệt tiêu nhau.

**4. Chuột của tôi cứ bị xử lý như trackpad (hoặc ngược lại)?**

> App phân biệt chuột và trackpad dựa vào loại sự kiện cuộn mà thiết bị gửi. Một số chuột có driver riêng (Logitech, Razer…) gửi kiểu cuộn giống trackpad nên có thể bị nhận nhầm. Cách xử lý đơn giản: bật tuỳ chọn tương ứng với thiết bị mà app đang nhận diện chuột của bạn, hoặc thoát phần mềm driver chuột đó rồi thử lại.

**5. Tôi không thấy biểu tượng ⇅ trên Menu Bar?**

> - Trên MacBook có tai thỏ (notch), nếu thanh menu quá đầy thì các biểu tượng bên phải có thể bị che. Hãy thoát bớt các app khác trên menu bar.
> - Kiểm tra `ReverseScroll` có đang chạy không bằng **Activity Monitor**.
> - Nếu vẫn không thấy, xem file nhật ký ở `~/Library/Logs/ReverseScroll.log`.

**6. Làm sao để ReverseScroll tự chạy khi mở máy?**

> Bấm vào biểu tượng ⇅ trên Menu Bar ➔ tích chọn **Mở khi đăng nhập**.

**7. Cập nhật lên phiên bản mới như thế nào?**

> Vào lại trang **[Releases](https://github.com/<USER>/<REPO>/releases/latest)**, tải file `.dmg` mới nhất, rồi:
>
> 1. Thoát ReverseScroll cũ (menu ⇅ ➔ **Thoát**).
> 2. Kéo bản mới vào `Applications` và chọn **Thay thế**.
> 3. Nếu macOS chặn, làm lại **Bước 3** ở trên. Nếu mất quyền, làm theo câu hỏi số 2.

**8. Gỡ cài đặt như thế nào?**

> 1. Bấm biểu tượng ⇅ ➔ bỏ chọn **Mở khi đăng nhập** (nếu đang bật) ➔ **Thoát**.
> 2. Xoá `ReverseScroll.app` khỏi thư mục Applications.
> 3. (Tuỳ chọn) Xoá các file còn lại: `~/.config/reverse-scroll.conf` (cài đặt) và `~/Library/Logs/ReverseScroll.log` (nhật ký).
> 4. (Tuỳ chọn) Vào **Trợ năng** và xoá mục `ReverseScroll` khỏi danh sách.

---

## 🛠️ Dành Cho Lập Trình Viên (Tự Biên Dịch Từ Mã Nguồn)

Yêu cầu: macOS 11 trở lên, [Rust](https://rustup.rs) và Xcode Command Line Tools (`xcode-select --install`).

```bash
git clone https://github.com/<USER>/<REPO>.git
cd <REPO>
chmod +x build-dmg.sh
./build-dmg.sh
```

File cài đặt sẽ nằm ở `dist/ReverseScroll-<phiên bản>.dmg`.

---

## 📝 Nhật Ký Phiên Bản (Changelog)

### [v0.4.0]

- Hai tuỳ chọn **Đảo chiều chuột** và **Đảo chiều trackpad** hoạt động **độc lập**, kèm tuỳ chọn **Đảo cả cuộn ngang**.
- Ứng dụng nằm trên **Menu Bar** với biểu tượng ⇅ riêng (tự đổi màu theo giao diện sáng/tối) và có **icon ứng dụng**.
- Hỗ trợ **Mở khi đăng nhập**, tự ghi nhớ cài đặt.
- Tự động chờ và kích hoạt ngay khi bạn cấp quyền **Trợ năng**, không cần mở lại app.
- Bản cài **Universal**: chạy native trên cả Apple Silicon và Intel.
- Có file nhật ký `~/Library/Logs/ReverseScroll.log` giúp chẩn đoán khi có sự cố.

---

## 👤 Tác Giả & Bản Quyền

- Phát triển bởi **<Tên của bạn>**.
- Ứng dụng miễn phí. Nếu bạn thấy hữu ích, hãy cho dự án một ⭐ trên GitHub nhé!