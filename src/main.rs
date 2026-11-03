//! reverse-scroll (menu bar): đảo chiều cuộn của con lăn chuột trên macOS.
//!
//! Icon ⇅ trên menu bar, bấm để:
//!  - Đảo chiều cuộn của chuột (bật/tắt độc lập)
//!  - Đảo chiều cuộn của trackpad (bật/tắt độc lập)
//!  - Đảo cuộn ngang
//!  - Mở cùng máy khi đăng nhập
//!
//! Cần quyền: Cài đặt hệ thống → Quyền riêng tư & Bảo mật → Trợ năng (Accessibility).
//! Log: ~/Library/Logs/ReverseScroll.log

use core_foundation::runloop::{kCFRunLoopCommonModes, CFRunLoop};
use core_graphics::event::{
    CGEvent, CGEventTap, CGEventTapLocation, CGEventTapOptions, CGEventTapPlacement, CGEventType,
};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tao::event::{Event, StartCause};
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
use tray_icon::menu::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem};
use tray_icon::{Icon, TrayIconBuilder};

// ---- Field của CGEvent cho scroll wheel (CGEventTypes.h) ----
const SCROLL_DELTA_AXIS_1: u32 = 11;
const SCROLL_DELTA_AXIS_2: u32 = 12;
const SCROLL_FIXED_AXIS_1: u32 = 93;
const SCROLL_FIXED_AXIS_2: u32 = 94;
const SCROLL_POINT_AXIS_1: u32 = 96;
const SCROLL_POINT_AXIS_2: u32 = 97;
const SCROLL_IS_CONTINUOUS: u32 = 88; // 1 = trackpad / Magic Mouse

// ---- Trạng thái dùng chung giữa menu và callback ----
static REVERSE_MOUSE: AtomicBool = AtomicBool::new(true);
static REVERSE_TRACKPAD: AtomicBool = AtomicBool::new(false);
static REVERSE_HORIZONTAL: AtomicBool = AtomicBool::new(true);

const LOGIN_LABEL: &str = "com.example.reversescroll";

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
}

fn trusted() -> bool {
    unsafe { AXIsProcessTrusted() }
}

// ------------------------------------------------------------------ log --

fn home() -> PathBuf {
    PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| "/tmp".into()))
}

fn log(msg: &str) {
    let dir = home().join("Library/Logs");
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("ReverseScroll.log"))
    {
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let _ = writeln!(f, "[{secs}] {msg}");
    }
}

// ---------------------------------------------------------------- scroll --

// Ghi log chi tiết cho 30 sự kiện cuộn đầu tiên để chẩn đoán
static DEBUG_LEFT: AtomicUsize = AtomicUsize::new(30);

fn handle_scroll(event: &CGEvent) {
    // ĐỌC hết giá trị gốc trước: các field delta/point/fixed liên kết với nhau,
    // ghi field này có thể làm đổi field kia.
    let continuous = event.get_integer_value_field(SCROLL_IS_CONTINUOUS);
    let d1 = event.get_integer_value_field(SCROLL_DELTA_AXIS_1);
    let d2 = event.get_integer_value_field(SCROLL_DELTA_AXIS_2);
    let f1 = event.get_double_value_field(SCROLL_FIXED_AXIS_1);
    let f2 = event.get_double_value_field(SCROLL_FIXED_AXIS_2);
    let p1 = event.get_integer_value_field(SCROLL_POINT_AXIS_1);
    let p2 = event.get_integer_value_field(SCROLL_POINT_AXIS_2);

    // Chuột và trackpad là hai tuỳ chọn độc lập
    let is_trackpad = continuous != 0;
    let skip = if is_trackpad {
        !REVERSE_TRACKPAD.load(Ordering::Relaxed)
    } else {
        !REVERSE_MOUSE.load(Ordering::Relaxed)
    };

    if DEBUG_LEFT.load(Ordering::Relaxed) > 0 {
        DEBUG_LEFT.fetch_sub(1, Ordering::Relaxed);
        log(&format!(
            "scroll: {} (continuous={continuous}) d1={d1} d2={d2} f1={f1:.2} f2={f2:.2} p1={p1} p2={p2} -> {}",
            if is_trackpad { "trackpad" } else { "chuột" },
            if skip { "BỎ QUA" } else { "ĐẢO" }
        ));
    }

    if skip {
        return;
    }

    // Rồi mới GHI giá trị đã đảo dấu
    event.set_integer_value_field(SCROLL_DELTA_AXIS_1, -d1);
    event.set_double_value_field(SCROLL_FIXED_AXIS_1, -f1);
    event.set_integer_value_field(SCROLL_POINT_AXIS_1, -p1);

    if REVERSE_HORIZONTAL.load(Ordering::Relaxed) {
        event.set_integer_value_field(SCROLL_DELTA_AXIS_2, -d2);
        event.set_double_value_field(SCROLL_FIXED_AXIS_2, -f2);
        event.set_integer_value_field(SCROLL_POINT_AXIS_2, -p2);
    }
}

