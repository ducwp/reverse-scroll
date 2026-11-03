# ReverseScroll

Ứng dụng nhỏ trên thanh menu của macOS, viết bằng Rust, dùng để **đảo chiều cuộn** của **chuột** và **trackpad** một cách độc lập. Ví dụ: trackpad cuộn kiểu "tự nhiên" như iPhone, còn con lăn chuột cuộn kiểu truyền thống.

## Vấn đề

macOS chỉ có **một** công tắc "Cuộn tự nhiên" (Cài đặt hệ thống → Chuột / Bàn di chuột) dùng chung cho cả chuột và trackpad. ReverseScroll cho phép bạn đảo chiều riêng từng thiết bị, ngược với cài đặt chung đó.

## Tính năng

- Icon `⇅` trên menu bar, không hiện trên Dock.
- Hai tuỳ chọn **độc lập** với nhau:
  - **Đảo chiều chuột**: áp dụng cho con lăn chuột ngoài (mặc định: bật).
  - **Đảo chiều trackpad**: áp dụng cho trackpad và Magic Mouse (mặc định: tắt).
- **Đảo cả cuộn ngang**: áp dụng cho thiết bị nào đang được bật ở trên.
- **Mở khi đăng nhập**.
- Tự lưu cài đặt, tự chờ và kích hoạt khi bạn cấp quyền Trợ năng.
- Binary universal: chạy trên cả Apple Silicon và Intel.

| Chuột | Trackpad | Kết quả |
|:-----:|:--------:|---------|
| ☑ | ☐ | Chỉ đảo chuột (mặc định) |
| ☐ | ☑ | Chỉ đảo trackpad |
| ☑ | ☑ | Đảo cả hai |
| ☐ | ☐ | Không đảo gì (giữ nguyên hệ thống) |

> **"Đảo" là đảo so với công tắc "Cuộn tự nhiên" của hệ thống.** Ví dụ: hệ thống đang bật Cuộn tự nhiên, bạn tích **Đảo chiều chuột** thì chuột cuộn kiểu truyền thống còn trackpad vẫn cuộn tự nhiên. Nếu bạn tắt Cuộn tự nhiên ở hệ thống thì kết quả sẽ ngược lại. Cách dùng phổ biến nhất: để hệ thống bật Cuộn tự nhiên, rồi tích **Đảo chiều chuột**.

## Yêu cầu

