//! Tạo icon app 1024x1024 (PNG): hình vuông bo góc, nền xanh gradient, mũi tên ⇅ trắng.
//! Dùng: cargo run --release --example make_icon -- dist/icon.png

use std::fs::File;
use std::io::BufWriter;

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

fn glyph(u: f32, v: f32) -> bool {
    let up_shaft = (0.28..=0.36).contains(&u) && (0.30..=0.88).contains(&v);
    let up_head = in_tri((u, v), (0.32, 0.10), (0.12, 0.40), (0.52, 0.40));
    let dn_shaft = (0.64..=0.72).contains(&u) && (0.12..=0.70).contains(&v);
    let dn_head = in_tri((u, v), (0.68, 0.90), (0.48, 0.60), (0.88, 0.60));
    up_shaft || up_head || dn_shaft || dn_head
}

/// Hình vuông bo góc, chừa lề như icon macOS.
fn in_rounded_rect(x: f32, y: f32) -> bool {
    let inset = 0.08;
    let radius = 0.19;
    let half = 0.5 - inset;
    let qx = ((x - 0.5).abs() - (half - radius)).max(0.0);
    let qy = ((y - 0.5).abs() - (half - radius)).max(0.0);
    (qx * qx + qy * qy).sqrt() <= radius
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "icon.png".into());
    const N: usize = 1024;
    const SS: usize = 2;

    let top = (0.35_f32, 0.64_f32, 1.00_f32); // xanh sáng
    let bottom = (0.12_f32, 0.37_f32, 0.84_f32); // xanh đậm
    let (gx0, gx1) = (0.25_f32, 0.75_f32); // vùng chứa mũi tên trên canvas

    let mut buf = vec![0u8; N * N * 4];
    for y in 0..N {
        for x in 0..N {
            let (mut sr, mut sg, mut sb, mut sa) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
            for j in 0..SS {
                for i in 0..SS {
                    let fx = (x as f32 + (i as f32 + 0.5) / SS as f32) / N as f32;
                    let fy = (y as f32 + (j as f32 + 0.5) / SS as f32) / N as f32;
                    if !in_rounded_rect(fx, fy) {
                        continue;
                    }
                    let t = ((fy - 0.08) / 0.84).clamp(0.0, 1.0);
                    let (mut r, mut g, mut b) = (
                        lerp(top.0, bottom.0, t),
                        lerp(top.1, bottom.1, t),
                        lerp(top.2, bottom.2, t),
                    );
                    let u = (fx - gx0) / (gx1 - gx0);
                    let v = (fy - gx0) / (gx1 - gx0);
                    if (0.0..=1.0).contains(&u) && (0.0..=1.0).contains(&v) && glyph(u, v) {
                        r = 1.0;
                        g = 1.0;
                        b = 1.0;
                    }
                    sr += r;
                    sg += g;
                    sb += b;
                    sa += 1.0;
                }
            }
            let n = (SS * SS) as f32;
            let idx = (y * N + x) * 4;
            if sa > 0.0 {
                buf[idx] = (sr / sa * 255.0) as u8;
                buf[idx + 1] = (sg / sa * 255.0) as u8;
                buf[idx + 2] = (sb / sa * 255.0) as u8;
                buf[idx + 3] = (sa / n * 255.0) as u8;
            }
        }
    }

    let file = File::create(&out).expect("Không tạo được file PNG");
    let mut enc = png::Encoder::new(BufWriter::new(file), N as u32, N as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()
        .expect("header PNG")
        .write_image_data(&buf)
        .expect("ghi dữ liệu PNG");
    println!("Đã tạo {out}");
}
