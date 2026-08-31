//! TEMPORARY palette search — deleted before commit.
use perceive_color::Color;
use perceive_cvd::{CvdType, Severity, simulate};
use viewer::theme::{Mark, Theme};
use editor_core::appearance::Rgba8;

const KINDS: [Option<CvdType>; 4] = [None, Some(CvdType::Protan), Some(CvdType::Deutan), Some(CvdType::Tritan)];

fn lin8(c: Rgba8) -> [f64; 3] {
    let f = |v: u8| { let c = f64::from(v)/255.0; if c <= 0.04045 { c/12.92 } else { ((c+0.055)/1.055).powf(2.4) } };
    [f(c.r), f(c.g), f(c.b)]
}
fn mixc(body: Rgba8, tint: Rgba8, t: f64) -> [f64; 3] {
    let (b, ti) = (lin8(body), lin8(tint));
    [0,1,2].map(|i| b[i] + (ti[i]-b[i])*t)
}
fn oklab(c: Color) -> [f64;3] { let p = c.to_oklch(); let h = p.h.to_radians(); [p.l, p.c*h.cos(), p.c*h.sin()] }
fn d(a: Color, b: Color) -> f64 { let (x,y)=(oklab(a),oklab(b)); ((x[0]-y[0]).powi(2)+(x[1]-y[1]).powi(2)+(x[2]-y[2]).powi(2)).sqrt() }
fn seen(c: Color, k: Option<CvdType>) -> Color { match k { None => c, Some(k) => simulate(c, k, Severity::FULL) } }

fn worst(body: Rgba8, ambient: f64, marks: &[(&str, Rgba8, f64); 4]) -> (f64, String) {
    let shades = [ambient, ambient + (1.0-ambient)*0.5, 1.0];
    let mut w = (f64::INFINITY, String::new());
    for sh in shades {
        let mut sw: Vec<(&str, Color)> = vec![("body", { let l = lin8(body); Color::new(l[0]*sh, l[1]*sh, l[2]*sh) })];
        for (n, t, s) in marks { let m = mixc(body, *t, *s); sw.push((n, Color::new(m[0]*sh, m[1]*sh, m[2]*sh))); }
        for k in KINDS {
            for i in 0..sw.len() { for j in i+1..sw.len() {
                let dd = d(seen(sw[i].1, k), seen(sw[j].1, k));
                if dd < w.0 { w = (dd, format!("{}/{} kind={:?} shade={sh:.2}", sw[i].0, sw[j].0, k)); }
            }}
        }
    }
    w
}

fn lightness(body: Rgba8, tint: Rgba8, s: f64) -> f64 { let m = mixc(body, tint, s); oklab(Color::new(m[0],m[1],m[2])).0 }

#[test]
fn search() {
    // xorshift, so the search is reproducible without a dep.
    let mut st: u64 = 0x2545F4914F6CDD1D;
    let mut rnd = move || { st ^= st << 13; st ^= st >> 7; st ^= st << 17; (st >> 11) as f64 / (1u64 << 53) as f64 };
    let ambient = 0.42;
    let mut body = Rgba8::opaque(120, 119, 117);
    let mut marks: [(&str, Rgba8, f64); 4] = [
        ("selected", Rgba8::opaque(255, 214, 90), 0.72),
        ("hovered",  Rgba8::opaque(35, 78, 168),  0.66),
        ("probe",    Rgba8::opaque(22, 12, 34),   0.74),
        ("focus",    Rgba8::opaque(214, 224, 238), 0.38),
    ];
    let (mut cw, mut cwhere) = worst(body, ambient, &marks);
    println!("seed {cw:.4} at {cwhere}");
    let (mut bb, mut bm, mut bw) = (body, marks, cw);
    for _ in 0..200_000 {
        let (mut nb, mut nm) = (body, marks);
        if rnd() < 0.15 {
            let i = (rnd()*3.0) as usize;
            let dv = ((rnd()*13.0) as i32) - 6;
            let ch = match i { 0 => &mut nb.r, 1 => &mut nb.g, _ => &mut nb.b };
            *ch = (i32::from(*ch) + dv).clamp(85, 180) as u8;
        } else {
            let li = (rnd()*4.0) as usize;
            if rnd() < 0.35 {
                nm[li].2 = (nm[li].2 + (rnd()-0.5)*0.08).clamp(0.30, 0.75);
            } else {
                let i = (rnd()*3.0) as usize;
                let dv = ((rnd()*19.0) as i32) - 9;
                let ch = match i { 0 => &mut nm[li].1.r, 1 => &mut nm[li].1.g, _ => &mut nm[li].1.b };
                *ch = (i32::from(*ch) + dv).clamp(14, 250) as u8;
            }
        }
        // keep the designed intent: focus stays the quietest mark, and
        // the ladder keeps its order.
        if nm[3].2 > nm[0].2 { continue; }
        let lb = oklab(Color::new(lin8(nb)[0], lin8(nb)[1], lin8(nb)[2])).0;
        let ls = lightness(nb, nm[0].1, nm[0].2);
        let lh = lightness(nb, nm[1].1, nm[1].2);
        let lp = lightness(nb, nm[2].1, nm[2].2);
        let lf = lightness(nb, nm[3].1, nm[3].2);
        if !(lp < lh && lh < lb && lb < lf && lf < ls) { continue; }
        let (nw, nwhere) = worst(nb, ambient, &nm);
        if nw >= cw { body = nb; marks = nm; cw = nw; cwhere = nwhere;
            if nw > bw { bw = nw; bb = nb; bm = nm; } }
    }
    println!("\nBEST {bw:.4}");
    println!("body ({}, {}, {})", bb.r, bb.g, bb.b);
    for (n, t, s) in &bm { println!("  {n:<9} rgb({}, {}, {}) strength {s:.2}  L={:.3}", t.r, t.g, t.b, lightness(bb, *t, *s)); }
    println!("worst at {}", worst(bb, ambient, &bm).1);
    let _ = cwhere;
}