/// Tạo event tap và gắn vào run loop chính (của tao/NSApplication).
fn install_tap() -> Option<CGEventTap<'static>> {
    let tap = match CGEventTap::new(
        CGEventTapLocation::HID,
        CGEventTapPlacement::HeadInsertEventTap,
        CGEventTapOptions::Default,
        vec![CGEventType::ScrollWheel],
        |_proxy, event_type, event| {
            if matches!(event_type, CGEventType::ScrollWheel) {
                handle_scroll(event);
            }
            Some(event.clone())
        },
    ) {
        Ok(t) => t,
        Err(_) => {
            log("Không tạo được event tap (thiếu quyền Trợ năng / Giám sát đầu vào?)");
            return None;
        }
    };

    let source = match tap.mach_port.create_runloop_source(0) {
        Ok(s) => s,
        Err(_) => {
            log("Không tạo được run loop source");
            return None;
        }
    };
    unsafe {
        CFRunLoop::get_main().add_source(&source, kCFRunLoopCommonModes);
    }
    tap.enable();
    log("Event tap đã được cài, đang hoạt động");
    Some(tap)
}

// ------------------------------------------------------------ menu icon --

fn in_tri(p: (f32, f32), a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> bool {
    let s = |p: (f32, f32), q: (f32, f32), r: (f32, f32)| {
        (p.0 - r.0) * (q.1 - r.1) - (q.0 - r.0) * (p.1 - r.1)
    };
    let d1 = s(p, a, b);
    let d2 = s(p, b, c);
    let d3 = s(p, c, a);
    let neg = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let pos = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(neg && pos)
}

/// Hình ⇅ (mũi tên lên bên trái, mũi tên xuống bên phải), toạ độ chuẩn hoá 0..1.
fn glyph(u: f32, v: f32) -> bool {
    let up_shaft = (0.28..=0.36).contains(&u) && (0.30..=0.88).contains(&v);
    let up_head = in_tri((u, v), (0.32, 0.10), (0.12, 0.40), (0.52, 0.40));
    let dn_shaft = (0.64..=0.72).contains(&u) && (0.12..=0.70).contains(&v);
    let dn_head = in_tri((u, v), (0.68, 0.90), (0.48, 0.60), (0.88, 0.60));
    up_shaft || up_head || dn_shaft || dn_head
}

/// Icon đen trong suốt (template) – macOS tự đổi màu theo giao diện sáng/tối.
fn make_tray_icon() -> Option<Icon> {
    const N: usize = 44;
    const SS: usize = 4;
    let mut rgba = vec![0u8; N * N * 4];
    for y in 0..N {
        for x in 0..N {
            let mut hit = 0;
            for j in 0..SS {
                for i in 0..SS {
                    let u = (x as f32 + (i as f32 + 0.5) / SS as f32) / N as f32;
                    let v = (y as f32 + (j as f32 + 0.5) / SS as f32) / N as f32;
                    if glyph(u, v) {
                        hit += 1;
                    }
                }
            }
            let a = (hit * 255 / (SS * SS)) as u8;
            let idx = (y * N + x) * 4;
            rgba[idx + 3] = a; // RGB = 0 (đen), chỉ alpha thay đổi
        }
    }
    Icon::from_rgba(rgba, N as u32, N as u32).ok()
}

// ---------------------------------------------------------------- config --

fn config_path() -> PathBuf {
    home().join(".config").join("reverse-scroll.conf")
}

fn load_config() {
    let Ok(text) = std::fs::read_to_string(config_path()) else {
        return;
    };
    for line in text.lines() {
        let Some((k, v)) = line.split_once('=') else {
            continue;
        };
        let on = v.trim() == "1";
        match k.trim() {
            "mouse" => REVERSE_MOUSE.store(on, Ordering::Relaxed),
            "trackpad" => REVERSE_TRACKPAD.store(on, Ordering::Relaxed),
            "horizontal" => REVERSE_HORIZONTAL.store(on, Ordering::Relaxed),
            _ => {}
        }
    }
}

fn save_config() {
    let b = |a: &AtomicBool| if a.load(Ordering::Relaxed) { 1 } else { 0 };
    let text = format!(
        "mouse={}\ntrackpad={}\nhorizontal={}\n",
        b(&REVERSE_MOUSE),
        b(&REVERSE_TRACKPAD),
        b(&REVERSE_HORIZONTAL)
    );
    let path = config_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(path, text);
}

// ------------------------------------------------------- launch at login --

fn login_plist_path() -> PathBuf {
    home()
        .join("Library/LaunchAgents")
        .join(format!("{LOGIN_LABEL}.plist"))
}

fn login_enabled() -> bool {
    login_plist_path().exists()
}

fn set_login(on: bool) {
    let path = login_plist_path();
    if !on {
        let _ = std::fs::remove_file(path);
        return;
    }
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let plist = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key><string>{LOGIN_LABEL}</string>
  <key>ProgramArguments</key><array><string>{}</string></array>
  <key>RunAtLoad</key><true/>
</dict>
</plist>
"#,
        exe.display()
    );
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    let _ = std::fs::write(path, plist);
}

