#!/usr/bin/env bash
# Build ReverseScroll.app (universal: Apple Silicon + Intel, có icon) và đóng gói thành .dmg
# Chạy trên macOS, trong thư mục gốc của project: ./build-dmg.sh
set -euo pipefail

APP_NAME="ReverseScroll"
BIN_NAME="reverse-scroll"
BUNDLE_ID="com.example.reversescroll"   # đổi thành ID của bạn
VERSION="0.3.0"
DIST="dist"
APP="$DIST/$APP_NAME.app"
DMG="$DIST/$APP_NAME-$VERSION.dmg"

echo "==> Cài target Rust (nếu thiếu)"
rustup target add aarch64-apple-darwin x86_64-apple-darwin

echo "==> Build release cho 2 kiến trúc"
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin

echo "==> Chuẩn bị thư mục app"
rm -rf "$DIST"
mkdir -p "$APP/Contents/MacOS" "$APP/Contents/Resources"

echo "==> Gộp thành universal binary"
lipo -create \
  "target/aarch64-apple-darwin/release/$BIN_NAME" \
  "target/x86_64-apple-darwin/release/$BIN_NAME" \
  -output "$APP/Contents/MacOS/$APP_NAME"

echo "==> Tạo icon app (.icns)"
cargo run --release --example make_icon -- "$DIST/icon.png"
ICONSET="$DIST/AppIcon.iconset"
mkdir -p "$ICONSET"
for s in 16 32 128 256 512; do
  d=$((s * 2))
  sips -z "$s" "$s" "$DIST/icon.png" --out "$ICONSET/icon_${s}x${s}.png" >/dev/null
  sips -z "$d" "$d" "$DIST/icon.png" --out "$ICONSET/icon_${s}x${s}@2x.png" >/dev/null
done
iconutil -c icns "$ICONSET" -o "$APP/Contents/Resources/$APP_NAME.icns"
rm -rf "$ICONSET" "$DIST/icon.png"

echo "==> Tạo Info.plist"
cat > "$APP/Contents/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>$APP_NAME</string>
  <key>CFBundleDisplayName</key><string>$APP_NAME</string>
  <key>CFBundleIdentifier</key><string>$BUNDLE_ID</string>
  <key>CFBundleVersion</key><string>$VERSION</string>
  <key>CFBundleShortVersionString</key><string>$VERSION</string>
  <key>CFBundleExecutable</key><string>$APP_NAME</string>
  <key>CFBundleIconFile</key><string>$APP_NAME</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>LSUIElement</key><true/>
  <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
EOF

echo "==> Ký app"
# Ad-hoc ("-") đủ để chạy trên máy bạn. Để phát hành rộng rãi, thay bằng
# "Developer ID Application: Tên (TEAMID)" và thêm --options runtime.
SIGN_ID="${SIGN_ID:--}"
codesign --force --deep --sign "$SIGN_ID" "$APP"

echo "==> Tạo DMG"
STAGE="$DIST/dmg-stage"
mkdir -p "$STAGE"
cp -R "$APP" "$STAGE/"
ln -s /Applications "$STAGE/Applications"
hdiutil create -volname "$APP_NAME" -srcfolder "$STAGE" -ov -format UDZO "$DMG"
rm -rf "$STAGE"

echo "✅ Xong: $DMG"