- macOS 11 trở lên.
- Để tự build: [Rust](https://rustup.rs) và Xcode Command Line Tools (`xcode-select --install`).

## Build

```bash
git clone <repo của bạn> reverse-scroll && cd reverse-scroll
chmod +x build-dmg.sh
./build-dmg.sh
```

Kết quả là `dist/ReverseScroll-<phiên bản>.dmg`. Script sẽ:

1. Build cho `aarch64-apple-darwin` và `x86_64-apple-darwin`, gộp bằng `lipo`.
2. Vẽ icon app (`examples/make_icon.rs`) và tạo `ReverseScroll.icns`.
3. Tạo `ReverseScroll.app`, ký ad-hoc, đóng gói DMG.

Chỉ chạy thử, không đóng gói:

```bash
cargo run --release
```

(Khi chạy bằng `cargo run`, quyền Trợ năng phải được cấp cho **Terminal**.)

## Cài đặt và cấp quyền

1. Mở file `.dmg`, kéo `ReverseScroll` vào `Applications`.
2. Mở app. Nếu macOS chặn (thường gặp với bản tải từ internet), làm theo mục [Mở app sau khi tải về](#mở-app-sau-khi-tải-về-gatekeeper) bên dưới.
3. Cấp quyền tại **Cài đặt hệ thống → Quyền riêng tư & Bảo mật → Trợ năng** (Accessibility): bật `ReverseScroll`. Lần đầu mở app sẽ tự mở trang này giúp bạn.
4. Nếu macOS yêu cầu, bật thêm tại **Giám sát đầu vào** (Input Monitoring).

Khi menu hiện **✅ Đang hoạt động** là app đã sẵn sàng.

## Mở app sau khi tải về (Gatekeeper)

App được ký ad-hoc và chưa notarize bởi Apple. Vì vậy khi tải file DMG từ internet (GitHub Releases, Artifacts...), macOS gắn nhãn "đã tải từ internet" (quarantine) và Gatekeeper sẽ chặn lần mở đầu tiên, với thông báo kiểu "Không thể xác minh ReverseScroll không chứa phần mềm độc hại". Điều này áp dụng cho **mọi người tải về, kể cả người build ra nó**. Bản tự build trên máy mình thì không có nhãn này nên không bị chặn.

Đây không phải lỗi của app. Bạn chọn một trong hai cách dưới đây (chỉ cần làm một lần cho mỗi bản tải về). Chỉ nên làm với file bạn tin tưởng, ví dụ tải từ chính repo này.

> Lưu ý: cách "chuột phải vào app rồi chọn **Mở**" chỉ hiệu quả trên macOS cũ. Từ macOS Sequoia (15) trở đi, Apple đã bỏ lối tắt đó, nên hãy dùng một trong hai cách sau.

### Cách 1: Qua giao diện (Cài đặt hệ thống)

1. Mở file `.dmg`, kéo `ReverseScroll` vào thư mục `Applications`.
2. Mở `ReverseScroll` từ `Applications`. macOS sẽ báo không thể xác minh app. Bấm **Xong** (hoặc **Hủy**), đừng chọn "Move to Trash / Chuyển vào thùng rác".
3. Vào **Cài đặt hệ thống → Quyền riêng tư & Bảo mật**.
4. Kéo xuống phần **Bảo mật**. Bạn sẽ thấy dòng nhắc rằng `ReverseScroll` đã bị chặn. Bấm nút **Vẫn mở** (Open Anyway).
5. Nhập mật khẩu hoặc dùng Touch ID để xác nhận, rồi bấm **Mở** (Open) trong hộp thoại cuối.

Dòng nhắc ở bước 4 chỉ hiện trong khoảng một giờ sau lần mở bị chặn. Nếu không thấy, hãy mở lại app một lần nữa để nó xuất hiện.

### Cách 2: Qua Terminal (gỡ nhãn quarantine)

Cách này nhanh hơn: gỡ nhãn "tải từ internet" khỏi app, rồi mở như app bình thường.

1. Kéo `ReverseScroll` từ DMG vào `Applications`.
2. Mở **Terminal** và chạy:
   ```bash
   xattr -cr /Applications/ReverseScroll.app
   ```
   (Lệnh này xoá mọi thuộc tính mở rộng, gồm cả nhãn quarantine. Nếu chỉ muốn xoá đúng nhãn quarantine, dùng `xattr -dr com.apple.quarantine /Applications/ReverseScroll.app`.)
3. Mở `ReverseScroll` bình thường (double-click hoặc `open /Applications/ReverseScroll.app`).

Nếu bạn để app ở chỗ khác, thay đường dẫn tương ứng. Kiểm tra nhãn còn không bằng:
```bash
xattr -l /Applications/ReverseScroll.app
```
Nếu kết quả không còn dòng `com.apple.quarantine` là đã gỡ xong.

### Sau khi mở được app

- Cấp quyền **Trợ năng** (và **Giám sát đầu vào** nếu macOS yêu cầu) như hướng dẫn ở mục trên.
- Mỗi lần cài một **bản cập nhật mới** (tải DMG mới), bạn cần làm lại bước vượt Gatekeeper ở trên. Quyền Trợ năng cũng có thể bị coi là của app khác và phải cấp lại, xem mục Xử lý sự cố.
- Muốn người dùng mở app mà không thấy cảnh báo nào, cần ký bằng Apple Developer ID và notarize, xem mục [Phát hành cho người khác](#phát-hành-cho-người-khác).

## Cài đặt và dữ liệu

| Mục | Vị trí |
|-----|--------|
| Cấu hình | `~/.config/reverse-scroll.conf` |
| Log | `~/Library/Logs/ReverseScroll.log` |
| Tự chạy khi đăng nhập | `~/Library/LaunchAgents/com.example.reversescroll.plist` |

File cấu hình có dạng:

```
mouse=1
trackpad=0
horizontal=1
```

(Giá trị `1` là bật, `0` là tắt. Các khoá `mouse_natural` và `trackpad_natural` của bản 0.5.0 không được dùng ở bản này và sẽ bị bỏ qua.)

## Xử lý sự cố

**Menu báo "Cần quyền Trợ năng" dù đã bật công tắc.**
Với ký ad-hoc, mỗi lần build lại macOS coi là một app mới. Chạy:
```bash
killall ReverseScroll
tccutil reset Accessibility com.example.reversescroll
tccutil reset ListenEvent com.example.reversescroll
open /Applications/ReverseScroll.app
```
rồi cấp lại quyền Trợ năng và Giám sát đầu vào.

**Báo "Đang hoạt động" nhưng cuộn không đổi.**
- Thoát các app cũng can thiệp vào cuộn (Scroll Reverser, Mac Mouse Fix, Logi Options+...). Chúng có thể đảo thêm lần nữa hoặc tự tạo sự kiện cuộn riêng.
- Cuộn vài lần rồi xem log:
  ```bash
  tail -n 20 ~/Library/Logs/ReverseScroll.log
  ```
  Mỗi dòng ghi rõ thiết bị được nhận diện là `chuột` hay `trackpad` và app có `ĐẢO` hay `BỎ QUA`. Nếu chuột của bạn bị nhận nhầm thành `trackpad` (do driver gửi cuộn "liên tục"), hãy bật tuỳ chọn **Đảo chiều trackpad**.

**Không thấy icon trên menu bar.**
Trên máy có tai thỏ (notch) hoặc khi thanh menu quá đầy, icon có thể bị che. Thoát bớt app menu bar khác. Kiểm tra `ReverseScroll` có đang chạy trong Activity Monitor không, và đọc file log.

**Gỡ cài đặt.**
Tắt "Mở khi đăng nhập" trong menu, thoát app, rồi xoá `ReverseScroll.app`, `~/.config/reverse-scroll.conf` và `~/Library/Logs/ReverseScroll.log`. Xoá thêm mục `ReverseScroll` trong Trợ năng nếu muốn.

## Cách hoạt động

App tạo một `CGEventTap` ở tầng HID để chặn sự kiện `ScrollWheel`. Với mỗi sự kiện, app đọc trước tất cả giá trị delta (số nguyên, fixed-point và point), rồi ghi lại các giá trị đã đảo dấu. Thiết bị được phân biệt bằng field `kCGScrollWheelEventIsContinuous`: bằng 1 là trackpad hoặc Magic Mouse, bằng 0 là con lăn chuột thường.

## Giới hạn

- Phân biệt thiết bị theo **loại** (chuột hay trackpad), không phân biệt được hai chuột khác nhau. Nếu cần cấu hình riêng từng thiết bị, hãy xem [LinearMouse](https://github.com/linearmouse/linearmouse).
- Nhận diện loại thiết bị dựa vào một dấu hiệu trong sự kiện, nên có thể sai với một số driver chuột bên thứ ba.
- Tuỳ chọn "Đảo" là tương đối so với công tắc "Cuộn tự nhiên" của hệ thống (xem ghi chú ở mục Tính năng).

## Cấu trúc project

```
reverse-scroll/
├── Cargo.toml
├── build-dmg.sh          # build universal, tạo icon, .app, .dmg
├── README.md
├── src/main.rs           # app menu bar + event tap
└── examples/make_icon.rs # vẽ icon app 1024x1024
```

## Phát hành cho người khác

Ký ad-hoc chỉ phù hợp dùng cá nhân. Để người khác mở không bị Gatekeeper chặn, cần Apple Developer ID:

```bash
SIGN_ID="Developer ID Application: Tên (TEAMID)" ./build-dmg.sh
xcrun notarytool submit dist/ReverseScroll-0.4.0.dmg --keychain-profile "profile" --wait
xcrun stapler staple dist/ReverseScroll-0.4.0.dmg
```

Đổi `BUNDLE_ID` trong `build-dmg.sh` (và `LOGIN_LABEL` trong `src/main.rs`) thành ID của bạn trước khi phát hành.

## Giấy phép

Chưa chọn giấy phép. Hãy thêm file `LICENSE` (ví dụ MIT) nếu bạn muốn chia sẻ mã nguồn.