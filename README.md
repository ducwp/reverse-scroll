# ReverseScroll

Ứng dụng nhỏ trên thanh menu của macOS, viết bằng Rust, dùng để **đảo chiều cuộn** của **chuột** và **trackpad** một cách độc lập. Ví dụ: trackpad cuộn kiểu "tự nhiên" như iPhone, còn con lăn chuột cuộn kiểu truyền thống.

## Tính năng

- Icon `⇅` trên menu bar, không hiện trên Dock.
- Hai tuỳ chọn **độc lập** với nhau:
  - **Đảo chiều chuột**: áp dụng cho con lăn chuột ngoài.
  - **Đảo chiều trackpad**: áp dụng cho trackpad (và Magic Mouse).
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
2. Mở app. Nếu macOS chặn, chuột phải vào app, chọn **Mở**.
3. Cấp quyền tại **Cài đặt hệ thống → Quyền riêng tư & Bảo mật → Trợ năng** (Accessibility): bật `ReverseScroll`. Lần đầu mở app sẽ tự mở trang này giúp bạn.
4. Nếu macOS yêu cầu, bật thêm tại **Giám sát đầu vào** (Input Monitoring).

Khi menu hiện **✅ Đang hoạt động** là app đã sẵn sàng.

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

(Giá trị `1` là bật, `0` là tắt. Các khoá cũ `enabled` và `all` của bản trước không còn được dùng.)

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