// ------------------------------------------------------------------ main --

fn open_accessibility_settings() {
    let _ = std::process::Command::new("open")
        .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
        .status();
}

fn main() {
    log("Khởi động ReverseScroll");
    load_config();

    let mut event_loop = EventLoopBuilder::<MenuEvent>::with_user_event().build();
    event_loop.set_activation_policy(ActivationPolicy::Accessory); // không hiện icon Dock

    // Menu event -> đánh thức event loop
    let proxy = event_loop.create_proxy();
    MenuEvent::set_event_handler(Some(move |e| {
        let _ = proxy.send_event(e);
    }));

    // ---- Menu ----
    let menu = Menu::new();
    let status_item = MenuItem::new("Đang kiểm tra quyền…", false, None);
    let mouse_item = CheckMenuItem::new("Đảo chiều chuột", true, REVERSE_MOUSE.load(Ordering::Relaxed), None);
    let trackpad_item = CheckMenuItem::new("Đảo chiều trackpad", true, REVERSE_TRACKPAD.load(Ordering::Relaxed), None);
    let horiz_item = CheckMenuItem::new("Đảo cả cuộn ngang", true, REVERSE_HORIZONTAL.load(Ordering::Relaxed), None);
    let login_item = CheckMenuItem::new("Mở khi đăng nhập", true, login_enabled(), None);
    let quit_item = MenuItem::new("Thoát", true, None);

    menu.append_items(&[
        &status_item,
        &PredefinedMenuItem::separator(),
        &mouse_item,
        &trackpad_item,
        &horiz_item,
        &PredefinedMenuItem::separator(),
        &login_item,
        &PredefinedMenuItem::separator(),
        &quit_item,
    ])
    .expect("Không tạo được menu");

    let mut tray = None;
    let mut tap: Option<CGEventTap<'static>> = None;
    let mut prompted = false;

    event_loop.run(move |event, _, control_flow| {
        match event {
            // Tạo tray khi event loop đã chạy (khuyến nghị trên macOS)
            Event::NewEvents(StartCause::Init) => {
                let mut builder = TrayIconBuilder::new()
                    .with_menu(Box::new(menu.clone()))
                    .with_tooltip("Reverse Scroll");
                match make_tray_icon() {
                    Some(icon) => builder = builder.with_icon(icon).with_icon_as_template(true),
                    None => builder = builder.with_title("⇅"), // dự phòng
                }
                match builder.build() {
                    Ok(t) => {
                        log("Đã tạo icon menu bar");
                        tray = Some(t);
                    }
                    Err(e) => log(&format!("Lỗi tạo icon menu bar: {e}")),
                }
            }
            Event::UserEvent(ev) => {
                let id = &ev.id;
                if id == quit_item.id() {
                    log("Thoát");
                    *control_flow = ControlFlow::Exit;
                    return;
                } else if id == mouse_item.id() {
                    REVERSE_MOUSE.store(mouse_item.is_checked(), Ordering::Relaxed);
                    save_config();
                } else if id == trackpad_item.id() {
                    REVERSE_TRACKPAD.store(trackpad_item.is_checked(), Ordering::Relaxed);
                    save_config();
                } else if id == horiz_item.id() {
                    REVERSE_HORIZONTAL.store(horiz_item.is_checked(), Ordering::Relaxed);
                    save_config();
                } else if id == login_item.id() {
                    set_login(login_item.is_checked());
                }
            }
            _ => {}
        }

        // Chưa có tap: kiểm tra quyền mỗi giây cho tới khi cài được
        if tap.is_none() {
            if trusted() {
                tap = install_tap();
            } else if !prompted {
                prompted = true;
                log("Chưa có quyền Trợ năng, mở trang cài đặt");
                open_accessibility_settings();
            }
        }

        if tap.is_some() {
            status_item.set_text("✅ Đang hoạt động");
            *control_flow = ControlFlow::Wait;
        } else {
            status_item.set_text("⚠️ Cần quyền Trợ năng (Accessibility)");
            *control_flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_secs(1));
        }

        let _ = &tray; // giữ tray sống suốt vòng đời app
    });
}
